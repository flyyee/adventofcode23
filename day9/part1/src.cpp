#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <stack>
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
        int n = sequence.size();
        do
        {
            done = true;
            for (int i = 0; i < n - 1; i++)
            {
                sequence[i] = sequence[i + 1] - sequence[i];
                if (sequence[i] != 0)
                {
                    done = false;
                }
            }
            n -= 1;
        } while (!done);

        int next = accumulate(begin(sequence) + n, end(sequence), 0);
        // std::cout << next << '\n';

        ans += next;
    }

    std::cout << ans << '\n';
}