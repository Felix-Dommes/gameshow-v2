use dotenv::dotenv;

mod datahandler;

use datahandler::DataHandler;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    dotenv().ok();
    
    let db = DataHandler::new();
    db.create_player("Test".to_owned()).await?;
    
    Ok(())
}
