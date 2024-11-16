use ntex::web;

use crate::api::{
    index, create_video, list_videos, get_video,
    update_video, delete_video,
};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(list_videos)
        .service(create_video)
        .service(get_video)
        .service(update_video)
        .service(delete_video);
}
