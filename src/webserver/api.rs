use actix_session::Session;
use actix_web::Result as HttpResult;
use actix_web::{error, get, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::env;

use super::ensure_cookie_consent;
use crate::datahandler::DataHandler;

mod lobby;

const MAX_NICKNAME_LENGTH: usize = 25;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(set_name)
        .service(get_name)
        .service(get_question_sets);
    lobby::config(cfg);
}

// Set the current user's name; if not logged in, create new user
#[derive(Serialize, Deserialize)]
struct SetNameData {
    name: String,
}
#[get("/set_name")]
async fn set_name(
    db: web::Data<DataHandler>,
    session: Session,
    request: HttpRequest,
    params: web::Query<SetNameData>,
) -> HttpResult<HttpResponse> {
    ensure_cookie_consent(&request)?;

    //get maximum name length
    let max_nickname_length = env::var("MAX_NICKNAME_LENGTH")
        .unwrap_or_default()
        .parse()
        .unwrap_or(MAX_NICKNAME_LENGTH);
    if params.name.len() > max_nickname_length
    //use params.name.chars().count() ?
    {
        return Err(error::ErrorBadRequest(format!(
            "Name is too long! Maximum length is {}!",
            max_nickname_length
        )));
    }

    //check if user exists before and possible change name only
    if let Some(uuid) = session.get::<String>("uuid")? {
        let was_there = db
            .set_player_name(uuid, params.name.clone())
            .await
            .map_err(error::ErrorInternalServerError)?;
        if was_there {
            Ok(HttpResponse::Ok().json(params.name.clone()))
        } else {
            Ok(HttpResponse::Created().json(params.name.clone()))
        }
    } else {
        let uuid = db
            .create_player(params.name.clone())
            .await
            .map_err(error::ErrorInternalServerError)?;
        session.set("uuid", uuid)?;
        Ok(HttpResponse::Created().json(params.name.clone()))
    }
}

// Get current user's name (for after reloading page, can be used to check if "logged in")
#[get("/get_name")]
async fn get_name(
    db: web::Data<DataHandler>,
    session: Session,
    request: HttpRequest,
) -> HttpResult<HttpResponse> {
    ensure_cookie_consent(&request)?;

    if let Some(uuid) = session.get::<String>("uuid")? {
        let name = db
            .get_player_name(uuid)
            .await
            .map_err(error::ErrorInternalServerError)?;
        if name.is_some() {
            Ok(HttpResponse::Ok().json(name.unwrap()))
        } else {
            Err(error::ErrorNotFound(
                "Invalid UUID: Player UUID not found in database!",
            ))
        }
    } else {
        Err(error::ErrorNotFound("Invalid session: No player UUID!"))
    }
}

// Get a list of question sets
#[get("/get_question_sets")]
async fn get_question_sets(db: web::Data<DataHandler>) -> HttpResult<HttpResponse> {
    let list = db
        .get_question_sets()
        .await
        .map_err(error::ErrorInternalServerError)?;
    let question_sets: Vec<String> = list.iter().map(|x| x.0.clone()).collect();
    Ok(HttpResponse::Ok().json(question_sets))
}
