#[allow(unused_imports)]
use actix_web::{web, error, middleware, App, HttpServer, HttpRequest, HttpMessage, HttpResponse};
use actix_web::Result as HttpResult;
use actix_session::CookieSession;

mod api;
mod events;

use crate::datahandler::DataHandler;


pub async fn startup(db: DataHandler) -> std::io::Result<()>
{
    let app_db = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            //shared data
            .app_data(app_db.clone())
            
            //middleware
            //.wrap(middleware::NormalizePath::default())
            .wrap(CookieSession::private(&[0; 32]).secure(false).lazy(true))
            
            //load modules
            .service(web::scope("/api").configure(api::config))
            .service(web::scope("/events").configure(events::config))
            
            //static files
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    } )
        .bind("127.0.0.1:8000")?
        .run()
        .await
}


fn ensure_cookie_consent(request: &HttpRequest) -> HttpResult<()>
{
    let consent = request.cookie("CONSENT");
    if consent.is_some()
    {
        match consent.unwrap().value()
        {
            "1" => return Ok(()),
            _ => {},
        };
    }
    Err(error::ErrorUnauthorized("Cookies were not accepted, but are required!"))
}
