cmake_minimum_required(VERSION 3.15)
project(tictactoe)

set(CMAKE_CXX_STANDARD 17)

add_executable(tictactoe main.cpp Board.cpp Board.h)