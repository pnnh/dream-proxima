cmake_minimum_required(VERSION 3.9)
project(LibFoo CXX)

add_library(foo++ STATIC foo.cpp)

install(TARGETS foo++ DESTINATION .)