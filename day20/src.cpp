// g++ src.cpp -std=c++20 -Wextra -Wall -fsanitize={address,undefined} -g -O3 -o src && ./src
#include <iostream>
#include <string>
#include <fstream>
#include <string_view>
#include <unordered_map>
#include <vector>
#include <deque>
#include <utility>
#include <ranges>
#include <memory>
#include <functional>

using namespace std;

enum class NodeVariant
{
    Broadcaster,
    Flipflop,
    Conjunction
};

enum PulseType
{
    Low,
    High
};

class Node;

struct Pulse
{
    Node *recipient;
    PulseType type;
    Node *origin;
};

class Node
{
    vector<Node *> children;

protected:
    void send_pulse_to_children(const PulseType type, deque<Pulse> &deq)
    {
        for (auto &child : this->children)
        {
            deq.push_back({child, type, this});
        }
    }

    virtual void add_parent(const Node *parent) = 0;

public:
    void set_children(const vector<Node *> &children)
    {
        this->children = std::move(children);
        for (auto &child : this->children)
        {
            child->add_parent(this);
        }
    };

    virtual void receive_pulse(const Node *origin, const PulseType type, deque<Pulse> &deq) = 0;
    virtual ~Node() = default;
};

class Untyped : public Node
{
    void add_parent(const Node *parent)
    {
        (void)parent;
    }
    void receive_pulse(const Node *origin, const PulseType type, deque<Pulse> &deq)
    {
        (void)origin;
        (void)type;
        (void)deq;
    }
};

class Broadcaster : public Node
{
    void add_parent(const Node *parent)
    {
        (void)parent;
    }
    void receive_pulse(const Node *origin, const PulseType type, deque<Pulse> &deq)
    {
        (void)origin;
        send_pulse_to_children(type, deq);
    }
};

class Flipflop : public Node
{
    bool off_state = true;
    void add_parent(const Node *parent)
    {
        // Don't care
        (void)parent;
    }

    void
    receive_pulse(const Node *origin, const PulseType type, deque<Pulse> &deq)
    {
        (void)origin;
        if (type == PulseType::Low)
        {
            send_pulse_to_children((this->off_state ? PulseType::High : PulseType::Low), deq);
            this->off_state = !this->off_state;
        }
    }
};

class Conjunction : public Node
{
    vector<bool> children_low_state;
    unordered_map<const Node *, PulseType> parents_pulse;
    void add_parent(const Node *parent)
    {
        parents_pulse[parent] = PulseType::Low;
    }

    void receive_pulse(const Node *origin, const PulseType type, deque<Pulse> &deq)
    {
        this->parents_pulse[origin] = type;

        const auto parents_all_high = [&]()
        {
            for (auto &[parent, pulse_type] : this->parents_pulse)
            {
                if (pulse_type == PulseType::Low)
                {
                    return false;
                }
            }
            return true;
        };

        send_pulse_to_children((parents_all_high() ? PulseType::Low : PulseType::High), deq);
    }
};

using Graph = unordered_map<string, unique_ptr<Node>>;
Graph parse(const string &filename)
{
    unordered_map<string, unique_ptr<Node>> graph;

    ifstream file(filename);
    const string separator = " -> ";

    string line;
    while (getline(file, line))
    {
        string_view line_sv = line;
        const auto separator_pos = line_sv.find(separator);
        auto name = line_sv.substr(0, separator_pos);

        unique_ptr<Node> node;

        if (name == "broadcaster")
        {
            node = make_unique<Broadcaster>();
        }
        else
        {
            if (name[0] == '%')
            {
                node = make_unique<Flipflop>();
            }
            else
            {
                node = make_unique<Conjunction>();
            }
            name = name.substr(1);
        }

        graph[string(name)] = move(node);
    }

    file.clear();
    file.seekg(0);

    while (getline(file, line))
    {
        string_view line_sv = line;
        const auto separator_pos = line_sv.find(separator);
        auto name = line_sv.substr(0, separator_pos);
        const auto destinations = line_sv.substr(separator_pos + separator.length());

        if (name != "broadcaster")
        {
            name = name.substr(1);
        }

        auto children_range = destinations | ranges::views::split(',') | ranges::views::transform([&graph](auto &&destination)
                                                                                                  {
            auto c = destination | ranges::views::common;
            auto name = string(begin(c), end(c));
            if (name[0] == ' ') {
                name = name.substr(1);
            }
            graph.try_emplace(name, make_unique<Untyped>());
            return graph[name].get(); });

        graph[string(name)]->set_children(vector(begin(children_range), end(children_range)));
    }

    return graph;
}

pair<long, long> PushButton(
    Graph &graph, function<bool(Node *, Node *, PulseType)> hook = [](Node *r, Node *o, PulseType p)
                  { return false; })
{
    deque<Pulse> deq;
    graph["broadcaster"]->receive_pulse(nullptr, PulseType::Low, deq);
    long low_pulse_count = 1, high_pulse_count = 0;
    while (!deq.empty())
    {
        auto pulse = deq.front();
        deq.pop_front();
        pulse.recipient->receive_pulse(pulse.origin, pulse.type, deq);
        if (pulse.type == PulseType::Low)
        {

            low_pulse_count++;
        }
        else
        {
            high_pulse_count++;
        }
        if (hook(pulse.recipient, pulse.origin, pulse.type))
        {
            // break;
        }
    }

    return {low_pulse_count, high_pulse_count};
}

void part1()
{
    cout << "Parsing...\n";
    auto graph = parse("testcase.txt");
    cout << "Parsing complete\n";

    long lows = 0, highs = 0;
    for (int i = 0; i < 1000; i++)
    {
        auto [l, h] = PushButton(graph);
        lows += l;
        highs += h;
    }

    cout << lows << ',' << highs << endl;
    cout << "ans: " << lows * highs << endl;
}

void part2()
{
    cout << "Parsing...\n";
    auto graph = parse("testcase.txt");
    cout << "Parsing complete\n";

    long presses = 0;
    auto rx_node = graph["rx"].get();
    bool done = false;
    while (!done)
    {
        presses++;
        PushButton(graph, [&](Node *recipient, Node *origin, PulseType type)
                   {
            if (recipient == rx_node && type == PulseType::Low) {
                done = true;
                return true;
            }
            return false; });
    }

    cout << "ans: " << presses << endl;
}

void part2_debug()
{
    cout << "Parsing...\n";
    auto graph = parse("testcase.txt");
    cout << "Parsing complete\n";

    long presses = 0;
    auto ft_node = graph["ft"].get();
    auto jz_node = graph["jz"].get();
    auto sv_node = graph["sv"].get();
    auto ng_node = graph["ng"].get();
    auto rx_node = graph["rx"].get();

    int ft_count = 0, jz_count = 0, sv_count = 0, ng_count = 0;

    bool done = false;
    while (!done)
    {
        presses++;
        PushButton(graph, [&](Node *recipient, Node *origin, PulseType type)
                   {
            if (origin == ft_node && type == PulseType::High) {
                ft_count++;
            } else if (origin == jz_node && type == PulseType::High) {
                jz_count++;
            } else if (origin == sv_node && type == PulseType::High) {
                sv_count++;
            } else if (origin == ng_node && type == PulseType::High) {
               ng_count++;
            }
            return false; });

        if (ft_count || jz_count || sv_count || ng_count)
        {
            cout << presses << ":\n";
            cout << ft_count << ',' << jz_count << ',' << sv_count << ',' << ng_count << endl;
        }

        ft_count = 0, jz_count = 0, sv_count = 0, ng_count = 0;
    }

    cout << "ans: " << presses << endl;
}

int main()
{
    part2_debug();
}