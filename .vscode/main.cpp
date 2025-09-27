
#include <iostream>

int fib(int n)
{
    if (n <= 1)
        return n;
    return fib(n - 1) + fib(n - 2);
}
int main() {
    std::cout << "Enter a positive integer that you want to find the Fibonacci of: ";
    int n;
    std::cin >> n;
    std::cout << "Fibonacci of " << n << " is " << fib(n) << std::endl;
    return 0;
}