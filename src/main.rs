use error_stack::{IntoReport, Result, ResultExt};

mod error;
use error::ServerError;
mod model;
mod routes;
mod tasks;

async fn run_server() -> Result<(), ServerError> {
    log::trace!("Getting environment variables");
    let database_url = std::env::var("DATABASE_URL")
        .into_report()
        .change_context(ServerError)
        .attach(
            "Failed to get environment variable `DATABASE_URL`, needed to connect to database.",
        )?;
    let api_key = std::env::var("ALPHA_VANTAGE_API_KEY")
        .into_report()
        .change_context(ServerError)
        .attach("Environment variable `ALPHA_VANTAGE_API_KEY` is not set")?;

    log::trace!("Connecting to database");
    let pool = tasks::connect_to_database(&database_url)
        .await
        .change_context(ServerError)?;

    log::trace!("Creating table");
    tasks::create_table_if_not_exists(pool.clone())
        .await
        .change_context(ServerError)
        .attach("Failed to create table on Postgres database.")?;

    log::trace!("Creating recurring task");
    let (task_send, task_recv) = tokio::sync::oneshot::channel::<()>();
    let upsert_task = tokio::spawn(tasks::recurring_get_raw_data(
        pool.clone(),
        task_recv,
        api_key,
        1,
    ));
    log::trace!("Starting up server.");
    let server_task = tokio::spawn(tasks::server_startup(pool.clone(), task_send));

    let (upsert_res, server_res) = tokio::join!(upsert_task, server_task);

    upsert_res
        .into_report()
        .change_context(ServerError)
        .attach("Failed to join Upsert task.")?
        .change_context(ServerError)
        .attach("Upsert task returned with an error.")?;
    server_res
        .into_report()
        .change_context(ServerError)
        .attach("Failed to join Upsert task.")?
        .change_context(ServerError)
}

fn main() -> Result<(), ServerError> {
    pretty_env_logger::init();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .into_report()
        .change_context(ServerError)
        .attach("Failed to build Tokio runtime.")?;

    runtime.block_on(run_server())
}
