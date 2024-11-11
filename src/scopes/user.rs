use crate::{
    database::Database,
    extractors::authentication_token::AuthenticationToken,
    models::user::{User, UserGroup},
    server::WebData,
    utils::jwt::generate_jwt_token,
};
use actix_web::{web, HttpResponse, Responder, Scope};
use serde::{Deserialize, Serialize};
use std::env;

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/sign-in", web::post().to(sign_in))
        .route("/protected", web::get().to(protected_route))
        .route("/sign-up", web::post().to(sign_up))
        .route("/info", web::get().to(get_user_info))
        .route("/change-info", web::post().to(change_user_info))
        .route("/forgot-password", web::post().to(forgot_password))
        .route("/reset-password", web::post().to(reset_password))
        .route("/change-password", web::post().to(change_password))
        // .route("/books", web::get().to(get_user_books))
        .route("/cart", web::get().to(get_user_cart))
}

#[derive(Deserialize)]
struct UserInfoJson {
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    group: UserGroup,
}

async fn sign_in(data: web::Json<UserInfoJson>, secret: web::Data<WebData>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = User {
        id: None,
        email: None,
        username: data.username.clone(),
        password: data.password.clone(),
        group: UserGroup::None,
    };

    match User::login_with_password(&mut db, user).await {
        Ok(logged_in_user) => HttpResponse::Ok().json(LoginResponse {
            token: generate_jwt_token(
                logged_in_user.id.unwrap() as usize,
                secret.auth_secret.clone(),
            )
            .await,
            group: logged_in_user.group,
        }),
        Err(e) => HttpResponse::Unauthorized().json(format!("Signin failed: {}", e)),
    }
}

async fn sign_up(data: web::Json<UserInfoJson>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = User {
        id: None,
        email: data.email.clone(),
        username: data.username.clone(),
        password: data.password.clone(),
        group: UserGroup::User,
    };
    match User::new(&mut db, user).await {
        Ok(_) => HttpResponse::Created().json("Signup successful"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Signup failed: {:?}", e)),
    }
}

#[derive(Serialize)]
struct ProtectedResponse {
    message: String,
}

async fn protected_route(_auth_token: AuthenticationToken) -> impl Responder {
    HttpResponse::Ok().json(ProtectedResponse {
        message: "Auth success".to_string(),
    })
}

// async fn get_user_books(auth_token: AuthenticationToken) -> impl Responder {
//     let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
//         .await
//         .unwrap();

//     match User::get_books(&mut db, auth_token.id as i32).await {
//         Ok(user_books) => HttpResponse::Ok().json(user_books),
//         Err(e) => {
//             HttpResponse::InternalServerError().json(format!("Error fetching user books: {:?}", e))
//         }
//     }
// }

async fn get_user_cart(auth_token: AuthenticationToken) -> impl Responder {
    let mut db = Database::new(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match User::get_cart(&mut db, auth_token.id as i32).await {
        Ok(cart) => HttpResponse::Ok().json(cart),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error getting cart: {:?}", e)),
    }
}

async fn get_user_info(auth_token: AuthenticationToken) -> impl Responder {
    let mut db = Database::new(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match User::get_info(&mut db, auth_token.id as i32).await {
        Ok(user_info) => HttpResponse::Ok().json(user_info),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error getting cart: {:?}", e)),
    }
}

#[derive(Deserialize, Debug)]
pub struct ChangeUserJson {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub billing_address: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
}

async fn change_user_info(
    auth_token: AuthenticationToken,
    data: web::Json<ChangeUserJson>,
) -> impl Responder {
    let mut db = Database::new(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match User::change_info(&mut db, auth_token.id as i32, data.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Successfully modified"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error getting cart: {:?}", e)),
    }
}

async fn forgot_password(data: web::Json<UserInfoJson>) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user = User {
        id: None,
        email: data.email.clone(),
        username: None,
        password: None,
        group: UserGroup::User,
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
struct ResetPasswordJson {
    password: String,
}

async fn reset_password(
    query: web::Query<ResetPasswordQuery>,
    data: web::Json<ResetPasswordJson>,
) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match User::reset_password(&mut db, query.token.to_string(), data.password.to_string()).await {
        Ok(_) => HttpResponse::Ok().json("Reset password successful"),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Reset password failed: {:?}", e))
        }
    }
}

#[derive(Deserialize)]
struct ChangePasswordJson {
    old_password: String,
    new_password: String,
}

async fn change_password(
    auth_token: AuthenticationToken,
    data: web::Json<ChangePasswordJson>,
) -> impl Responder {
    let mut db = Database::new(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    match User::change_password(
        &mut db,
        auth_token.id as i32,
        data.old_password.clone(),
        data.new_password.clone(),
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().json("Change password successful"),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Change password failed: {:?}", e))
        }
    }
}
