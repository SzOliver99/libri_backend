use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

pub struct Email;
impl Email {
    pub async fn send_password_reset_email(
        to: &str,
        reset_token: &str,
    ) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from("noreply@library-basement.com".parse().unwrap())
            .to(to.parse().unwrap())
            .subject("Jelszó visszaállítási kérelem")
            .header(ContentType::TEXT_PLAIN)
            .body(format!("\nA jelszó-visszaállítási kód a következő: {} \n Vagy a következő linkre kattintva: https://library-basement.vercel.app/reset-password?token={}.", reset_token, reset_token))
            .unwrap();

        let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let creds = Credentials::new(smtp_username, smtp_password);

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("E-mail sikeresen elküldve!"),
            Err(e) => eprintln!("Hiba történt: {:?}", e),
        }

        Ok(())
    }

    pub async fn send_authentication_code(to: &str, code: &str) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from("noreply@library-basement.com".parse().unwrap())
            .to(to.parse().unwrap())
            .subject("E-mail hitelesítési kód")
            .header(ContentType::TEXT_PLAIN)
            .body(format!("Az Ön e-mail hitelesítési kódja: {}", code))
            .unwrap();

        let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
        let creds = Credentials::new(smtp_username, smtp_password);

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("E-mail sikeresen elküldve!"),
            Err(e) => eprintln!("Hiba történt: {:?}", e),
        }

        Ok(())
    }
}

use rand::Rng;
pub struct Token;
impl Token {
    pub fn generate_reset_token() -> String {
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

    pub fn generate_six_digit_number() -> String {
        const TOKEN_LEN: usize = 6;

        let mut rng = rand::thread_rng();
        let token: String = (0..TOKEN_LEN)
            .map(|_| {
                let idx: u8 = rng.gen_range(0..10);
                idx.to_string()
            })
            .collect();

        token
    }
}
