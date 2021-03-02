#![allow(dead_code)] //TODO remove?

use tokio_postgres::{Client, Error, NoTls};
use tokio_postgres::config::Config;
use actix_web::rt;
use std::env;


pub struct Database
{
    client: Client,
}

impl Database
{
    pub async fn connect() -> Result<Database, Error>
    {
        let mut config = Config::new();
        config.host(&env::var("DATABASE_HOST").unwrap_or(String::from("localhost")))
            .port(env::var("DATABASE_PORT").unwrap_or_default().parse().unwrap_or(5432))
            .user(&env::var("DATABASE_USER").unwrap_or(String::from("postgres")))
            .dbname(&env::var("DATABASE_DATABASE").unwrap_or(String::from("gameshow")))
            .application_name("Gameshow-Server");
        let pw = env::var("DATABASE_PASSWORD");
        if pw.is_ok()
        {
            config.password(pw.ok().unwrap());
        }
        
        let (client, connection) = config.connect(NoTls).await?;
        
        rt::spawn(async move {
            if let Err(e) = connection.await
            {
                eprintln!("Postgresql connection error: {}", e);
                //TODO panic?
            }
        });
        
        Ok(Database { client: client })
    }
    
    pub async fn migration_up(&self) -> Result<(), Error>
    {
        let sql = "
            CREATE TABLE test (
                id SERIAL PRIMARY KEY
            );
        ";
        self.client.batch_execute(sql).await?;
        Ok(())
    }
    
    pub async fn migration_down(&self) -> Result<(), Error>
    {
        let sql = "
            DROP TABLE test;
        ";
        self.client.batch_execute(sql).await?;
        Ok(())
    }
    
    pub async fn test(&self) -> Result<(), Error>
    {
        println!("Testing..");
        // Now we can execute a simple statement that just returns its parameter.
        let rows = self.client
            .query("SELECT $1::TEXT", &[&"hello world"])
            .await?;

        // And then check that we got back the same string we sent over.
        let value: &str = rows[0].get(0);
        assert_eq!(value, "hello world");
        println!("{}", value);
        
        Ok(())
    }
}
