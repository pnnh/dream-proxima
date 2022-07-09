//
// Created by Larry on 2021/12/21.
//

#ifndef CPP_SERVER_STATE_H
#define CPP_SERVER_STATE_H

#include <cstdlib>
#include <chrono>

namespace my_program_state {

std::size_t request_count();

std::time_t now();
}

#endif //CPP_SERVER_STATE_H
