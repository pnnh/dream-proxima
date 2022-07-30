## macOS构建

尝试了使用clang-14和gcc-12来构建，都是通过brew命令安装

发现，当使用clang-14时folly无法正常编译，使用gcc-12可以

使用clang-14时如果把C++标准设置为17就可以通过编译，不过得同时将SYSROOT设置为CommandLineTools，不能用xcode自带的

相反的是，如果用gcc-12，则不能设置SYSROOT，否则编译不通过