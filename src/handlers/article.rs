use async_trait::async_trait;
use axum::body::{Bytes, HttpBody};
use axum::extract::rejection::{JsonDataError, JsonRejection, JsonSyntaxError};
use axum::extract::{FromRequest, RequestParts};
use axum::headers::HeaderValue;
use axum::http::header;
use axum::response::{Html, IntoResponse, Response};
use axum::{extract::Extension, extract::Path, http::StatusCode, BoxError, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

use nanoid::nanoid;
use serde::de::DeserializeOwned;

use crate::handlers::jwt::{AuthError, Claims};
use crate::handlers::State;
use crate::utils::article::{build_body, TocItem};
use crate::{layers, utils};

pub async fn article_read_handler<'a>(
    Path(params): Path<HashMap<String, String>>,
    Extension(state): Extension<Arc<State<'_>>>,
) -> Result<Html<String>, (StatusCode, String)> {
    let pk = params
        .get("pk")
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "pk参数有误".to_string()))?;
    tracing::debug!("pk:{}", pk,);

    let conn = state.pool.get().await.map_err(layers::internal_error)?;

    let query_result = conn
        .query(
            "select articles.pk, articles.title, articles.body, 
articles.description, articles.update_time, articles.creator, articles.keywords,
accounts.nickname, accounts.email, accounts.description, accounts.photo, 
    accounts.create_time as accounts_create_time,
articles_views.views
from articles
left join accounts on articles.creator = accounts.pk
left join articles_views on articles.pk = articles_views.pk
where articles.pk = $1;",
            &[&pk],
        )
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    if query_result.len() < 1 {
        return Err((StatusCode::NOT_FOUND, "文章未找到".to_string()));
    }

    let title: &str = query_result[0].get("title");
    let body: serde_json::Value = query_result[0].get("body");
    let description: &str = query_result[0].get("description");
    let update_time: chrono::NaiveDateTime = query_result[0].get("update_time");
    let creator: String = query_result[0].get("creator");
    let keywords: String = query_result[0].get("keywords");
    let views: Option<i64> = query_result[0].get("views");
    let creator_nickname: &str = query_result[0].get("nickname");
    let creator_email: Option<&str> = query_result[0].get("email");
    let creator_description: Option<&str> = query_result[0].get("description");
    let creator_photo: Option<&str> = query_result[0].get("photo");
    let creator_create_time: chrono::NaiveDateTime = query_result[0].get("accounts_create_time");

    let mut toc_list: Vec<TocItem> = Vec::new();
    toc_list.push(TocItem {
        title: title.to_string(),
        header: 0,
    });
    let body_html = build_body(&mut toc_list, &body).or_else(|err| {
        tracing::error!("解析body出错: {}", err);
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "文章解析出错".to_string(),
        ))
    })?;

    let page_data = &json!({
        "pk": pk.to_string(),
        "title": title.to_string(),
        "body_html": body_html,
        "description": description.to_string(),
        "update_time_formatted": update_time.format("%Y年%m月%d日 %H:%M").to_string(),
        "creator": {
            "pk": creator,
            "email": creator_email.unwrap_or(""),
            "description": creator_description.unwrap_or(""),
            "nickname": creator_nickname.to_string(),
            "photo": utils::get_photo_or_default(creator_photo.unwrap_or("")),
            "create_time": creator_create_time.format("%Y年%m月%d日 %H:%M").to_string(),
        },
        "views": views.unwrap_or(0),
        "keywords": keywords,
        "toc_list": toc_list,
    });
    //println!("page_data: {:?}", page_data);

    let result = state
        .registry
        .render("article_read", page_data)
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(Html(result))
}

#[derive(Debug, Deserialize)]
struct CreatePayload {
    title: String,
    body: String,
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
struct ProtectedPayload<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ProtectedPayload<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if json_content_type(req) {
            let bytes = Bytes::from_request(req)
                .await
                .map_err(|err| AuthError::InvalidData)?;

            let value = match serde_json::from_slice(&bytes) {
                Ok(value) => value,
                Err(err) => {
                    let rejection = match err.classify() {
                        serde_json::error::Category::Data => err,
                        serde_json::error::Category::Syntax | serde_json::error::Category::Eof => {
                            err
                        }
                        serde_json::error::Category::Io => {
                            if cfg!(debug_assertions) {
                                // we don't use `serde_json::from_reader` and instead always buffer
                                // bodies first, so we shouldn't encounter any IO errors
                                unreachable!()
                            } else {
                                err
                            }
                        }
                    };
                    return Err(AuthError::InvalidToken);
                }
            };

            Ok(ProtectedPayload {
                // 0: Claims {
                //     // sub: "".to_string(),
                //     // company: "".to_string(),
                //     exp: 0,
                // },
                0: value,
            })
        } else {
            Err(AuthError::InvalidToken)
        }
    }
}

impl<T> IntoResponse for ProtectedPayload<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_vec(&self.0) {
            Ok(bytes) => (
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
                )],
                bytes,
            )
                .into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                )],
                err.to_string(),
            )
                .into_response(),
        }
    }
}

fn json_content_type<B>(req: &RequestParts<B>) -> bool {
    let content_type = if let Some(content_type) = req.headers().get(header::CONTENT_TYPE) {
        content_type
    } else {
        return false;
    };

    let content_type = if let Ok(content_type) = content_type.to_str() {
        content_type
    } else {
        return false;
    };

    let mime = if let Ok(mime) = content_type.parse::<mime::Mime>() {
        mime
    } else {
        return false;
    };

    let is_json_content_type = mime.type_() == "application"
        && (mime.subtype() == "json" || mime.suffix().map_or(false, |name| name == "json"));

    is_json_content_type
}
#[derive(Debug, Deserialize)]
struct CreateBody {
    pk: String,
}

pub async fn article_create_handler(
    ProtectedPayload(payload): ProtectedPayload<CreatePayload>,
    Extension(state): Extension<Arc<State<'_>>>,
) -> Result<Json<CreateBody>, AuthError> {
    tracing::debug!("pk:{:?}", payload);

    let conn = state
        .pool
        .get()
        .await
        .map_err(|err| AuthError::InvalidToken)?;

    let pk = nanoid!(12);
    let query_result = conn
        .execute(
            "insert into articles(pk, title, body, create_time, update_time, creator, keywords, description)
values(pk, 'a', 'b','2022-07-05 21:54:53','2022-07-05 21:54:57','x','y','z');",
            &[&pk],
        )
        .await
        .map_err(|err| AuthError::InvalidToken)?;

    //println!("page_data: {:?}", page_data);

    Ok(Json(CreateBody { pk: pk }))
}
