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
        .service(get_player_data)
        .service(update_lobby)
        .service(upload_custom_questions)
        .service(kick_player)
        .service(set_player_attributes)
        .service(next_state)
        .service(bet_money)
        .service(attack_player)
        .service(answer_question)
        .service(get_joker);
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
            let name = name.unwrap();
            let lobby_uuid = db.create_lobby(uuid, name.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
            return Ok(HttpResponse::Ok().json( (lobby_uuid, name) ));
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
                let admin_name = lobby.get_admin_name().await;
                //finally do the joining itself
                let joined_name = lobby.join(&uuid, player_name.unwrap()).await;
                if joined_name.is_some()
                {
                    return Ok(HttpResponse::Ok().json(JoinLobbyReturn { admin: admin_name, new_name: joined_name.unwrap() }));
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
            if lobby.leave(&uuid).await
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
async fn get_events(db: web::Data<DataHandler>, request: HttpRequest, params: web::Query<GetEventsData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    let db_lobby = db.get_lobby(params.lobby_id.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
    if db_lobby.is_some()
    {
        let lobby = db_lobby.unwrap();
        return Ok(HttpResponse::Ok().header("Cache-Control", "no-cache").json(lobby.get_events().await));
    }
    else
    {
        return Err(error::ErrorNotFound("Lobby not found: Lobby UUID not in database!"));
    }
}

// Get a lobby's player data
#[derive(Serialize, Deserialize)]
struct GetPlayerDataData
{
    lobby_id: String,
}
#[get("/get_player_data")]
async fn get_player_data(db: web::Data<DataHandler>, request: HttpRequest, params: web::Query<GetPlayerDataData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    let db_lobby = db.get_lobby(params.lobby_id.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
    if db_lobby.is_some()
    {
        let lobby = db_lobby.unwrap();
        return Ok(HttpResponse::Ok().header("Cache-Control", "no-cache").json(lobby.get_player_data().await));
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
                lobby.set_question_set(&params.question_set).await?;
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

// Kick a player from playing in the lobby
#[derive(Serialize, Deserialize)]
struct KickPlayerData
{
    lobby_id: String,
    name: String,
}
#[post("/kick_player")]
async fn kick_player(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Json<KickPlayerData>) -> HttpResult<HttpResponse>
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
                let res = lobby.kick_player(&params.name).await;
                if res
                {
                    return Ok(HttpResponse::NoContent().finish());
                }
                else
                {
                    return Err(error::ErrorNotFound("Player name was not found"));
                }
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

// Set a players attributes
#[derive(Serialize, Deserialize)]
struct SetPlayerAttributesData
{
    lobby_id: String,
    name: String,
    money: i64,
    jokers: usize,
}
#[post("/set_player_attributes")]
async fn set_player_attributes(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Json<SetPlayerAttributesData>) -> HttpResult<HttpResponse>
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
                if params.money < 1
                {
                    return Err(error::ErrorBadRequest("Money must be at least 1!"));
                }
                
                let res = lobby.set_player_attributes(&params.name, params.money, params.jokers).await;
                if res
                {
                    return Ok(HttpResponse::NoContent().finish());
                }
                else
                {
                    return Err(error::ErrorNotFound("Player name was not found"));
                }
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

// Activate (force) next lobby state
#[derive(Serialize, Deserialize)]
struct NextStateData
{
    lobby_id: String,
}
#[get("/next_state")]
async fn next_state(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Query<NextStateData>) -> HttpResult<HttpResponse>
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
                lobby.next_state().await;
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


// A player bets for a question
#[derive(Serialize, Deserialize)]
struct BetMoneyData
{
    lobby_id: String,
    money_bet: i64,
}
#[get("/bet_money")]
async fn bet_money(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Query<BetMoneyData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let db_lobby = db.get_lobby(params.lobby_id.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if db_lobby.is_some()
        {
            let lobby = db_lobby.unwrap();
            let player_money = lobby.get_player_money(&uuid).await;
            if player_money.is_none()
            {
                return Err(error::ErrorNotFound("Player(you) not found"));
            }
            if params.money_bet < 1 || params.money_bet > player_money.unwrap()
            {
                return Err(error::ErrorBadRequest("Money_bet is invalid (< 1 or > player money)!"));
            }
            let res = lobby.bet(&uuid, params.money_bet).await;
            if res
            {
                return Ok(HttpResponse::NoContent().finish());
            }
            else
            {
                return Err(error::ErrorNotAcceptable("Game lobby is in wrong state!"));
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

// A player selects a player to attack
#[derive(Serialize, Deserialize)]
struct AttackPlayerData
{
    lobby_id: String,
    vs_player: String,
}
#[get("/attack_player")]
async fn attack_player(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Query<AttackPlayerData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let db_lobby = db.get_lobby(params.lobby_id.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if db_lobby.is_some()
        {
            let lobby = db_lobby.unwrap();
            let (player_name, valid_vs_player) = join!(lobby.get_player_name(&uuid), lobby.is_valid_vs_player(&params.vs_player));
            if player_name.is_none()
            {
                return Err(error::ErrorNotFound("Player(you) not found"));
            }
            if !valid_vs_player || params.vs_player == player_name.unwrap()
            {
                return Err(error::ErrorBadRequest("Vs_player ist invalid!"));
            }
            let res = lobby.attack(&uuid, &params.vs_player).await;
            if res
            {
                return Ok(HttpResponse::NoContent().finish());
            }
            else
            {
                return Err(error::ErrorNotAcceptable("Game lobby is in wrong state!"));
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

// A player answers a question
#[derive(Serialize, Deserialize)]
struct AnswerQuestionData
{
    lobby_id: String,
    answer: usize,
}
#[get("/answer_question")]
async fn answer_question(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Query<AnswerQuestionData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let db_lobby = db.get_lobby(params.lobby_id.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if db_lobby.is_some()
        {
            let lobby = db_lobby.unwrap();
            if !lobby.is_joined(&uuid).await
            {
                return Err(error::ErrorNotFound("Player(you) not found"));
            }
            if params.answer < 1
            {
                return Err(error::ErrorBadRequest("Answer is invalid (< 1)!"));
            }
            let res = lobby.answer(&uuid, params.answer).await;
            if res
            {
                return Ok(HttpResponse::NoContent().finish());
            }
            else
            {
                return Err(error::ErrorNotAcceptable("Game lobby is in wrong state!"));
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

// A player retrieves wrong answers per joker
#[derive(Serialize, Deserialize)]
struct GetJokerData
{
    lobby_id: String,
}
#[get("/get_joker")]
async fn get_joker(db: web::Data<DataHandler>, session: Session, request: HttpRequest, params: web::Query<GetJokerData>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    if let Some(uuid) = session.get::<String>("uuid")?
    {
        let db_lobby = db.get_lobby(params.lobby_id.clone()).await.map_err(|err| error::ErrorInternalServerError(err))?;
        if db_lobby.is_some()
        {
            let lobby = db_lobby.unwrap();
            let player_jokers = lobby.get_player_jokers(&uuid).await;
            if player_jokers.is_none()
            {
                return Err(error::ErrorNotFound("Player(you) not found"));
            }
            if player_jokers.unwrap() < 1
            {
                return Err(error::ErrorBadRequest("You have no jokers!"));
            }
            let res = lobby.get_joker(&uuid).await;
            if res.is_some()
            {
                return Ok(HttpResponse::Ok().json(res.unwrap()));
            }
            else
            {
                return Err(error::ErrorNotAcceptable("Game lobby is in wrong state!"));
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
