//
// Created by Abiodun Quadri Adekunle on 17/10/2019.
//

#ifndef TICTACTOE_BOARD_H
#define TICTACTOE_BOARD_H

class Board {
    char positions[16];
public:
    Board();
    void printBoard();
    char* getPositions();
    void checkResponse(char input);
    static int getUserResponse();
    int setPosition(int gridNumber, char user);
    char determineWinner();
    char checkDiagonals();
    char checkRows();
    char checkColumns();
};


#endif //TICTACTOE_BOARD_H
