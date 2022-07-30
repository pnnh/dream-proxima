include(ExternalProject)

set(MultiMarkdown_ROOT ${CMAKE_BINARY_DIR}/thirdparty/MultiMarkdown)
set(MultiMarkdown_GIT_TAG 6.6.0)  # 指定版本
set(MultiMarkdown_GIT_URL https://github.com/fletcher/MultiMarkdown-6.git)  # 指定git仓库地址
set(MultiMarkdown_CONFIGURE cd ${MultiMarkdown_ROOT}/src/MultiMarkdown && cmake -D CMAKE_INSTALL_PREFIX=${MultiMarkdown_ROOT} .)  # 指定配置指令（注意此处修改了安装目录，否则默认情况下回安装到系统目录）
set(MultiMarkdown_MAKE cd ${MultiMarkdown_ROOT}/src/MultiMarkdown && make)  # 指定编译指令（需要覆盖默认指令，进入我们指定的MultiMarkdown_ROOT目录下）
set(MultiMarkdown_INSTALL cd ${MultiMarkdown_ROOT}/src/MultiMarkdown && make install)  # 指定安装指令（需要覆盖默认指令，进入我们指定的MultiMarkdown_ROOT目录下）

ExternalProject_Add(MultiMarkdown
        PREFIX ${MultiMarkdown_ROOT}
        GIT_REPOSITORY ${MultiMarkdown_GIT_URL}
        GIT_TAG ${MultiMarkdown_GIT_TAG}
        CONFIGURE_COMMAND ${MultiMarkdown_CONFIGURE}
        BUILD_COMMAND ${MultiMarkdown_MAKE}
        INSTALL_COMMAND ${MultiMarkdown_INSTALL}
        )

# 指定编译好的静态库文件的路径
set(MultiMarkdown_LIB ${MultiMarkdown_ROOT}/lib/MultiMarkdown/libMultiMarkdown.a)
# 指定头文件所在的目录
set(MultiMarkdown_INCLUDE_DIR ${MultiMarkdown_ROOT}/include)

message("Rescan0 ${MultiMarkdown_FOUND}")

#if (NOT ${MultiMarkdown_FOUND})
#    #rerun cmake in initial build
#    #will update cmakecache/project files on first build
#    #so you may have to reload project after first build
#    message("Rescan1")
#    add_custom_target(Rescan ${CMAKE_COMMAND} ${CMAKE_SOURCE_DIR} DEPENDS MultiMarkdown)
#else ()
#    #Rescan becomes a dummy target after first build
#    #this prevents cmake from rebuilding cache/projects on subsequent builds
#    message("Rescan2")
#    add_custom_target(Rescan)
#endif ()


add_custom_target(Rescan ${CMAKE_COMMAND} ${CMAKE_SOURCE_DIR} DEPENDS MultiMarkdown)