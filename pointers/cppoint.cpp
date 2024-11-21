#include <cassert>
#include <iostream>
using namespace std;

int main() {
    int num = 42;
    int* ptr = &num;
    cout << "Pointer value: " << ptr << endl;

    int x = 10;
	int &r = x;
    ptr = &x;

	assert(r == 10);
	r = 20;

    std::cout << r << std::endl;
    std::cout << ptr << std::endl;

}
// Output::
// Pointer value: 0x7ffd1d3b2040
// 20
// 0x7ffd1d3b2044