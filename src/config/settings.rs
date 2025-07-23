use dotenvy::dotenv;

pub struct Settings {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_exp: i64,
    pub username: String,
    pub password: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = dotenvy::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_exp = dotenvy::var("JWT_EXP")
            .expect("JWT_EXP must be set")
            .parse::<i64>()
            .unwrap();
        let username = dotenvy::var("USERNAME").expect("USERNAME must be set");
        let password = dotenvy::var("PASSWORD").expect("PASSWORD must be set");
        Self {
            database_url: database_url,
            jwt_secret: jwt_secret,
            jwt_exp: jwt_exp,
            username: username,
            password: password,
        }
    }
}
