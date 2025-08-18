use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor, message::SinglePart,
    transport::smtp::authentication::Credentials,
};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

// Create a shared mailer that lives for the app's lifetime
static MAILER: Lazy<Arc<Mutex<AsyncSmtpTransport<Tokio1Executor>>>> = Lazy::new(|| {
    let sender_email = std::env::var("GMAIL_APP_EMAIL").expect("Missing GMAIL_APP_EMAIL");
    let app_password = std::env::var("GMAIL_APP_PASSWORD").expect("Missing GMAIL_APP_PASSWORD");

    let creds = Credentials::new(sender_email.clone(), app_password);

    let transport = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    Arc::new(Mutex::new(transport))
});

pub async fn send_mail(recipient_email: &str, otp: u32) -> Result<(), String> {
    let sender_email =
        std::env::var("GMAIL_APP_EMAIL").map_err(|_| "Missing GMAIL_APP_EMAIL".to_string())?;
    let sender = sender_email
        .parse()
        .map_err(|e| format!("Failed to parse sender email: {}", e))?;
    let recipient = recipient_email
        .parse()
        .map_err(|e| format!("Failed to parse recipient email: {}", e))?;

    let email = Message::builder()
        .from(sender)
        .to(recipient)
        .subject(format!("Your OTP is: {}", otp))
        .singlepart(SinglePart::plain(format!(
            "Your OTP is: {}. It will expire in 5 minutes.",
            otp
        )))
        .map_err(|e| format!("Failed to create email body: {}", e))?;

    let  mailer = MAILER.lock().await;

    mailer
        .send(email)
        .await
        .map_err(|e| format!("Failed to send email: {}", e))?;

    Ok(())
}
