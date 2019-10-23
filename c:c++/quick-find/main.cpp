#include <iostream>
#include "QuickFind.h"
using namespace std;
int main() {
    QuickFind sample = QuickFind(10);

    sample.join(4,3);
    sample.join(5, 6);
    sample.join(1, 0);
    sample.join(5, 0);

    return 0;
}