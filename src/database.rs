use mongodb::{bson::doc, options::ClientOptions, Client};

pub async fn get_database_client() -> mongodb::error::Result<Client> {
    tracing::info!("Database Setup Started...");
    let client_options = ClientOptions::parse(dotenv!("DATABASE_CONNECTION_STRING")).await?;
    let client = Client::with_options(client_options)?;
    client
        .database(dotenv!("DATABASE_NAME"))
        .run_command(doc! {"ping": 1}, None)
        .await?;

    tracing::info!("Database Setup Succeeded...");
    Ok(client)
}
