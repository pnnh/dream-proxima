//
// Created by Larry on 2021/12/21.
//

#ifndef CPP_SERVER_MD5_H
#define CPP_SERVER_MD5_H

#include <iostream>
#include <algorithm>
#include <iterator>
#include <boost/uuid/detail/md5.hpp>
#include <boost/algorithm/hex.hpp>

using boost::uuids::detail::md5;

std::string toString(const md5::digest_type &digest);

#endif //CPP_SERVER_MD5_H
