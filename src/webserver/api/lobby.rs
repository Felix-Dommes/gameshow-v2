use actix_web::{web, error, get, post, HttpRequest, HttpResponse};
use actix_web::Result as HttpResult;
use actix_session::Session;
use serde::{Serialize, Deserialize};
use futures::join;

use crate::datahandler::DataHandler;
use crate::game;
use super::ensure_cookie_consent;


pub fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(create_lobby)
        .service(join_lobby)
        .service(leave_lobby)
        .service(get_events)
        .service(update_lobby)
        .service(upload_custom_questions);
}


// Create a new lobby without joining
#[get("/create_lobby")]
async fn create_lobby(db: web::Data<DataHandler>, session: Session, request: HttpRequest) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let name = db.get_player_name(uuid.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if name.is_some()
        {
            let lobby_uuid = db.create_lobby(uuid, name.unwrap()).await.map_err(|err| error::ErrorInternalServerError(err))?;
            return Ok(HttpResponse::Ok().json(lobby_uuid));
        }
        else
        {
            return Err(error::ErrorNotFound("Invalid UUID: Player UUID not found in database!"));
        }
    }
    else
    {
        return Err(error::ErrorUnauthorized("Invalid session: No player UUID!"));
    }
}

// Join an existing lobby
#[derive(Serialize, Deserialize)]
struct JoinLobbyData
{
    uuid: String,
}
#[derive(Serialize, Deserialize)]
struct JoinLobbyReturn
{
    admin: String,
    new_name: String,
}
#[get("/join_lobby")]
async fn join_lobby(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Query<JoinLobbyData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let player_name = db.get_player_name(uuid.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if player_name.is_some()
        {
            let db_lobby = db.get_lobby(params.uuid.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
            if db_lobby.is_some()
            {
                let lobby = db_lobby.unwrap();
                let admin_uuid = lobby.get_admin_uuid().await;
                //finally do the joining itself
                let joined = lobby.join(uuid, player_name.unwrap()).await;
                if joined.0
                {
                    return Ok(HttpResponse::Ok().json(JoinLobbyReturn { admin: admin_uuid, new_name: joined.1.unwrap() }));
                }
                else
                {
                    return Err(error::ErrorForbidden("Could not join lobby: Lobby is closed!"));
                }
            }
            else
            {
                return Err(error::ErrorNotFound("Lobby not found: Lobby UUID not in database!"));
            }
        }
        else
        {
            return Err(error::ErrorBadRequest("Invalid UUID: Player UUID not found in database!"));
        }
    }
    else
    {
        return Err(error::ErrorUnauthorized("Invalid session: No player UUID!"));
    }
}

// Leave a lobby
#[derive(Serialize, Deserialize)]
struct LeaveLobbyData
{
    uuid: String,
}
#[get("/leave_lobby")]
async fn leave_lobby(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Query<LeaveLobbyData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let db_lobby = db.get_lobby(params.uuid.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if db_lobby.is_some()
        {
            let lobby = db_lobby.unwrap();
            if lobby.leave(uuid).await
            {
                return Ok(HttpResponse::NoContent().finish())
            }
            else
            {
                return Err(error::ErrorNotFound("You did not join before!"));
            }
        }
        else
        {
            return Err(error::ErrorNotFound("Lobby not found: Lobby UUID not in database!"));
        }
    }
    else
    {
        return Err(error::ErrorUnauthorized("Invalid session: No player UUID!"));
    }
}

// Get a lobby's events
#[derive(Serialize, Deserialize)]
struct GetEventsData
{
    lobby_id: String,
}
#[get("/get_events")]
async fn get_events(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Query<GetEventsData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    let db_lobby = db.get_lobby(params.lobby_id.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
    if db_lobby.is_some()
    {
        let lobby = db_lobby.unwrap();
        return Ok(HttpResponse::Ok().json(lobby.get_events().await));
    }
    else
    {
        return Err(error::ErrorNotFound("Lobby not found: Lobby UUID not in database!"));
    }
}

// Update the lobby preferences
#[derive(Serialize, Deserialize)]
struct UpdateLobbyData
{
    lobby_id: String,
    open: bool,
    initial_money: i64,
    initial_jokers: usize,
    normal_q_money: i64,
    estimation_q_money: i64,
    question_set: String,
}
#[post("/update_lobby")]
async fn update_lobby(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Json<UpdateLobbyData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let db_lobby = db.get_lobby(params.lobby_id.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if db_lobby.is_some()
        {
            let lobby = db_lobby.unwrap();
            if lobby.get_admin_uuid().await == uuid
            {
                join!(lobby.set_open(params.open),
                    lobby.update_preferences(params.initial_money, params.initial_jokers, params.normal_q_money, params.estimation_q_money));
                lobby.set_question_set(params.question_set.clone()).await?;
                return Ok(HttpResponse::NoContent().finish());
            }
            else
            {
                return Err(error::ErrorUnauthorized("You are not the lobby admin!"));
            }
        }
        else
        {
            return Err(error::ErrorNotFound("Lobby not found: Lobby UUID not in database!"));
        }
    }
    else
    {
        return Err(error::ErrorUnauthorized("Invalid session: No player UUID!"));
    }
}

// Upload custom questions to a lobby
#[derive(Serialize, Deserialize)]
struct UploadCustomQuestionsData
{
    lobby_id: String,
    questions: Vec<game::Question>,
}
#[post("/upload_custom_questions")]
async fn upload_custom_questions(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Json<UploadCustomQuestionsData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let db_lobby = db.get_lobby(params.lobby_id.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if db_lobby.is_some()
        {
            let lobby = db_lobby.unwrap();
            if lobby.get_admin_uuid().await == uuid
            {
                lobby.set_questions(params.questions.clone()).await?;
                return Ok(HttpResponse::NoContent().finish());
            }
            else
            {
                return Err(error::ErrorUnauthorized("You are not the lobby admin!"));
            }
        }
        else
        {
            return Err(error::ErrorNotFound("Lobby not found: Lobby UUID not in database!"));
        }
    }
    else
    {
        return Err(error::ErrorUnauthorized("Invalid session: No player UUID!"));
    }
}
