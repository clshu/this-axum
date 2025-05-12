#[derive(Debug, Clone)]
pub struct Config {
    pub db_name: String,
    pub mongodb_uri: String,
    pub app_secret: String,
    pub token_expires_in: u64,
    pub session_secret: String,
    pub session_expires_in: u64,
    pub mail_server: String,
    pub mail_server_port: u16,
    pub mail_user: String,
    pub mail_pass: String,
    pub mail_from: String,
}

impl Config {
    pub fn new() -> Self {
        // Load environment variables from a .env file
        dotenv::dotenv().ok();

        let db_name = std::env::var("DB_NAME").unwrap();
        if db_name.is_empty() {
            panic!("DB_NAME is not set");
        }
        let mongodb_uri = std::env::var("MONGODB_URI").unwrap();
        if mongodb_uri.is_empty() {
            panic!("MONGODB_URI is not set");
        }
        let app_secret = std::env::var("APP_SECRET").unwrap();
        if app_secret.is_empty() {
            panic!("APP_SECRET is not set");
        }
        let token_expires_in = std::env::var("TOKEN_EXPIRES_IN").unwrap();
        let token_expires_in = token_expires_in.parse::<u64>().unwrap_or(3600);

        let session_secret = std::env::var("SESSION_SECRET").unwrap();
        if session_secret.is_empty() {
            panic!("SESSION_SECRET is not set");
        }
        let session_expires_in = std::env::var("SESSION_EXPIRES_IN").unwrap();
        let session_expires_in = session_expires_in.parse::<u64>().unwrap_or(3600);
        let mail_server = std::env::var("MAIL_SERVER").unwrap();
        if mail_server.is_empty() {
            panic!("MAIL_SERVER is not set");
        }
        let mail_server_port = std::env::var("MAIL_SERVER_PORT").unwrap();
        let mail_server_port = mail_server_port.parse::<u16>().unwrap_or(587);
        let mail_user = std::env::var("MAIL_USER").unwrap();
        if mail_user.is_empty() {
            panic!("MAIL_USER is not set");
        }
        let mail_pass = std::env::var("MAIL_PASS").unwrap();
        if mail_pass.is_empty() {
            panic!("MAIL_PASS is not set");
        }
        let mail_from = std::env::var("MAIL_FROM").unwrap();
        if mail_from.is_empty() {
            panic!("MAIL_FROM is not set");
        }

        Self {
            db_name,
            mongodb_uri,
            app_secret,
            token_expires_in,
            session_secret,
            session_expires_in,
            mail_server,
            mail_server_port,
            mail_user,
            mail_pass,
            mail_from,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_name: String::from("default_db"),
            mongodb_uri: String::from("mongodb://localhost:27017"),
            app_secret: String::from("default_secret"),
            token_expires_in: 3600,
            session_secret: String::from("default_session_secret"),
            session_expires_in: 3600,
            mail_server: String::from("localhost"),
            mail_server_port: 587,
            mail_user: String::from("default_user"),
            mail_pass: String::from("default_pass"),
            mail_from: String::from("default@example.com"),
        }
    }
}
