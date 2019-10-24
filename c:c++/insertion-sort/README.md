# Insertion Sort C++

C++ function to perform insertion sort

```c++
int main() {
    int arr[] = {5,2,4,6,1,3};

    int len = sizeof(arr)/sizeof(arr[0]);
    insertionSort(arr, len);
    printArray(arr, len); // {1, 2, 3, 4, 5, 6};
    return 0;
}
```
