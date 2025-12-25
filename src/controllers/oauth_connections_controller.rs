use actix_web:: {web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OAuthConnectionsQuery {
    user_id: i32,
}

/// Get user's connected OAuth providers
pub async fn get_connections(
    query: web::Query<OAuthConnectionsQuery>,
    pool: web::Data<crate::models::MySQLPool>,
) -> Result<HttpResponse, actix_web::Error> {
    use crate::schema::oauth_connections;
    use diesel::prelude::*;
    
    let conn = pool.get()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    let connections: Vec<(i32, String, String)> = oauth_connections::table
        .inner_join(crate::schema::oauth_providers::table)
        .filter(oauth_connections::user_id.eq(query.user_id))
        .select((
            oauth_connections::id,
            crate::schema::oauth_providers::name,
            oauth_connections::provider_user_id,
        ))
        .load(&conn)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(connections))
}
