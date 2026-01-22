use std::env;
use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Serialize)]
struct StripeCreateIntentRequest {
    amount: i64,
    currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
struct StripePaymentIntent {
    id: String,
    amount: i64,
    currency: String,
    status: String,
    client_secret: Option<String>,
    #[serde(default)]
    metadata: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY not set");
    let api_key = key.trim().to_string();
    
    println!("Loaded Key Bytes: {:?}", api_key.as_bytes());

    let client = Client::new();
    let url = "https://api.stripe.com/v1/payment_intents";
    
    let mut metadata = HashMap::new();
    metadata.insert("subscription_id".to_string(), "123".to_string());

    let stripe_request = StripeCreateIntentRequest {
        amount: 2500, // 25.00
        currency: "usd".to_string(),
        metadata: Some(metadata),
    };
    
    let auth_header = format!("Bearer {}", api_key);
    println!("Auth Payload: {}", auth_header);

    println!("--- TEST 1: Header ONLY ---");
    let rb1 = client.post(url)
        .header("Authorization", &auth_header);
    match rb1.build() {
        Ok(_) => println!("TEST 1 PASSED (Build only)"),
        Err(e) => println!("TEST 1 FAILED: {:?}", e),
    }

    println!("--- TEST 2: Form ONLY ---");
    let rb2 = client.post(url)
        .form(&stripe_request);
    match rb2.build() {
        Ok(_) => println!("TEST 2 PASSED (Build only)"),
        Err(e) => println!("TEST 2 FAILED: {:?}", e),
    }

    println!("--- TEST 3: Header + Form ---");
    let rb3 = client.post(url)
        .header("Authorization", &auth_header)
        .form(&stripe_request);
    match rb3.build() {
        Ok(_) => println!("TEST 3 PASSED (Build only)"),
        Err(e) => println!("TEST 3 FAILED: {:?}", e),
    }

    println!("--- TEST 4: Flattened Map ---");
    let mut params = HashMap::new();
    params.insert("amount", "2500");
    params.insert("currency", "usd");
    params.insert("metadata[subscription_id]", "123");
    
    let rb4 = client.post(url)
        .header("Authorization", &auth_header)
        .form(&params);
        
    match rb4.build() {
        Ok(_) => println!("TEST 4 PASSED (Build only)"),
        Err(e) => println!("TEST 4 FAILED: {:?}", e),
    }
}
