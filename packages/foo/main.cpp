
#include "libMultiMarkdown.h"
#include <iostream>
#include <spdlog/spdlog.h>

extern "C" {
void foo_rs(uint32_t a, uint32_t b);
}

int main(int argc, char *argv[]) {
  spdlog::info("i love c++");
  std::cout << "Hello, World!" << std::endl;
}