use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use futures::Stream;
use actix_web::{web, error, get, HttpRequest, HttpResponse, Responder};
use actix_web::Result as HttpResult;
use actix_session::Session;
use tokio::sync::broadcast;
use tokio::time;

use crate::datahandler::DataHandler;
use crate::game::Event;
use super::ensure_cookie_consent;

const PING_INTERVAL:u64 = 10; //interval to ping clients in seconds


pub fn config(cfg: &mut web::ServiceConfig)
{
    cfg.service(event_stream);
}

#[get("/{lobby_id}")]
async fn event_stream(db: web::Data<DataHandler>, session: Session, request: HttpRequest, lobby_id: web::Path<String>) -> HttpResult<HttpResponse>
{
    ensure_cookie_consent(&request)?;
    
    let db_lobby = db.get_lobby(lobby_id.into_inner()).await.map_err(|err| error::ErrorInternalServerError(err))?;
    if db_lobby.is_some()
    {
        let lobby = db_lobby.unwrap();
        let client = EventStreamClient::new(lobby.subsribe_events().await);
        
        let mut resp = HttpResponse::Ok();
        resp.header("Content-Type", "text/event-stream")
            .header("Cache-Control", "no-cache");
        Ok(resp.streaming(client))
    }
    else
    {
        return Err(error::ErrorNotFound("Lobby not found: Lobby UUID not in database!"));
    }
}


struct EventStreamClient
{
    receiver: broadcast::Receiver<Event>,
    pinger: time::Interval,
}

impl EventStreamClient
{
    pub fn new(event_source: broadcast::Receiver<Event>) -> Self
    {
        EventStreamClient {
            receiver: event_source,
            pinger: time::interval(Duration::from_secs(PING_INTERVAL)),
        }
    }
    
    fn ping() -> web::Bytes
    {
        web::Bytes::from("event: ping\ndata: \"ping\"\n\n")
    }
    
    fn event_to_bytes(event: Event) -> web::Bytes
    {
        let data = serde_json::to_string(&event).unwrap();
        web::Bytes::from(format!("event: {}\nid: {}\ndata: {}\n\n", event.event_name, event.id, data))
    }
}

impl Stream for EventStreamClient
{
    type Item = Result<web::Bytes, error::Error>;
    
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
    {
        if let Poll::Ready(_) = self.pinger.poll_tick(cx)
        {
            return Poll::Ready(Some(Ok(EventStreamClient::ping())));
        }
        else
        {
            match self.receiver.try_recv()
            {
                Ok(content) => Poll::Ready(Some(Ok(EventStreamClient::event_to_bytes(content)))),
                Err(broadcast::error::TryRecvError::Closed) => Poll::Ready(None),
                Err(broadcast::error::TryRecvError::Lagged(_)) => Poll::Ready(None), //close connection when messages were lost
                Err(broadcast::error::TryRecvError::Empty) => Poll::Pending,
            }
        }
    }
}
