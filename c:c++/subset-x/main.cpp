#include <iostream>

using namespace std;

/*
 * There are 90 cards with all two-digit numbers on them:
 * 10,11,12,...,19,20,21,...,90,91,...,99.10,11,12,...,19,20,21,...,90,91,...,99.
 * A player takes some of these cards. For each card taken she gets $1. However,
 * if the player takes two cards that add up to 100 (say, 23 and 77),
 * at the same time, she loses all the money. How much could she get?
 * (In mathematical language: what is the maximum number of elements in a subset of
 * {11,12,...,99}{11,12,...,99} that does not contain any two numbers x and y with x+y=100?)
*/

int subsetXfrom100(int nums[], int sum, int len) {
    int res = 0;
    int lastIndex = len;
    for (int i = 0; i < len; ++i) {
        for (int j = lastIndex; j > 0; --j) {
            if (nums[i] + nums[j] > sum) {
                nums[j] = -1;
                lastIndex = j;
                continue;
            }
        }
    }

    for (int i = 0; i < len; ++i) {
        if (nums[i] != -1) {
            ++res;
        }
    }
    return res;
}

int main() {
    int nums[90];

    int len = sizeof(nums)/ sizeof(*nums);
    int sum = 100;

    for (int i = 0; i < 91; i++) {
        nums[i] = i + 10;
    }

    // This implementation is currently incorrect
    int res = subsetXfrom100(nums, sum, len);
    cout << res << "\n"; // 50
    return 0;
}
