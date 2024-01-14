// g++ src.cpp -std=c++23 -Wextra -Wall -fsanitize={address,undefined} -g -o src && ./src
#include <iostream>
#include <fstream>
#include <vector>
#include <ranges>
#include <utility>
#include <queue>

using namespace std;

enum class Tile
{
    Garden,
    Rock
};

using Position = pair<int, int>;

struct Info
{
    bool accessed_by_even = false;
    bool accessed_by_odd = false;
};

int main()
{
    cout << "Let's go...\n";

    ifstream file("testcase.txt");
    vector<vector<Tile>> tiles;
    string line;
    Position start_pos;
    while (getline(file, line))
    {
        int start_col = -1;
        auto row = line | ranges::views::enumerate | ranges::views::transform([&start_col](const auto t)
                                                                              {
            auto &[col, c] = t;
            switch (c) {
                case 'S':
                    start_col = col;
                    [[fallthrough]];
                case '.':
                    return Tile::Garden;
                case '#':
                    return Tile::Rock;
                default:
                    unreachable();
            } });
        tiles.push_back(vector(begin(row), end(row)));
        if (start_col != -1)
        {
            start_pos = {tiles.size() - 1, start_col};
        }
    }

    // cout << start_pos.first << ',' << start_pos.second << endl;

    const int height = tiles.size(), width = tiles[0].size();
    vector<vector<Info>> infos(height, vector<Info>(width));
    // TODO: precompute list of accessible for each grid position
    int steps = 0;
    int current_ways = 1;
    int future_ways = 0;
    queue<Position> que({start_pos});
    while (steps <= 64 && !que.empty())
    {
        current_ways--;
        auto curr = que.front();
        que.pop();

        constexpr array<int, 5> offsets = {1, 0, -1, 0, 1};
        for (int o = 1; o < static_cast<int>(offsets.size()); o++)
        {
            int x = curr.first + offsets[o - 1];
            int y = curr.second + offsets[o];
            if (x < 0 || x >= height || y < 0 || y >= width || tiles[x][y] == Tile::Rock)
            {
                continue;
            }

            // Candidate: {x, y}
            if ((steps + 1) % 2 == 1)
            {
                if (infos[x][y].accessed_by_odd)
                {

                    continue;
                }
                else
                {
                    infos[x][y].accessed_by_odd = true;
                }
            }
            else if ((steps + 1) % 2 == 0)
            {
                if (infos[x][y].accessed_by_even)
                {

                    continue;
                }
                else
                {
                    infos[x][y].accessed_by_even = true;
                }
            }

            que.push({x, y});
            future_ways++;
        }

        if (current_ways == 0)
        {
            current_ways = future_ways;
            future_ways = 0;
            steps++;
        }
    }

    int ans = 0;
    for (const auto &row : infos)
    {
        for (const auto &info : row)
        {
            if (info.accessed_by_even)
            {
                ans++;
            }
        }
    }
    cout << "Ans: " << ans << '\n';
}