use std::env;
use reqwest::{Client, header};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY not set");
    println!("Loaded Key: {:?}", key);
    println!("Key Bytes: {:?}", key.as_bytes());
    
    let trimmed = key.trim();
    println!("Trimmed Key: {:?}", trimmed);
    println!("Trimmed Bytes: {:?}", trimmed.as_bytes());
    
    let auth_value = format!("Bearer {}", trimmed);
    println!("Auth Header Value: {:?}", auth_value);
    
    // Test direct header creation (http crate)
    match header::HeaderValue::from_str(&auth_value) {
        Ok(_) => println!("HeaderValue::from_str passed"),
        Err(e) => println!("HeaderValue::from_str FAILED: {:?}", e),
    }

    // Test reqwest builder
    let client = Client::new();
    let res = client.post("https://api.stripe.com/v1/payment_intents")
        .bearer_auth(trimmed)
        .build();
        
    match res {
        Ok(req) => {
            println!("Reqwest build passed");
            println!("Headers: {:?}", req.headers());
        },
        Err(e) => println!("Reqwest build FAILED: {:?}", e),
    }
}
