#include <iostream>
#include <optional>
using namespace std;

struct B;

struct A
{
    int x;
    template <typename T>
    A operator*(const T &other)
    {
        return A{this->x * other};
    }
};

struct B
{
    int x;
    B operator*(const B &other)
    {
        return B{this->x * other.x};
    }
};

int main()
{
    optional<int> o = 3;
    if (o) {
        cout << "good";
    } else {
        cout << "bad";
    }
}