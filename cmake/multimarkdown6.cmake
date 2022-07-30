include(FetchContent)

set(MultiMarkdown_GIT_TAG 6.6.0)  # 指定版本
set(MultiMarkdown_GIT_URL https://github.com/fletcher/MultiMarkdown-6.git)  # 指定git仓库地址

FetchContent_Declare(MultiMarkdown
        GIT_REPOSITORY ${MultiMarkdown_GIT_URL}
        GIT_TAG ${MultiMarkdown_GIT_TAG}
        #        BUILD_IN_SOURCE true
        #        CONFIGURE_COMMAND "cmake ."
        #        BUILD_COMMAND "make"
        #        INSTALL_COMMAND "make install"
        )

#FetchContent_GetProperties(MultiMarkdown)
#if (NOT MultiMarkdown_POPULATED)
#    FetchContent_Populate(MultiMarkdown)
#endif ()

FetchContent_MakeAvailable(MultiMarkdown)
#add_library(MultiMarkdown INTERFACE)
#FetchContent_GetProperties(MultiMarkdown)
message(STATUS xxx ${multimarkdown_BINARY_DIR})
include_directories(${multimarkdown_SOURCE_DIR}/src)
link_directories(${multimarkdown_BINARY_DIR})

#add_subdirectory(${multimarkdown_SOURCE_DIR})