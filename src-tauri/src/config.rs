use serde::Deserialize;
use tokio::fs::read_to_string;

#[derive(Deserialize)]
pub struct SqlserverConfig {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub sql_server: SqlserverConfig,
    pub db_name: String,
}

impl AppConfig {
    pub fn new(yml_data: &str) -> Self {
        let config = match serde_yaml::from_str(yml_data) {
        Ok(app_config) => app_config,
        Err(e) => panic!("{}", e),
        };

        config
    }

    pub fn validate(&self) {
        if self.sql_server.host.is_empty() {
            panic!("请配置连接数据库ip !!!!")
        }
    }
}


pub async fn init_config() -> AppConfig {
    let content = read_to_string("config.yml").await.unwrap(); // TODO
    let app_config = AppConfig::new(&content);

    app_config
}

