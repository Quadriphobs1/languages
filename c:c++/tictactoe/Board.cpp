//
// Created by Abiodun Quadri Adekunle on 17/10/2019.
//
#include <iostream>
#include "Board.h"

using namespace std;

char defaultChar = '_';

Board::Board() {
    for (int i = 0; i < 16; i++) positions[i] = defaultChar;
}

void Board::printBoard() {
    for (int i = 0; i < 16; i++) {
        cout << "|" << getPositions()[i];
        if (i%4 == 3) {
            cout << "|\n";
        }
    }
    cout << "\n\n\n";
}

char* Board::getPositions() {
    return positions;
}

int Board::getUserResponse() {
    int position = -1;
    cout << "Enter an integer between 0 - 15 \n";
    cin >> position;

    while (position > 15 or position < 0 or !cin) {
        cin.clear();
        cout << "That is not a valid position \n";
        cin.clear();
        cin >> position;
    }
    return position;
}

void Board::checkResponse(char input) {
    int position;
    int userInput;

    do {
        position = getUserResponse();
        userInput = setPosition(position, input);
        if (userInput == -1) cout << "That position is taken \n";
    } while(userInput == -1);
}


int Board::setPosition(int gridNumber, char user) {
    bool positionEmpty = positions[gridNumber] == defaultChar;
    if (positionEmpty) {
        positions[gridNumber] = user;
        return 0;
    } else {
        return -1;
    }
}

char Board::determineWinner() {
    char winner = 'z';
    winner = checkRows();
    if (winner == 'z') winner = checkColumns();
    if (winner == 'z')  winner = checkDiagonals();

    return winner;
}

char Board::checkDiagonals() {
    char winner = 'z';
    int fourInRowX = 0;
    int fourInRowO = 0;

    for (int i = 0; i < 16; i+=5) {
        if (positions[i] == 'x') fourInRowX++;
        if (positions[i] == 'o')  fourInRowO++;
    }

    if (fourInRowO != 4 and fourInRowX != 4) {
        fourInRowX = 0;
        fourInRowO = 0;
        for(int i = 3; i < 15; i+=3) {
            if (positions[i] == 'x')  fourInRowO++;
            if (positions[i] == 'o') fourInRowO++;
        }
    }
    if(fourInRowX== 4) {
        winner = 'x';
        return winner;
    }
    if (fourInRowO == 4) {
        winner = 'o';
        return winner;
    }
    return winner;
}

char Board::checkColumns(){
    char winner = 'z';
    int fourInRowX = 0;
    int fourInRowO = 0;
    for (int c = 0; c < 4; c++) {
        for (int i = 0; i < 16; i+=4) {
            if (positions[i + c] == 'x') fourInRowX++;
            if (positions[i + c] == 'o') fourInRowO++;
        }

        if(fourInRowX== 4) {
            winner = 'x';
            return winner;
        }
        if (fourInRowO == 4) {
            winner = 'o';
            return winner;
        }
    }

    return winner;
}

char Board::checkRows() {
    char winner = 'z';
    int fourInRowX = 0;
    int fourInRowO = 0;
    for (int r = 0; r < 16; r+=4) {
        for (int i = 0; i < 4; i++) {
            if (positions[i + r] == 'x') fourInRowX++;
            if (positions[i + r] == 'o') fourInRowO++;
        }
        if(fourInRowX== 4) {
            winner = 'x';
            return winner;
        }
        if (fourInRowO == 4) {
            winner = 'o';
            return winner;
        }
    }
    return winner;
}