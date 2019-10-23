//
// Created by Abiodun Quadri Adekunle on 19/10/2019.
//

#include "WeightedQuickUnion.h"

WeightedQuickUnion::WeightedQuickUnion(int N) {
    for (int i = 0; i < N; i++) {
        id[i] = i;
        sz[i] = i;
    };
}

void WeightedQuickUnion::join(int x, int y) {
    int i = root(x);
    int j = root(y);
    if (i == j) return;
    if (sz[i] < sz[j]) {
        id[i] = j;
        sz[j] += sz[i];
    } else{
        id[j] = i;
        sz[i] += sz[j];
    }
}

bool WeightedQuickUnion::connected(int x, int y) {
    return root(x) == root(y);
}

int WeightedQuickUnion::root(int i) {
    while (i != id[i]) i = id[i];
    return i;
}