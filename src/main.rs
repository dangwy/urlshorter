use urlshorter::configure;
use urlshorter::error::AppResult;
use urlshorter::server::AppServer;
use urlshorter::constant::CONFIG;
use tracing::info;

#[tokio::main]
async fn main()-> AppResult<()> {
    let _file_appender_guard = configure::tracing::init()?;
    info!("The initialization of Tracing was successful.");

    let config = CONFIG.clone();
    info!("Reading the config file was successful.");

    info!("Create a new server.");
    let server = AppServer::new(config).await?;

    info!("Run the server.");
    server.run().await?;

    Ok(())
}
