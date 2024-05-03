
use actix_web::web::{self, ServiceConfig};
use actix_files::Files;

use crate::AppState;

use handlers::{login,logout,index, auth};
use crate::common::handlers::default_handler;

mod handlers;
mod pages;


pub fn configure(app_state: AppState) -> Box<dyn Fn(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(app_state.clone()))
        .service(web::resource("/login").route(web::get().to(login)))
        .service(web::resource("/auth").route(web::get().to(auth)))
        .service(web::resource("/logout").route(web::get().to(logout)))
        .service(web::resource("/").route(web::get().to(index)))
        .service(Files::new("/static", "static/app").show_files_listing())
        .default_service(web::to(default_handler));
    })
}