//
// Created by ubuntu on 2/17/22.
//

#include <boost/date_time/posix_time/posix_time.hpp>
#include "datetime.h"


std::chrono::system_clock::time_point makeTimePoint(const std::string &s) {
    using namespace boost::posix_time;
    using namespace std::chrono;

    const ptime ts = time_from_string(s);
    auto seconds = to_time_t(ts);
    time_duration td = ts.time_of_day();
    auto microseconds = td.fractional_seconds();
    auto d = std::chrono::seconds{seconds} + std::chrono::microseconds{microseconds};
    system_clock::time_point tp{duration_cast<system_clock::duration>(d)};
    return tp;
}

std::string formatTime(const std::chrono::system_clock::time_point &time_point) {
    std::time_t now = std::chrono::system_clock::to_time_t(time_point);

    char mbstr[100];
    std::strftime(mbstr, sizeof(mbstr), "%FT%TZ", std::localtime(&now));
    return mbstr;
}