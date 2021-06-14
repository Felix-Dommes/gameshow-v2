use actix_web::{web, error, get, post, HttpRequest, HttpResponse};
use actix_web::Result as HttpResult;
use actix_session::Session;
use serde::{Serialize, Deserialize};
use futures::join;

use crate::datahandler::DataHandler;
use crate::game;
use super::ensure_cookie_consent;

mod lobby;


pub fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(set_name)
        .service(get_name)
        .service(get_question_sets);
    lobby::config(cfg);
}


// Set the current user's name; if not logged in, create new user
#[derive(Serialize, Deserialize)]
struct SetNameData
{
    name: String,
}
#[get("/set_name")]
async fn set_name(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Query<SetNameData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    //check if user exists before and possible change name only
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let was_there = db.set_player_name(uuid, params.name.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if was_there
        {
            Ok(HttpResponse::Ok().json(params.name.clone()))
        }
        else
        {
            Ok(HttpResponse::Created().json(params.name.clone()))
        }
    }
    else
    {
        let uuid = db.create_player(params.name.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        session.set("uuid", uuid)?;
        Ok(HttpResponse::Created().json(params.name.clone()))
    }
}

// Get current user's name (for after reloading page, can be used to check if "logged in")
#[get("/get_name")]
async fn get_name(db: web::Data<DataHandler>, session: Session, request: HttpRequest) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let name = db.get_player_name(uuid).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if name.is_some()
        {
            return Ok(HttpResponse::Ok().json(name.unwrap()));
        }
        else
        {
            return Err(error::ErrorNotFound("Invalid UUID: Player UUID not found in database!"));
        }
    }
    else
    {
        return Err(error::ErrorNotFound("Invalid session: No player UUID!"));
    }
}

// Get a list of question sets
#[get("/get_question_sets")]
async fn get_question_sets(db: web::Data<DataHandler>) -> HttpResult<HttpResponse>
{
    let list = db.get_question_sets().await.map_err(|err| error::ErrorInternalServerError(err))?;
    let question_sets:Vec<String> = list.iter().map(|x| x.0.clone()).collect();
    Ok(HttpResponse::Ok().json(question_sets))
}
