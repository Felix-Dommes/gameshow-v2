use dotenv::dotenv;

mod database;

use database::Database;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    dotenv().ok();
    let db = Database::connect().await?;
    db.migration_up().await?;
    
    //
    
    db.migration_down().await?;
    Ok(())
}
