//
// Created by ubuntu on 2/17/22.
//

#ifndef SFX_SERVER_API_DATETIME_H
#define SFX_SERVER_API_DATETIME_H

#include <string>
#include <chrono>

std::chrono::system_clock::time_point makeTimePoint(const std::string &s);

std::string formatTime(const std::chrono::system_clock::time_point &time_point);

#endif //SFX_SERVER_API_DATETIME_H
