use std::sync::Arc;
use bcrypt::verify;


use axum::{
    body,
    Form,
    extract::State,
    Router,
    routing,
    http::{header, Response, StatusCode, },
    response::{IntoResponse, Html},
    Json,
};
use cookie::time::Duration;

use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use minijinja::context;

use crate::{
    models::{
        User,
        SimpleUser,
        TokenClaims,
    },
    http::AppState,
};

use super::ENV;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login",
            routing::get(login).post(do_login)
        )
        .route("/logout",
            routing::get(logout)
        )
}


pub async fn get_token(
    app_state: &Arc<AppState>,
    body: SimpleUser
) -> Result<String, (StatusCode, Json<serde_json::Value>)>{
//) -> Result<Json<serde_json::Value>,(StatusCode, Json<serde_json::Value>)>{
    tracing::info!("init login");
    let user = User::read_by_name(&app_state.pool, &body.username)
        .await
        .map_err(|_e| {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": "Invalid name or password. Please <a href='/login'>log in</a>",
            });
            (StatusCode::UNAUTHORIZED, Json(error_response))
        })?;
    if !user.active {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "User inactive. Please contact admin."
        });
        return Err((StatusCode::UNAUTHORIZED, Json(error_response)));

    }
    if !verify(body.password, &user.hashed_password).unwrap_or(false){
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Invalid name or password. Please <a href='/login'>log in</a>"
        });
        return Err((StatusCode::UNAUTHORIZED, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.username.to_string(),
        exp,
        iat,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(app_state.jwt_secret.as_bytes()),
    ).map_err(|e| {
        let error_response = serde_json::json!({
            "status": "error",
            "message": format!("Encoding JWT error: {}. Please <a href='/login'>log in</a>", e),
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })
}

pub async fn do_login(
    State(app_state): State<Arc<AppState>>,
    Form(user_data): Form<SimpleUser>,
) -> impl IntoResponse{
    tracing::info!("Post data: {:?}", user_data);
    match get_token(&app_state, user_data).await {
        Ok(token) => {
            let cookie = Cookie::build(("token", token.to_owned()))
                .path("/")
                .max_age(Duration::hours(1))
                .same_site(SameSite::Lax)
                .http_only(true)
                .build();
            tracing::info!("El token: {}", token.to_string());
            
            Ok(Response::builder()
                .status(StatusCode::SEE_OTHER)
                .header(header::LOCATION, "/")
                .header(header::SET_COOKIE, cookie.to_string())
                .body(body::Body::empty())
                .unwrap())
        },
        Err(e) => {
            tracing::info!("{:?}", e);
            let template = ENV.get_template("error.html").unwrap();
            let ctx = context! {
                title             => "PodMixer",
                error_title       => "Error",
                error_description => e.1.get("message"),
            };
            Err(Html(template.render(ctx).unwrap()))
        }
    }
}

pub async fn logout() -> impl IntoResponse {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    tracing::info!("The cookie: {}", cookie.to_string());

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, "/")
        .header(header::SET_COOKIE, cookie.to_string())
        .body(body::Body::empty())
        .unwrap()
}

pub async fn login(
    State(_app_state): State<Arc<AppState>>,
) -> impl IntoResponse{
    let template = ENV.get_template("login.html").unwrap();
    let ctx = context! {
        title => "PodMixer",
    };
    Html(template.render(ctx).unwrap())
}


