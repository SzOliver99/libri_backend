use crate::database::Database;
use crate::models::user::User;

use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
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

#[post("/sign-up")]
pub async fn signup(web::Form(form): web::Form<Info>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = User {
        id: None,
        email: form.email,
        username: form.username,
        password: form.password,
    };
    match User::new(&mut db, user).await {
        Ok(_) => HttpResponse::Ok().json("Signup successful"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Signup failed: {:?}", e)),
    }
}

#[post("/sign-in")]
pub async fn signin(web::Form(form): web::Form<Info>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = User {
        id: None,
        email: None,
        username: form.username,
        password: form.password,
    };
    match User::login_with_password(&mut db, user).await {
        Ok(_) => HttpResponse::Ok().json("Signin successful"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Signin failed: {:?}", e)),
    }
}

#[post("/forgot-password")]
pub async fn forgot_password(web::Form(form): web::Form<Info>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = User {
        id: None,
        email: form.email,
        username: None,
        password: None,
    };
    match User::forgot_password(&mut db, user).await {
        Ok(_) => HttpResponse::Ok().json("Forgot password successful"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Forgot password failed: {:?}", e)),
    }
}

#[post("/reset-password")]
pub async fn reset_password(query: web::Query<ResetPasswordQuery>, form: web::Form<ResetPassword>) -> impl Responder {
    let mut db = Database::new(dotenv::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    if form.password == form.repeat_password {
        match User::reset_password(&mut db, query.token.to_string(), form.password.to_string()).await {
            Ok(_) => HttpResponse::Ok().json("Reset password successful"),
            Err(e) => HttpResponse::InternalServerError().json(format!("Reset password failed: {:?}", e)),
        }
    } else {
        HttpResponse::BadRequest().json("Passwords do not match")
    }
}
