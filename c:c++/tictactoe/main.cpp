#include "Board.h"
#include <iostream>
#include <string>


using namespace std;


void getUsernames(string &, string &);
void printInstructions();
void printUserPrompt(string name, char letter);
void printGameWinner(Board board, string nameX, string nameO);
void askUser(Board &gameBoard, string user, char letter, char &);
int main() {
    Board gameBoard;
    string userX, userO;

    char gameWinner;

    int turns = 0;

    getUsernames(userX, userO);
    printInstructions();
    while (turns < 8) {
        askUser(gameBoard, userX, 'x', gameWinner);

        if (gameWinner != 'z') {
            gameBoard.printBoard();
            printGameWinner(gameBoard, userX, userO);
            break;
        }
        askUser(gameBoard, userO, 'o', gameWinner);

        if (gameWinner != 'z') {
            gameBoard.printBoard();
            printGameWinner(gameBoard, userX, userO);
            break;
        }
        turns++;
    }

    if (turns >= 8) cout << "Oops!!! It is a draw \n\n";
    return 0;
}

void getUsernames(string &userX, string &userO) {
    cout << "Name of user to be X: ";
    cin >> userX;
    cout << "Name of user to be O: ";
    cin >> userO;
}

void printInstructions() {
    cout << "This is a 4X4 game which gives makes it 16 cells space available \n";
    cout << "To fill a cell you would input integer between 0 - 15 \n";
    cout << "\n\n\n";
}

void printUserPrompt(string name, char letter) {
    cout << name << " where would you place " << letter << "?: ";
    cout << "\n\n";
}

void printGameWinner(Board board, string nameX, string nameO){
    char winner;
    winner = board.determineWinner();
    if (winner == 'x') cout << "Congrats " << nameX << " you won! \n\n";
    if (winner == 'o') cout << "Congrats " << nameO << " you won! \n\n";
}

void askUser(Board &gameBoard, string user, char letter, char &gameWinner) {
    gameBoard.printBoard();
    printUserPrompt(user, letter);
    gameBoard.checkResponse(letter);
    gameWinner = gameBoard.determineWinner();
}
