use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
use tauri::{AppHandle, Manager};

pub fn get_database_pool(app: &AppHandle) -> DatabaseConnection {
    println!("Connecting to developer database");
    tauri::async_runtime::block_on(async {
        let app_data_dir = app.path().app_data_dir().unwrap();

        std::fs::create_dir_all(&app_data_dir).unwrap();
        let url = format!(
            "sqlite:{}",
            app_data_dir
                .join("database.sqlite3?mode=rwc")
                .to_string_lossy()
        );

        let mut opt = ConnectOptions::new(url.to_owned());
        opt.max_connections(10).acquire_timeout(Duration::from_secs(6));

        let connection = sea_orm::Database::connect(opt).await.unwrap();
        Migrator::up(&connection, None).await.unwrap();

        println!("app_data_dir: {url:?}");

        return connection;
    })
}
