use lettre::{
    Message, SmtpTransport, Transport, message::SinglePart,
    transport::smtp::authentication::Credentials,
};

pub async fn send_mail(recipient_email: &str, otp: u32) -> Result<(), String> {
    let sender_email = std::env::var("GMAIL_APP_EMAIL")
        .map_err(|_| "Missing GMAIL_APP_EMAIL environment variable".to_string())?;
    let recipient = recipient_email
        .parse()
        .map_err(|e| format!("Failed to parse recipient email: {}", e))?;

    let sender = sender_email
        .parse()
        .map_err(|e| format!("Failed to parse sender email: {}", e))?;
    let email_result = tokio::task::spawn_blocking(move || {
        let email = Message::builder()
            .from(sender)
            .to(recipient)
            .subject(format!("Your OTP is: {}", otp))
            .singlepart(SinglePart::plain(format!(
                "Your OTP is: {}. It will expire in 5 minutes.",
                otp
            )))
            .map_err(|e| format!("Failed to create email body: {}", e))?;

        let app_password = std::env::var("GMAIL_APP_PASSWORD")
            .map_err(|_| "Missing GMAIL_APP_PASSWORD environment variable".to_string())?;

        let creds = Credentials::new(sender_email.to_string(), app_password);

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .map_err(|e| format!("Failed to configure SMTP relay: {}", e))?
            .credentials(creds)
            .build();

        mailer
            .send(&email)
            .map_err(|e| format!("Failed to send email: {}", e))
    })
    .await;

    match email_result {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(format!("Task execution failed: {}", e)),
    }
}
