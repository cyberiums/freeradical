use actix_web::{web, Scope};
use super::Router;

use crate::controllers::user_controllers::*;

pub struct UserRouter;

impl Router for UserRouter {
    fn new() -> Scope {
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("", web::get().to(list_users))
            .route("/login", web::post().to(login))
            .route("/logout", web::delete().to(logout))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::delete().to(delete_user))
            .route("/{id}/2fa/setup", web::get().to(setup_2fa))
            .route("/{id}/2fa/enable", web::post().to(enable_2fa))
            
    }
}
