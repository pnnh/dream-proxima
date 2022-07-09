//
// Created by ubuntu on 12/26/21.
//

#ifndef CPP_SERVER_MIME_H
#define CPP_SERVER_MIME_H

#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>

boost::beast::string_view mime_type(boost::beast::string_view path);

#endif //CPP_SERVER_MIME_H
