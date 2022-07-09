//
// Created by ubuntu on 2/16/22.
//

#ifndef SFX_SERVER_API_ARTICLE_H
#define SFX_SERVER_API_ARTICLE_H

#include <string>
#include <chrono>

struct ArticleModel {
    std::string pk;
    std::string title;
    std::string body;
    std::chrono::system_clock::time_point create_time;
    std::chrono::system_clock::time_point update_time;
    std::string creator;
    std::string keywords;
    std::string description;
};


#endif //SFX_SERVER_API_ARTICLE_H
