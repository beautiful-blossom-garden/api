use crate::config::settings::Settings;
use diesel_async::{AsyncConnection, AsyncPgConnection};

pub async fn establish_connection() -> AsyncPgConnection {
    let settings = Settings::new();
    AsyncPgConnection::establish(&settings.database_url)
        .await
        .unwrap()
}
