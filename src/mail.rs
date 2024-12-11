use std::ops::RangeFull;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use nvim_oxi::print;

use crate::config::MailConfigs;
use crate::error::JustError;

pub fn sent_mail(creds: &MailConfigs) -> Result<(), JustError> {
    let current_buf = nvim_oxi::api::get_current_buf();
    let lines: Vec<String> = current_buf
        .get_lines(RangeFull, false)?
        .map(|line| line.to_string())
        .collect();

    let frist_line = &lines[0].trim().to_string();
    let subject = &lines[1].trim().to_string();
    let sep = &lines[2];

    let (to, contents) = if !frist_line.as_str().contains("@") && sep != "---" {
        (&creds.default_to.to_string(), &lines.join("\n"))
    } else if !frist_line.as_str().contains("@") {
        print!("invalid format!\nwhere is @ in this email id?");
        return Ok(());
    } else if sep.as_str() != "---" {
        print!("invalid format!\n\nName <name@example.com>\nSubject\n---\nbody...");
        return Ok(());
    } else {
        (&lines[0].trim().to_string(), &lines[3..].join("\n"))
    };

    let from = &creds.from;
    let email = Message::builder()
        .from(from.to_string().as_str().parse()?)
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(contents.to_string())?;

    let smtp = &creds.smtp;
    let creds = Credentials::new(creds.email.to_string(), creds.password.to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&smtp.to_string())?
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            print!("Email sent successfully!");
            Ok(())
        }
        Err(e) => {
            print!("Could not send email: {e}");
            Ok(())
        }
    }
}
