cmake_minimum_required(VERSION 3.9)
project(foo CXX)

set(CMAKE_OSX_ARCHITECTURES "arm64")

add_subdirectory(packages/foo)