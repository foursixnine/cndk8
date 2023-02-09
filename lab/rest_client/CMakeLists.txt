# CMakeLists.txt

cmake_minimum_required(VERSION 3.7)
## I want to debug
# set (CMAKE_CXX_FLAGS "-v")

include(CMakePrintHelpers)
find_package(Corrosion REQUIRED)

project(rest-client)
# build dependencies
add_subdirectory(client)
add_subdirectory(gui)
enable_testing()
