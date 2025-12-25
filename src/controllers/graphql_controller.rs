// GraphQL Controller
// Handles GraphQL queries and provides GraphQL Playground

use actix_web::{get, post, web, HttpResponse, Result};
use async_graphql::http;
use crate::graphql::AppSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GraphQLRequest {
    pub query: String,
    #[serde(default)]
    pub variables: Option<serde_json::Value>,
    #[serde(default)]
    pub operation_name: Option<String>,
}

/// GraphQL endpoint handler
#[post("/graphql")]
pub async fn graphql_handler(
    schema: web::Data<AppSchema>,
    req: web::Json<GraphQLRequest>,
) -> Result<HttpResponse> {
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

/// GraphQL Playground UI
#[get("/graphql")]
pub async fn graphql_playground() -> Result<HttpResponse> {
    let source = http::playground_source(http::GraphQLPlaygroundConfig::new("/graphql"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
