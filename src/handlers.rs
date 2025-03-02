use actix_web::{Responder, HttpResponse, post};
use actix_web::web::{Json, Data};
use std::sync::Mutex;
use crate::api_models::{CreateGroup, CreateUser, JoinGroup, LeaveGroup, MakeUserAdmin, MakeUserNonadmin, Response};
use crate::db::{Db};
use serde::Serialize;

#[derive(Serialize)]
struct CreatedObjectId {
    pub id: i32,
}

#[post("/createuser")]
pub async fn create_user(req: Json<CreateUser>, db: Data<Mutex<Db>>) -> impl Responder {
    let user_name = req.into_inner().name;
    let mut db = db.lock().unwrap();
    let resp = Response::<CreatedObjectId>::from(
        db.create_user(user_name)
            .map(|id| CreatedObjectId { id })
    );
    HttpResponse::Ok().json(&resp)
}

#[post("/creategroup")]
pub async fn create_group(req: Json<CreateGroup>, db: Data<Mutex<Db>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let resp = Response::<CreatedObjectId>::from(
        db.create_group(req.initiator_id, req.into_inner().name)
            .map(|id| CreatedObjectId { id })
    );
    HttpResponse::Ok().json(&resp)
}

#[post("/joingroup")]
pub async fn join_group(req: Json<JoinGroup>, db: Data<Mutex<Db>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let resp = Response::<()>::from(db.join_group(req.user_id, req.group_id));
    HttpResponse::Ok().json(&resp)
}

#[post("/makeuseradmin")]
pub async fn make_user_admin(req: Json<MakeUserAdmin>, db: Data<Mutex<Db>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let resp = Response::<()>::from(db.make_user_admin(req.initiator_id, req.user_id, req.group_id));
    HttpResponse::Ok().json(&resp)
}

#[post("/makeusernonadmin")]
pub async fn make_user_nonadmin(req: Json<MakeUserNonadmin>, db: Data<Mutex<Db>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let resp = Response::<()>::from(db.make_user_nonadmin(req.user_id, req.group_id));
    HttpResponse::Ok().json(&resp)
}

#[post("/leavegroup")]
pub async fn leave_group(req: Json<LeaveGroup>, db: Data<Mutex<Db>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    let resp = Response::<()>::from(db.leave_group(req.user_id, req.group_id));
    HttpResponse::Ok().json(&resp)
}