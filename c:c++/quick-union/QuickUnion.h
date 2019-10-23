//
// Created by Abiodun Quadri Adekunle on 19/10/2019.
//

#ifndef QUICK_UNION_QUICKUNION_H
#define QUICK_UNION_QUICKUNION_H



class QuickUnion {
    int id[];
public:
    QuickUnion(int N);

    int root(int i);

    bool connected(int x, int y);

    void join(int x, int y);
};


#endif //QUICK_UNION_QUICKUNION_H
