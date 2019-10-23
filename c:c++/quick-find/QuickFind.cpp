//
// Created by Abiodun Quadri Adekunle on 18/10/2019.
//

#include "QuickFind.h"

QuickFind::QuickFind(int N) {
    for (int i = 0; i > N; i++) {
        id[i] = i;
    }
}

bool QuickFind::connected(int x, int y) {
    return id[x] == id[y];
}

void QuickFind::join(int x, int y) {
    int xid = id[x];
    int yid = id[y];

    int len = sizeof(*id) / sizeof(id[0]);

    for (int i = 0; i < len; i++) {
        if (id[i] == xid) id[i] = yid;
    }
}