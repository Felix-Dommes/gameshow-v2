use actix_web::{web, error, get, post, HttpRequest, HttpResponse, Responder};
use actix_web::Result as HttpResult;
use actix_session::Session;
use serde::{Serialize, Deserialize};

use crate::datahandler::DataHandler;
use super::ensure_cookie_consent;


pub fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(set_name)
        .service(get_name)
        .service(create_lobby)
        .service(join_lobby)
        .service(leave_lobby)
        .service(get_events);
}


//HttpResponse::NoContent() //use as HttpResponse::Ok() when there is no body

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
        let was_there = db.set_player_name(uuid.clone(), params.name.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if was_there
        {
            Ok(HttpResponse::Ok().json(uuid))
        }
        else
        {
            Ok(HttpResponse::Created().json(uuid))
        }
    }
    else
    {
        let uuid = db.create_player(params.name.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        session.set("uuid", uuid.clone())?;
        Ok(HttpResponse::Created().json(uuid))
    }
}

// Get current user's name (for after reloading page, can be used to check if "logged in")
#[get("/get_name")]
async fn get_name(db: web::Data<DataHandler>, session: Session, request: HttpRequest) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let name = db.get_player_name(uuid.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if name.is_some()
        {
            return Ok(HttpResponse::Ok().json((name.unwrap(), uuid)));
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
                return Ok(HttpResponse::Ok().finish())
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
