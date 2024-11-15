use crate::api;
use ntex::web;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/").route("", web::get().to(api::index)));
}
