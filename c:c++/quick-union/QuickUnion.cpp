//
// Created by Abiodun Quadri Adekunle on 19/10/2019.
//

#include "QuickUnion.h"
QuickUnion::QuickUnion(int N) {
    for (int i = 0; i < N; i++) id[i] = i;
}

int QuickUnion::root(int i) {
    while (i != id[i]) i = id[i];
    return i;
}

bool QuickUnion::connected(int x, int y) {
    return root(x) == root(y);
}

void QuickUnion::join(int x, int y) {
    int i = root(x);
    int j = root(y);

    id[i] = j;
}