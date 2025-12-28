use actix_web:: {web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OAuthConnectionsQuery {
    user_id: i32,
}

/// Get user's connected OAuth providers
pub async fn get_connections(
    query: web::Query<OAuthConnectionsQuery>,
    pool: web::Data<crate::models::DatabasePool>,
) -> Result<HttpResponse, actix_web::Error> {
    use crate::schema::user_oauth_connections;
    use diesel::prelude::*;
    
    let mut conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let connections: Vec<(i32, String, String)> = user_oauth_connections::table
        .inner_join(crate::schema::oauth_providers::table)
        .filter(user_oauth_connections::user_id.eq(query.user_id))
        .select((
            user_oauth_connections::id,
            crate::schema::oauth_providers::name,
            user_oauth_connections::provider_user_id,
        ))
        .load(&mut conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(connections))
}
