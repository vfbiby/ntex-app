use ntex::web::{self, types::{Json, Path, Query}, HttpResponse, Responder};
use crate::services::video_service::VideoService;
use crate::api::{CreateVideoRequest, UpdateVideoRequest};
use crate::db::VideoQuery;
use crate::error::AppResult;
use std::sync::Arc;

#[derive(Clone)]
pub struct VideoController {
    service: VideoService,
}

impl VideoController {
    pub fn new(service: VideoService) -> Self {
        Self { service }
    }

    pub async fn create_video(&self, req: Json<CreateVideoRequest>) -> AppResult<impl Responder> {
        let video = self.service.create_video(req.into_inner()).await?;
        Ok(HttpResponse::Created().json(&video))
    }

    pub async fn get_video(&self, id: Path<i32>) -> AppResult<impl Responder> {
        let video = self.service.get_video(id.into_inner()).await?;
        Ok(HttpResponse::Ok().json(&video))
    }

    pub async fn update_video(&self, id: Path<i32>, req: Json<UpdateVideoRequest>) -> AppResult<impl Responder> {
        let video = self.service.update_video(id.into_inner(), req.into_inner()).await?;
        Ok(HttpResponse::Ok().json(&video))
    }

    pub async fn delete_video(&self, id: Path<i32>) -> AppResult<impl Responder> {
        self.service.delete_video(id.into_inner()).await?;
        Ok(HttpResponse::NoContent().finish())
    }

    pub async fn list_videos(&self, query: Query<VideoQuery>) -> AppResult<impl Responder> {
        let videos = self.service.list_videos(query.into_inner()).await?;
        Ok(HttpResponse::Ok().json(&videos))
    }
}

pub fn config(cfg: &mut web::ServiceConfig, controller: VideoController) {
    let controller = Arc::new(controller);
    let c1 = controller.clone();
    let c2 = controller.clone();
    let c3 = controller.clone();
    let c4 = controller.clone();
    let c5 = controller.clone();
    
    cfg.service(
        web::scope("/api/v1/videos")
            .route("", web::post().to(move |req: Json<CreateVideoRequest>| {
                let ctrl = Arc::clone(&c1);
                async move { ctrl.create_video(req).await }
            }))
            .route("", web::get().to(move |query: Query<VideoQuery>| {
                let ctrl = Arc::clone(&c2);
                async move { ctrl.list_videos(query).await }
            }))
            .route("/{id}", web::get().to(move |id: Path<i32>| {
                let ctrl = Arc::clone(&c3);
                async move { ctrl.get_video(id).await }
            }))
            .route("/{id}", web::put().to(move |id: Path<i32>, req: Json<UpdateVideoRequest>| {
                let ctrl = Arc::clone(&c4);
                async move { ctrl.update_video(id, req).await }
            }))
            .route("/{id}", web::delete().to(move |id: Path<i32>| {
                let ctrl = Arc::clone(&c5);
                async move { ctrl.delete_video(id).await }
            }))
    );
}
