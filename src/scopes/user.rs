use crate::{
    database::Database,
    extractors::authentication_token::AuthenticationToken,
    models::{book::Book, user::User},
    utils::jwt::encode_token,
};
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use std::env;

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/sign-in", web::post().to(sign_in))
        .route("/protected", web::get().to(protected_route))
        .route("/sign-up", web::post().to(sign_up))
        .route("/forgot-password", web::post().to(forgot_password))
        .route("/reset-password", web::post().to(reset_password))
        .route("/{user_id}/buy-book/{book_id}", web::patch().to(buy_book))
        .route("/{user_id}/books", web::get().to(get_user_books))
}

#[derive(Deserialize)]
struct UserInfo {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
struct LoginResponse {
    message: String,
    token: String,
}

async fn sign_in(data: web::Json<UserInfo>, secret: web::Data<String>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = User {
        id: None,
        email: None,
        username: data.username.clone(),
        password: data.password.clone(),
        group: None,
    };

    match User::login_with_password(&mut db, user).await {
        Ok(logged_in_user) => HttpResponse::Ok().json(LoginResponse {
            message: "Signin successful".to_owned(),
            token: encode_token(logged_in_user.id.unwrap() as usize, secret).await,
        }),
        Err(e) => HttpResponse::Unauthorized().json(format!("Signin failed: {}", e)),
    }
}

async fn sign_up(data: web::Json<UserInfo>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = User {
        id: None,
        email: data.email.clone(),
        username: data.username.clone(),
        password: data.password.clone(),
        group: None,
    };
    match User::new(&mut db, user).await {
        Ok(_) => HttpResponse::Ok().json("Signup successful"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Signup failed: {:?}", e)),
    }
}

async fn buy_book(path: web::Path<(i32, i32)>) -> impl Responder {
    let (user_id, book_id) = path.into_inner();
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match Book::buy_book(&mut db, user_id, book_id).await {
        Ok(_) => HttpResponse::Ok().json("Book bought"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error buying book: {:?}", e)),
    }
}

async fn get_user_books(user_id: web::Path<i32>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match User::find_user_books(&mut db, user_id.into_inner()).await {
        Ok(user_books) => HttpResponse::Ok().json(user_books),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Error fetching user books: {:?}", e))
        }
    }
}

async fn forgot_password(web::Form(form): web::Form<UserInfo>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = User {
        id: None,
        email: form.email,
        username: None,
        password: None,
        group: None,
    };
    match User::forgot_password(&mut db, user).await {
        Ok(_) => HttpResponse::Ok().json("Forgot password successful"),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Forgot password failed: {:?}", e))
        }
    }
}

#[derive(Deserialize)]
struct ResetPasswordQuery {
    token: String,
}

#[derive(Deserialize)]
struct ResetPassword {
    password: String,
    repeat_password: String,
}

async fn reset_password(
    query: web::Query<ResetPasswordQuery>,
    form: web::Form<ResetPassword>,
) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    if form.password == form.repeat_password {
        match User::reset_password(&mut db, query.token.to_string(), form.password.to_string())
            .await
        {
            Ok(_) => HttpResponse::Ok().json("Reset password successful"),
            Err(e) => {
                HttpResponse::InternalServerError().json(format!("Reset password failed: {:?}", e))
            }
        }
    } else {
        HttpResponse::BadRequest().json("Passwords do not match")
    }
}

#[derive(Serialize)]
struct ProtectedResponse {
    message: String,
    id: usize,
}

// Example on how it probably should be handled
async fn protected_route(auth_token: AuthenticationToken) -> impl Responder {
    HttpResponse::Ok().json(ProtectedResponse {
        message: "Authorized".to_owned(),
        id: auth_token.id,
    })
}
