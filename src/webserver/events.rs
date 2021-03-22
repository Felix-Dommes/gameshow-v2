use actix_web::{web, error, get, HttpRequest, HttpResponse, Responder};
use actix_web::Result as HttpResult;
use actix_session::Session;

use crate::datahandler::DataHandler;
use super::ensure_cookie_consent;


pub fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(event_stream);
}

#[get("/{lobby_id}")]
async fn event_stream(db: web::Data<DataHandler>, session: Session, request: HttpRequest, lobby_id: web::Path<String>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    Ok(HttpResponse::Ok().body(lobby_id.clone())) //TODO implement
}
