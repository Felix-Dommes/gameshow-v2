use actix_web::{web, get, HttpRequest, HttpResponse, Responder};
use actix_session::Session;

use crate::datahandler::DataHandler;
use super::{has_cookie_consent, no_consent_error};


pub fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(event_stream);
}

#[get("/{lobby_id}")]
async fn event_stream(db: web::Data<DataHandler>, session: Session, request: HttpRequest, lobby_id: web::Path<String>) -> impl Responder
{
    if !has_cookie_consent(&request)
    {
        return no_consent_error();
    }
    
    HttpResponse::Ok().body(lobby_id.clone()) //TODO implement
}
