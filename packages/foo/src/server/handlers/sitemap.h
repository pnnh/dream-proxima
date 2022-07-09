//
// Created by ubuntu on 2/16/22.
//

#ifndef SFX_SERVER_API_SITEMAP_H
#define SFX_SERVER_API_SITEMAP_H

#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/asio.hpp>
#include <chrono>
#include <cstdlib>
#include <ctime>
#include <iostream>
#include <memory>

void HandleSitemap(boost::beast::http::response<boost::beast::http::dynamic_body> &response);


#endif //SFX_SERVER_API_SITEMAP_H
