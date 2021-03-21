//TODO remove later
#![allow(dead_code, unused_variables, unused_imports)]

use dotenv::dotenv;

mod datahandler;
mod webserver;

use datahandler::DataHandler;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    dotenv().ok();
    
    let db = DataHandler::new();
    let res = webserver::startup(db).await?;
    
    Ok(res)
}
