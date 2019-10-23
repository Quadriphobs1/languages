//
// Created by Abiodun Quadri Adekunle on 19/10/2019.
//

#ifndef QUICK_UNION_WEIGHTEDQUICKUNION_H
#define QUICK_UNION_WEIGHTEDQUICKUNION_H


class WeightedQuickUnion {
public:
    WeightedQuickUnion(int N);
    void join(int x, int y);
    int root(int x);
    bool connected(int x, int y);

private:
    int *sz[];
    int *id[];
};

#endif //QUICK_UNION_WEIGHTEDQUICKUNION_H
