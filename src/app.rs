use crate::api;
use ntex::web;

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(api::index);
}
