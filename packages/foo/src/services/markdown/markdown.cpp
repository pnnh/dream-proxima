//
//  main.cpp
//  MarkdownParser
//

#include <fstream>         // std::ofstream
#include "markdown.h"
#include "mdtransform.hpp"  // 需要实现的 Markdown 解析类
#include "src/utils/mime.h"

void HandleMarkdown(boost::beast::http::response<boost::beast::http::dynamic_body> &response_) {

    // 装载构造 Markdown 文件
    MarkdownTransform transformer("static/test.md");

    // 编写一个 `getTableOfContents()` 方法来获取 Markdown 文件 HTML 格式的目录
    std::string table = transformer.getTableOfContents();

    // 编写一个 `getContents()` 方法来获取 Markdown 转成 HTML 后的内容
    std::string contents = transformer.getContents();

    // 准备要写入的 HTML 文件头尾信息
    std::string head = "<!DOCTYPE html><html><head>\
        <meta charset=\"utf-8\">\
        <title>Markdown</title>\
        <link rel=\"stylesheet\" href=\"github-markdown.css\">\
        </head><body><article class=\"markdown-body\">";
    std::string end = "</article></body></html>";

    response_.result(boost::beast::http::status::ok);
    response_.keep_alive(false);
    response_.set(boost::beast::http::field::server, "Beast");
    response_.set(boost::beast::http::field::content_type, "text/html");

    boost::beast::ostream(response_.body()) << head + table + contents + end;
}
