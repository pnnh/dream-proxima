cmake_minimum_required(VERSION 3.0)
project(Foo CXX)

add_executable(foo main.cpp)
install(TARGETS foo DESTINATION .)

include(../../cmake/cargo.cmake)

add_dependencies(foo rs)
target_link_directories(foo PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR})
target_link_libraries(foo foo_rs)

