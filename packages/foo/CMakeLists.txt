cmake_minimum_required(VERSION 3.0)
project(Foo CXX)

add_executable(foo foo/main.cpp)
install(TARGETS foo DESTINATION .)