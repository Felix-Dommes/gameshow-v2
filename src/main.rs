use datahandler::DataHandler;
use dotenvy::dotenv;
use gameshow_v2::*;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	dotenv().ok();

	let db = DataHandler::new();
	db.set_question_sets(game::find_question_files()?).await?;

	webserver::startup(db).await?;

	Ok(())
}
