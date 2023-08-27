#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub db_url: String,

    pub access_token_private_key: String,
    pub access_token_public_key: String,
    pub access_token_expires_in: String,
    pub access_token_max_age: i64,

    pub refresh_token_private_key: String,
    pub refresh_token_public_key: String,
    pub refresh_token_expires_in: String,
    pub refresh_token_max_age: i64,
}

impl Config {
    pub fn init() -> Config {
        let port: u16 = std::env::var("PORT")
            .unwrap_or("8080".to_string())
            .parse()
            .unwrap();
        let db_url = get_env("DATABASE_URL");

        let access_token_private_key = get_env("ACCESS_TOKEN_PRIVATE_KEY");
        let access_token_public_key = get_env("ACCESS_TOKEN_PUBLIC_KEY");
        let access_token_expires_in = get_env("ACCESS_TOKEN_EXPIRED_IN");
        let access_token_max_age = get_env("ACCESS_TOKEN_MAXAGE");

        let refresh_token_private_key = get_env("REFRESH_TOKEN_PRIVATE_KEY");
        let refresh_token_public_key = get_env("REFRESH_TOKEN_PUBLIC_KEY");
        let refresh_token_expires_in = get_env("REFRESH_TOKEN_EXPIRED_IN");
        let refresh_token_max_age = get_env("REFRESH_TOKEN_MAXAGE");

        Config {
            port,
            db_url,
            access_token_private_key,
            access_token_public_key,
            refresh_token_private_key,
            refresh_token_public_key,
            access_token_expires_in,
            refresh_token_expires_in,
            access_token_max_age: access_token_max_age.parse::<i64>().unwrap(),
            refresh_token_max_age: refresh_token_max_age.parse::<i64>().unwrap(),
        }
    }
}

fn get_env(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| panic!("{} Missing Env .env*", var_name))
}
