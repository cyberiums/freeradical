use super::email_template_service::EmailTemplateService;
use serde::Serialize;
use std::env;
use std::sync::Arc;
use log::{info, error};
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use aws_sdk_sesv2::Client as SesClient;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;

#[derive(Clone)]
pub enum EmailProviderType {
    Smtp,
    Ses,
    Mock, // For dev/test
}

#[derive(Clone)]
pub struct EmailService {
    template_service: EmailTemplateService,
    provider_type: EmailProviderType,
    smtp_transport: Option<SmtpTransport>,
    ses_client: Option<Arc<SesClient>>, // Arc for thread safety cloning
    from_email: String,
}

impl EmailService {
    pub async fn new(template_service: EmailTemplateService) -> Self {
        let provider_str = env::var("EMAIL_PROVIDER").unwrap_or_else(|_| "mock".to_string());
        let from_email = env::var("EMAIL_FROM").unwrap_or_else(|_| "noreply@freeradical.dev".to_string());
        
        let (provider_type, smtp_transport, ses_client) = match provider_str.to_lowercase().as_str() {
            "smtp" => {
                let host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
                let user = env::var("SMTP_USER").expect("SMTP_USER must be set");
                let pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
                
                let creds = Credentials::new(user, pass);
                let transport = SmtpTransport::relay(&host)
                    .expect("Failed to create SMTP transport")
                    .credentials(creds)
                    .build();
                    
                info!("EmailService initialized with SMTP provider ({})", host);
                (EmailProviderType::Smtp, Some(transport), None)
            },
            "ses" => {
                let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
                let config = aws_config::defaults(BehaviorVersion::latest())
                    .region(region_provider)
                    .load()
                    .await;
                let client = SesClient::new(&config);
                
                info!("EmailService initialized with AWS SES provider");
                (EmailProviderType::Ses, None, Some(Arc::new(client)))
            },
            _ => {
                info!("EmailService initialized with MOCK provider (logs only)");
                (EmailProviderType::Mock, None, None)
            }
        };

        Self {
            template_service,
            provider_type,
            smtp_transport,
            ses_client,
            from_email,
        }
    }

    pub async fn send_email(&self, to: &str, subject: &str, html_body: &str) -> Result<(), String> {
        let email_res = Message::builder()
            .from(self.from_email.parse().map_err(|e: lettre::address::AddressError| e.to_string())?)
            .to(to.parse().map_err(|e: lettre::address::AddressError| e.to_string())?)
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(html_body.to_string());

        let email = match email_res {
            Ok(e) => e,
            Err(e) => return Err(format!("Failed to build email: {}", e)),
        };

        match self.provider_type {
            EmailProviderType::Smtp => {
                if let Some(transport) = &self.smtp_transport {
                    // SmtpTransport::send is blocking, might need wrapping in spawn_blocking if high volume
                    match transport.send(&email) {
                        Ok(_) => {
                            info!("Email sent via SMTP to {}", to);
                            Ok(())
                        },
                        Err(e) => {
                            error!("SMTP send error: {}", e);
                            Err(e.to_string())
                        }
                    }
                } else {
                    Err("SMTP transport not initialized".to_string())
                }
            },
            EmailProviderType::Ses => {
                if let Some(client) = &self.ses_client {
                    let body = aws_sdk_sesv2::types::Body::builder()
                        .html(aws_sdk_sesv2::types::Content::builder().data(html_body).build().map_err(|e| e.to_string())?) // Fix: handle build error
                        .build();
                        
                    let message = aws_sdk_sesv2::types::Message::builder()
                        .subject(aws_sdk_sesv2::types::Content::builder().data(subject).build().map_err(|e| e.to_string())?) // Fix: handle build error
                        .body(body)
                        .build();
                        
                    let email_content = aws_sdk_sesv2::types::EmailContent::builder()
                        .simple(message)
                        .build();

                    let dest = aws_sdk_sesv2::types::Destination::builder()
                        .to_addresses(to)
                        .build();

                    match client.send_email()
                        .from_email_address(&self.from_email)
                        .destination(dest)
                        .content(email_content)
                        .send()
                        .await 
                    {
                        Ok(_) => {
                            info!("Email sent via SES to {}", to);
                            Ok(())
                        },
                        Err(e) => {
                            error!("SES send error: {}", e);
                            Err(e.to_string())
                        }
                    }
                } else {
                    Err("SES client not initialized".to_string())
                }
            },
            EmailProviderType::Mock => {
                info!("(Mock) Sending email to: {}", to);
                info!("(Mock) Subject: {}", subject);
                info!("(Mock) Body len: {} chars", html_body.len());
                Ok(())
            }
        }
    }

    pub async fn send_template_email<T>(&self, to: &str, subject: &str, template_name: &str, data: &T) -> Result<(), String>
    where
        T: Serialize,
    {
        let html_body = self.template_service.render(template_name, data)?;
        self.send_email(to, subject, &html_body).await
    }
}
