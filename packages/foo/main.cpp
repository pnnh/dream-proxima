
#include <iostream>
#include <spdlog/spdlog.h>
#include <stdio.h>

extern "C" {
#include "libMultiMarkdown.h"
}
//
// extern char *mmd_string_convert(const char *source, unsigned long extensions,
//                                short format, short language);

int main(int argc, char *argv[]) {
  spdlog::info("i love c++");
  std::cout << "Hello, World!" << std::endl;

  const char *source = "# hello\nworld";
  char *target = mmd_string_convert(source, EXT_COMPLETE, FORMAT_HTML, ENGLISH);
  printf("%s", target);
  free(target);
}