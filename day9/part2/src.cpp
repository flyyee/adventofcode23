#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <numeric>

using namespace std;

int main()
{
    int ans = 0;

    fstream file("testcase.txt");
    string line;
    while (getline(file, line))
    {
        istringstream iss(line);
        int i;
        vector<int> sequence;
        while (iss >> i)
        {
            sequence.push_back(i);
        }

        // process
        bool done = false;
        int running = sequence[0];
        int n = 0;
        do
        {
            done = true;
            for (int i = sequence.size() - 1; i > n; i--)
            {
                sequence[i] = sequence[i] - sequence[i - 1];
                if (sequence[i] != 0)
                {
                    done = false;
                }
            }
            n += 1;
        } while (!done);

        for (auto x : sequence)
        {
            // cout << x << ' ';
        }
        // cout << '\n';
        int next;
        if (n % 2 == 0)
        {
            next = sequence[0] - accumulate(begin(sequence) + 1, begin(sequence) + n, 0, [](int sum, int curr)
                                            { return curr - sum; });
        }
        else
        {
            next = sequence[0] + accumulate(begin(sequence) + 1, begin(sequence) + n, 0, [](int sum, int curr)
                                            { return curr - sum; });
        }

        // std::cout << next << '\n';
        ans += next;

        // exit(0);
    }

    std::cout << ans << '\n';
}