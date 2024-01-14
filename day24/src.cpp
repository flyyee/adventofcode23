// g++ -std=c++23 -g -pedantic -Wall -Wextra -fsanitize={address,undefined} src.cpp -I/home/kali/projects/aoc/rust23/day24/gmp-6.3.0 -L/home/kali/projects/aoc/rust23/day24/gmp-6.3.0/.libs -lgmpxx -lgmp -o src && LD_LIBRARY_PATH=/home/kali/projects/aoc/rust23/day24/gmp-6.3.0/.libs ./src

#include <iostream>
#include <vector>
#include <fstream>
#include <string>
#include <sstream>
#include <string_view>
#include <gmpxx.h>
#include <array>
#include <expected>
#include <format>
#include <functional>

template <typename T>
class vec3
{
public:
    T x, y, z;

    vec3() = default;
    vec3(const vec3 &other) : x(std::move(other.x)), y(std::move(other.y)), z(std::move(other.z)) {}

    vec3(T x, T y, T z) : x(x), y(y), z(z) {}

    vec3 operator+(const vec3 &other) const
    {
        return vec3(this->x + other.x, this->y + other.y, this->z + other.z);
    }

    vec3 operator-(const vec3 &other) const
    {
        return vec3(this->x - other.x, this->y - other.y, this->z - other.z);
    }

    template <typename U>
    vec3 operator*(const U &factor) const
    {
        return vec3(this->x * factor, this->y * factor, this->z * factor);
    }

    friend std::ostream &operator<<(std::ostream &os, const vec3 &v)
    {
        os << v.x << ", " << v.y << ", " << v.z;
        return os;
    }
};

class Hailstone
{
public:
    using T = mpq_class;

private:
    vec3<T> position;
    vec3<T> velocity;

public:
    Hailstone(vec3<T> position, vec3<T> velocity) : position(position), velocity(velocity) {}
    enum ParseError
    {
        GENERAL_FAILURE
    };
    static std::expected<Hailstone, ParseError> parse(std::string_view line)
    {
        const std::string separator = " @ ";
        auto separator_pos = line.find(separator);
        auto sv_position = line.substr(0, separator_pos);
        auto sv_velocity = line.substr(separator_pos + separator.length());
        auto string_to_vec3 = [](std::string_view sv, std::string_view delimeter) -> std::expected<vec3<T>, ParseError>
        {
            std::array<T, 3> values;
            size_t prev_pos = 0, pos = 0;
            for (int i = 0; i < 2; i++)
            {
                if ((pos = sv.find(delimeter, prev_pos)) == std::string_view::npos)
                {
                    return std::unexpected(ParseError::GENERAL_FAILURE);
                }
                values[i] = std::string(sv.substr(prev_pos, pos - prev_pos)); // one day...
                prev_pos = pos + delimeter.length();
            }
            values[2] = std::string(sv.substr(prev_pos, pos - prev_pos));
            return vec3<T>(values[0], values[1], values[2]);
        };

        auto position = string_to_vec3(sv_position, ", ");
        if (!position.has_value())
        {
            return std::unexpected(position.error());
        }
        auto velocity = string_to_vec3(sv_velocity, ", ");
        if (!velocity.has_value())
        {
            return std::unexpected(velocity.error());
        }

        return Hailstone(std::move(position.value()), std::move(velocity.value()));
    }

    static std::optional<vec3<T>> crosses_xy(const Hailstone &h1, const Hailstone &h2, bool &infinite_solutions)
    {
        infinite_solutions = false;

        // Essentially, intersection of two lines. Just need to check if intersection happens post both starts
        // Intersection: h1.p + i * h1.v == h2.p + j * h2.v
        mpq_class den = h1.velocity.y * h2.velocity.x - h1.velocity.x * h2.velocity.y;
        if (den == mpq_class{0})
        {
            // TODO: Parallel, just need to check if they are colinear
            // Check gradient between the two points and see if they equal the gradient (v)
            if (h1.velocity.x * (h2.position.y - h1.position.y) == h1.velocity.y * (h2.position.x - h1.position.x))
            {
                // TODO: return
                infinite_solutions = true;
                return vec3<T>{};
            }
            else
            {
                return std::nullopt;
            }
        }
        mpq_class j = (h1.velocity.x * h2.position.y - h1.velocity.x * h1.position.y - h1.velocity.y * h2.position.x + h1.velocity.y * h1.position.x) / den;
        if (sgn(j) == -1)
        {
            // Intersection at negative j
            return std::nullopt;
        }

        // TODO: if h1vy is 0, it could mean the h1py is already correct. Instead check h1vx.
        // If h1vx is also 0, check that h1p == h2p, else use the other formula
        mpq_class i = (h2.position.y + j * h2.velocity.y - h1.position.y) / h1.velocity.y;
        if (sgn(i) == -1)
        {
            // Intersection at negative i
            return std::nullopt;
        }

        return h1.position + h1.velocity * i;
    }

    const vec3<T> &get_position() const
    {
        return this->position;
    }
    const vec3<T> &get_velocity() const
    {
        return this->velocity;
    }
};

long long project_hailstones(std::vector<Hailstone> &hailstones, std::function<bool(const vec3<Hailstone::T> &)> within_target_area)
{
    long long crosses = 0ll;
    const int n = hailstones.size();
    for (int i = 0; i < n; i++)
    {
        for (int j = i + 1; j < n; j++)
        {
            bool infinite_solutions = false;
            auto result = Hailstone::crosses_xy(hailstones[i], hailstones[j], infinite_solutions);
            if (!result)
            {
                continue;
            }
            // Crosses
            if (infinite_solutions)
            {
                // Infinite body of solutions, so necessarily one meets our criteria
                crosses++;
                continue;
            }
            // Check if crossing happens within target area
            if (within_target_area(result.value()))
            {
                crosses++;
            }
        }
    }

    return crosses;
}

int main()
{
    std::cout << "Parsing file for hailstones\n";
    std::vector<Hailstone> hailstones;
    auto parse_file_to_hailstones = [](const std::string &file_name, std::vector<Hailstone> &hailstones)
    {
        std::ifstream file(file_name);
        std::string line;
        while (std::getline(file, line))
        {
            auto result = Hailstone::parse(line);
            if (!result.has_value())
            {
                return true;
            }
            hailstones.push_back(result.value());
        }
        return false;
    };
    if (parse_file_to_hailstones("testcase.txt", hailstones))
    {
        std::cout << "Parse failure\n";
        return 1;
    }

    // std::cout << "Debug: Printing all hailstones\n";
    // for (const auto &h : hailstones)
    // {
    //     std::cout << h.get_position() << '\n'
    //               << h.get_velocity() << '\n';
    // }

    std::cout << "Projecting hailstones...\n";
    std::cout << std::format("Ans = {}\n", project_hailstones(hailstones, [](auto cross)
                                                              //   { return cross.x >= 7 && cross.x <= 27 && cross.y >= 7 && cross.y <= 27; }));
                                                              { return cross.x >= Hailstone::T{200000000000000} && cross.x <= Hailstone::T{400000000000000} && cross.y >= Hailstone::T{200000000000000} && cross.y <= Hailstone::T{400000000000000}; }));
    return 0;
}