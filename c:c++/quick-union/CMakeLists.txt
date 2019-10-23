cmake_minimum_required(VERSION 3.15)
project(quick_union)

set(CMAKE_CXX_STANDARD 17)

add_executable(quick_union main.cpp QuickUnion.cpp QuickUnion.h WeightedQuickUnion.cpp WeightedQuickUnion.h)