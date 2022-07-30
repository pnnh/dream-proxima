#cmake_minimum_required(VERSION 3.0)
#project(Foo CXX)
#
#
#set(CMAKE_CXX_STANDARD 20)
#set(CMAKE_VERBOSE_MAKEFILE ON)
#set(CMAKE_INCLUDE_CURRENT_DIR ON)
#
#if (APPLE)
#    message("CMAKE_OSX_SYSROOT: ${CMAKE_OSX_SYSROOT}")
#    set(CMAKE_OSX_SYSROOT "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk")
#endif ()
#
#
#set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake;${CMAKE_MODULE_PATH}")
#include(spdlog)
#
#find_package(PkgConfig REQUIRED)
#
#find_package(Boost COMPONENTS system thread REQUIRED)
#if (Boost_FOUND)
#    message(STATUS "Found Boost: ${FOLLY_LIBRARIES}")
#    INCLUDE_DIRECTORIES(${Boost_INCLUDE_DIRS})
#    LINK_LIBRARIES(${Boost_LIBRARIES})
#endif (Boost_FOUND)
#
#find_package(folly REQUIRED)
#if (folly_FOUND)
#    message(STATUS "Found folly: ${FOLLY_LIBRARIES}")
#    INCLUDE_DIRECTORIES(${folly_INCLUDE_DIRS})
#    LINK_LIBRARIES(${FOLLY_LIBRARIES})
#endif (folly_FOUND)
#
#add_executable(sfx-server main.cpp)
#
#target_link_libraries(sfx-server PRIVATE spdlog)