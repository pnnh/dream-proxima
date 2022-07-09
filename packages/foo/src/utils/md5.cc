//
// Created by Larry on 2021/12/21.
//

#include "md5.h"

std::string toString(const md5::digest_type &digest) {
  const auto charDigest = reinterpret_cast<const char *>(&digest);
  std::string result;
  boost::algorithm::hex(charDigest, charDigest + sizeof(md5::digest_type), std::back_inserter(result));
  return result;
}