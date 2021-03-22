use actix_web::{web, error, get, post, HttpRequest, HttpResponse, Responder};
use actix_web::Result as HttpResult;
use actix_session::Session;
use serde::{Serialize, Deserialize};

use crate::datahandler::DataHandler;
use super::ensure_cookie_consent;


pub fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(set_name);
}


//HttpResponse::NoContent() //use as HttpResponse::Ok() when there is no body

/// Set the current user's name
#[derive(Serialize, Deserialize)]
struct SetNameData
{
    name: String,
}
#[get("/set_name")]
async fn set_name(db: web::Data<DataHandler>, session: Session, request: HttpRequest, query: web::Query<SetNameData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    let uuid = db.create_player(query.name.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
    session.set("uuid", uuid)?;
    
    Ok(HttpResponse::Created().finish())
}

/// Get current user's name (for after reloading page, can be used to check if "logged in")
#[get("/get_name")]
async fn get_name(db: web::Data<DataHandler>, session: Session, request: HttpRequest) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let name = db.get_player_name(uuid).await.map_err(|err| error::ErrorInternalServerError(err))?;
        return Ok(HttpResponse::Ok().body(name));
    }
    else
    {
        return Err(error::ErrorNotFound("Invalid session: UUID not found!"));
    }
}
