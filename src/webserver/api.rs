use actix_web::{web, get, post, HttpRequest, HttpResponse, Responder};
use actix_session::Session;
use serde::{Serialize, Deserialize};

use crate::datahandler::DataHandler;
use super::{has_cookie_consent, no_consent_error};


pub fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(set_name);
}


/// Set the current user's name
#[derive(Serialize, Deserialize)]
struct SetNameData
{
    name: String,
}
#[get("/set_name")]
async fn set_name(db: web::Data<DataHandler>, session: Session, request: HttpRequest, query: web::Query<SetNameData>) -> impl Responder
{
    HttpResponse::Ok().finish()
}
