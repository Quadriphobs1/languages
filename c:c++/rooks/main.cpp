#include <iostream>
#include <vector>
using namespace std;

typedef vector< vector<int> > Matrix;

void find_rook(Matrix& arr, int cols) {

    for (int i = 0; i < cols; ++i) {
        for (int j = 0; j < cols; ++j) {
            if (arr[i][j] == -1) {
                break;
            } else {
                // check from first row to last row with the current column
                // if it is possible to position a rook
                for (int k = 0; k < cols; ++k) {
                    if (arr[j][k] == -1) {
                        break;
                    } else {
                        cout << k << " " << j << " " << arr[j][k] << "\n";
                        arr[i][j] = -1;
                        continue;
                    }
                }
            }
        }
    }

}

int main() {
    int numberOfColsXRows = 3;
    int currentRows = 0;
    int currentColumns = 0;
    Matrix arr( numberOfColsXRows,vector<int>(numberOfColsXRows,0));

    int count = 1;
    // fill the array with 0 to represent non filled space
    for (int i = 0; i < numberOfColsXRows; ++i) {
        for (int j = 0; j < numberOfColsXRows; ++j) {
            arr[i][j] = count;
            count++;
        }
    }

    arr[currentRows][currentColumns] = -1;

    find_rook(arr, numberOfColsXRows);

    cout << "\n";
    for (int i = 0; i < numberOfColsXRows; ++i) {
        for (int j = 0; j < numberOfColsXRows; ++j) {
            cout << "| " << arr[i][j] << " ";
        }
        cout << "\n";
    }
    return 0;
}