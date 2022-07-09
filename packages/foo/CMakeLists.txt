cmake_minimum_required(VERSION 3.0)
project(Foo CXX)

add_executable(foo main.cpp)
install(TARGETS foo DESTINATION .)

include(../../cmake/cargo.cmake)

add_dependencies(foo rs)
target_link_directories(foo PRIVATE ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR})
target_link_libraries(foo foo_rs)


set(CMAKE_CXX_STANDARD 20)
set(CMAKE_VERBOSE_MAKEFILE on)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

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

add_executable(sfx-server-api
        src/main.cc
        src/server/http_connection.cc
        src/server/state.cc
        src/utils/md5.cc src/server/handlers/index.cpp src/server/handlers/index.h
        src/utils/mime.h src/utils/mime.cpp
        src/services/markdown/markdown.cpp src/services/markdown/mdtransform.hpp src/services/markdown/markdown.h
        src/server/handlers/sitemap.cpp src/server/handlers/sitemap.h src/models/article.cpp src/models/article.h
        src/utils/datetime.cpp src/utils/datetime.h)
