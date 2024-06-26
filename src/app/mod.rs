use actix_files::Files;
use actix_web::web::{self, ServiceConfig};

use crate::common::handlers::default_handler;
use handlers::{auth, index, login, logout};

mod handlers;
mod pages;

pub fn configure() -> Box<dyn Fn(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.service(web::resource("/login").route(web::get().to(login)))
            .service(web::resource("/auth").route(web::get().to(auth)))
            .service(web::resource("/logout").route(web::get().to(logout)))
            .service(web::resource("/").route(web::get().to(index)))
            .service(Files::new("/static", "static/app").show_files_listing())
            .default_service(web::to(default_handler));
    })
}
