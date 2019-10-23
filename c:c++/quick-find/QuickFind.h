//
// Created by Abiodun Quadri Adekunle on 18/10/2019.
//

#ifndef QUICK_FIND_QUICKFIND_H
#define QUICK_FIND_QUICKFIND_H


class QuickFind {
    int id[];
public:
    QuickFind(int N);
    void join(int x, int y);
    bool connected(int x, int y);

    void display();
};


#endif //QUICK_FIND_QUICKFIND_H
