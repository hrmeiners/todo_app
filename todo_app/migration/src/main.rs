use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    // setting 'DATABASE_URL' environment variable
    let key = "DATABASE_URL";

    if std::env::var(key).is_err() {
        // Getting database URL from Rocket.toml if it is not set
        let figment = rocket::Config::figment();
        let database_url: String = figment
            .extract_inner("databases.todo.url")
            .expect("Cannot find Database URL in Rocket.toml");
        
        std::env::set_var(key, database_url);
    }

    cli::run_cli(migration::Migrator).await;

}
