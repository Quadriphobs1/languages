#include <iostream>
#include "QuickUnion.h"
#include "WeightedQuickUnion.h"

int main() {
    QuickUnion qu = QuickUnion(10);

    qu.join(4, 3);
    qu.join(3,8);
    qu.join(6, 5);
    qu.join(9, 4);
    qu.join(2, 1);
    std::cout << qu.connected(8, 9) << "\n";
    std::cout << qu.connected(5, 4) << "\n";
    qu.join(5, 0);
    qu.join(7, 2);
    qu.join(6, 1);
    qu.join(7, 3);
    std::cout << qu.connected(5, 4) << "\n";

    WeightedQuickUnion wqu = WeightedQuickUnion(10);
    wqu.join(4, 3);
    wqu.join(3,8);
    wqu.join(6, 5);
    wqu.join(9, 4);
    wqu.join(2, 1);
    std::cout << wqu.connected(8, 9) << "\n";
    std::cout << wqu.connected(5, 4) << "\n";
    wqu.join(5, 0);
    wqu.join(7, 2);
    wqu.join(6, 1);
    wqu.join(7, 3);
    std::cout << wqu.connected(5, 4) << "\n";
    return 0;
}