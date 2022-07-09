//
// Created by Larry on 2021/12/21.
//

#include "state.h"

namespace my_program_state {

std::size_t request_count() {
  static std::size_t count = 0;
  return ++count;
}

std::time_t now() {
  return std::time(nullptr);
}
}