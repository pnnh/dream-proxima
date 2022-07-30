#cmake_minimum_required(VERSION 3.22)
#project(proxima C CXX)
#
#add_subdirectory(packages/foo)


cmake_minimum_required(VERSION 3.22)
project(Proxima C CXX)


set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_VERBOSE_MAKEFILE ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

if (APPLE)
    # xcode自带的工具链缺少一些功能，这里使用命令行工具链
    set(CMAKE_OSX_SYSROOT "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk")
    # mac m1 下通过homebrew安装的程序默认在/opt目录下，需要添加到引用路径里去
    if (${CMAKE_SYSTEM_PROCESSOR} STREQUAL "arm64")
        link_directories("/opt/homebrew/lib")
        include_directories("/opt/homebrew/include")
    endif ()
endif ()


set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake;${CMAKE_MODULE_PATH}")
include(spdlog)

find_package(PkgConfig REQUIRED)

find_package(Boost COMPONENTS system thread REQUIRED)
if (Boost_FOUND)
    message(STATUS "Found Boost: ${FOLLY_LIBRARIES}")
    INCLUDE_DIRECTORIES(${Boost_INCLUDE_DIRS})
    LINK_LIBRARIES(${Boost_LIBRARIES})
endif (Boost_FOUND)

find_package(folly REQUIRED)
if (folly_FOUND)
    message(STATUS "Found folly: ${FOLLY_LIBRARIES}")
    INCLUDE_DIRECTORIES(${folly_INCLUDE_DIRS})
    LINK_LIBRARIES(${FOLLY_LIBRARIES})
endif (folly_FOUND)

add_executable(sfx-server packages/foo/main.cpp)

#target_link_libraries(sfx-server PRIVATE spdlog)