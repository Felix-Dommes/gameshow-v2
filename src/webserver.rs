use std::env;

use actix_session::{
	config::CookieContentSecurity, storage::CookieSessionStore, SessionMiddleware,
};
use actix_web::{
	cookie::Key, error, middleware, web, App, HttpRequest, HttpServer, Result as HttpResult,
};

mod api;
mod events;

use crate::datahandler::DataHandler;

const MAX_JSON_PAYLOAD: usize = 51200; //50 kiB

pub async fn startup(db: DataHandler) -> std::io::Result<()> {
	let app_db = web::Data::new(db);
	let json_config = web::JsonConfig::default().limit(MAX_JSON_PAYLOAD);
	let cookie_key = Key::generate();

	HttpServer::new(move || {
		App::new()
			//config
			.app_data(json_config.clone())
			//shared data
			.app_data(app_db.clone())
			//middleware
			//.wrap(middleware::NormalizePath::default())
			.wrap(
				SessionMiddleware::builder(CookieSessionStore::default(), cookie_key.clone())
					.cookie_secure(false)
					.cookie_content_security(CookieContentSecurity::Private)
					.build(),
			)
			//load modules
			.service(
				web::scope("/api")
					.wrap(middleware::DefaultHeaders::new().add(("Cache-Control", "no-cache")))
					.configure(api::config),
			)
			.service(web::scope("/events").configure(events::config))
			//static files
			.service(actix_files::Files::new("/", "./static").index_file("index.html"))
	})
	.bind(env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_owned()))?
	.run()
	.await
}

fn ensure_cookie_consent(request: &HttpRequest) -> HttpResult<()> {
	let consent_cookie = request.cookie("CONSENT");
	if let Some(consent) = consent_cookie {
		if consent.value() == "1" {
			return Ok(());
		};
	}
	Err(error::ErrorUnauthorized("Cookies were not accepted, but are required!"))
}
