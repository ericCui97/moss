cmake_minimum_required(VERSION 3.0)
project(cmoss)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

add_subdirectory(cmoss)
include_directories(cmoss)