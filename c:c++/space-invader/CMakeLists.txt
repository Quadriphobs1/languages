cmake_minimum_required(VERSION 3.15)
project(space_invader)

set(CMAKE_CXX_STANDARD 17)

add_executable(space_invader main.cpp)
include_directories(${PROJECT_SOURCE_DIR}/GLFW/include)
include_directories(${PROJECT_SOURCE_DIR}/Glew/include)
link_directories(${PROJECT_SOURCE_DIR}/GLFW/lib-macos)
link_directories(${PROJECT_SOURCE_DIR}/Glew/lib)
add_executable(openGL main.cpp)
target_link_libraries(openGL glew glfw3 gdi32 opengl32)