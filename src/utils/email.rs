use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

use crate::database::Database;

pub async fn send_password_reset_email(to: &str, reset_token: &str) -> Result<(), Box<dyn Error>> {
    let email = Message::builder()
        .from("noreply@libri.com".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Password Reset")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Your password reset token is: {}\nThe link to the reset page: http://localhost:5173/libri_project/reset-password?token={}.", reset_token, reset_token))
        .unwrap();

    let smtp_username = dotenv::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_password = dotenv::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        // If email was sent successfully, print confirmation message
        Ok(_) => println!("Email sent successfully!"),
        // If there was an error sending the email, print the error
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

    Ok(())
}

pub async fn generate_reset_token() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const TOKEN_LEN: usize = 32;

    let mut rng = rand::thread_rng();

    let token: String = (0..TOKEN_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    token
}

pub async fn store_reset_token(
    db: &mut Database,
    user_id: i32,
    reset_token: &str,
) -> Result<(), Box<dyn Error>> {
    // Delete expired tokens
    sqlx::query!("DELETE FROM reset_tokens WHERE token_expires < DATE_SUB(NOW(), INTERVAL 1 HOUR)")
        .execute(&db.pool)
        .await?;

    // Insert new token
    sqlx::query!(
        "INSERT INTO reset_tokens (user_id, token, token_expires) VALUES (?, ?, DATE_ADD(NOW(), INTERVAL 1 HOUR))",
        user_id,
        reset_token
    )
    .execute(&db.pool)
    .await?;

    Ok(())
}
