use actix_web::web::{self, ServiceConfig};
use actix_files::Files;

use handlers::{health,index};

pub mod handlers;
pub mod pages;


pub fn configure() -> Box<dyn Fn(&mut ServiceConfig)> {
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/health").route(web::get().to(health)))
        .service(Files::new("/static", "static/admin").show_files_listing());
    })
}