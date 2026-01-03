// GraphQL Controller
// Handles GraphQL queries and provides GraphQL Playground

use actix_web::{get, post, web, HttpResponse, Result};
use crate::graphql::AppSchema;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GraphQLRequest {
    pub query: String,
    #[serde(default)]
    pub variables: Option<serde_json::Value>,
    #[serde(default)]
    pub operation_name: Option<String>,
}

/// GraphQL endpoint handler with JWT authentication
#[post("/graphql")]
pub async fn graphql_handler(
    schema: web::Data<AppSchema>,
    req: web::Json<GraphQLRequest>,
    http_req: actix_web::HttpRequest,
) -> Result<HttpResponse> {
    // Extract and verify JWT token from Authorization header
    let auth_header = http_req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));
    
    // Require authentication for GraphQL queries
    if auth_header.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "errors": [{
                "message": "Authentication required. Please provide a valid JWT token in the Authorization header.",
                "extensions": {
                    "code": "UNAUTHENTICATED"
                }
            }]
        })));
    }
    
    // Verify JWT token
    if let Some(token) = auth_header {
        // Validate token
        use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
        use std::env;
        
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        let key = DecodingKey::from_secret(secret.as_bytes());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        
        match decode::<serde_json::Value>(token, &key, &validation) {
            Ok(_) => {
                // Token is valid, proceed
            }
            Err(_) => {
                return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                    "errors": [{
                        "message": "Invalid or expired JWT token",
                        "extensions": {
                            "code": "UNAUTHENTICATED"
                        }
                    }]
                })));
            }
        }
    } else {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "errors": [{
                "message": "Authentication required",
                "extensions": {
                    "code": "UNAUTHENTICATED"
                }
            }]
        })));
    }
    
    let mut request = async_graphql::Request::new(&req.query);
    
    if let Some(variables) = &req.variables {
        if let Ok(vars) = serde_json::from_value(variables.clone()) {
            request = request.variables(vars);
        }
    }
    
    if let Some(operation_name) = &req.operation_name {
        request = request.operation_name(operation_name);
    }
    
    let response = schema.execute(request).await;
    Ok(HttpResponse::Ok().json(response))
}

/// GraphQL Playground UI with FreeRadical branding
#[get("/graphql")]
pub async fn graphql_playground() -> Result<HttpResponse> {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GraphQL Playground - FreeRadical CMS</title>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;900&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/css/index.css" />
    <style>
        body {
            margin: 0;
            padding: 0;
            font-family: 'Inter', sans-serif;
            background-color: #0F172A !important;
        }
        #root {
            height: 100vh;
        }
        /* Custom header */
        .header-container {
            background: linear-gradient(135deg, #1E293B 0%, #0F172A 100%);
            border-bottom: 3px solid #F97316;
            padding: 16px 24px;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .header-logo {
            display: flex;
            align-items: center;
            gap: 12px;
        }
        .header-logo img {
            width: 40px;
            height: 40px;
        }
        .header-title {
            font-size: 24px;
            font-weight: 900;
            color: white;
            letter-spacing: -0.04em;
        }
        .header-subtitle {
            font-family: 'JetBrains Mono', monospace;
            font-size: 11px;
            font-weight: 700;
            color: #F97316;
            letter-spacing: 0.1em;
        }
        .header-actions {
            display: flex;
            gap: 12px;
        }
        .header-btn {
            padding: 8px 16px;
            border-radius: 8px;
            font-family: 'JetBrains Mono', monospace;
            font-size: 11px;
            font-weight: 700;
            letter-spacing: 0.05em;
            text-decoration: none;
            transition: all 0.2s;
            background-color: #1E293B;
            color: #06B6D4;
            border: 2px solid #334155;
        }
        .header-btn:hover {
            border-color: #06B6D4;
        }
    </style>
</head>
<body>
    <div class="header-container">
        <div class="header-logo">
            <img src="/static/logo.svg" alt="FreeRadical">
            <div>
                <div class="header-title">FreeRadical</div>
                <div class="header-subtitle">GRAPHQL PLAYGROUND</div>
            </div>
        </div>
        <div class="header-actions">
            <a href="/static/api.html" class="header-btn">API DOCS</a>
            <a href="/" class="header-btn">HOME</a>
        </div>
    </div>
    <div id="root"></div>
    <script src="https://cdn.jsdelivr.net/npm/graphql-playground-react/build/static/js/middleware.js"></script>
    <script>
        window.addEventListener('load', function (event) {
            GraphQLPlayground.init(document.getElementById('root'), {
                endpoint: '/graphql',
                settings: {
                    'editor.theme': 'dark',
                    'editor.cursorShape': 'block',
                    'editor.reuseHeaders': true,
                    'tracing.hideTracingResponse': true,
                    'queryPlan.hideQueryPlanResponse': true,
                    'editor.fontSize': 14,
                    'editor.fontFamily': '"JetBrains Mono", monospace',
                },
                tabs: [{
                    endpoint: '/graphql',
                    query: '# Welcome to FreeRadical CMS GraphQL Playground\n# Type your queries here\n\n{\n  __schema {\n    types {\n      name\n    }\n  }\n}'
                }]
            })
        })
    </script>
</body>
</html>"#;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
