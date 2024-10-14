use crate::database::Database;
use crate::models::user::User;

use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    email: Option<String>,
    username: String,
    password: String,
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
