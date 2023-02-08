# CMakeLists.txt

cmake_minimum_required(VERSION 3.7)
## I want to debug
set (CMAKE_CXX_FLAGS "-v")

project(rest-client)

add_subdirectory(client)
add_subdirectory(gui)
enable_testing()
