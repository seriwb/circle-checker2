use config::{Config, ConfigError};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CC {
    pub target_list: Vec<String>,
    pub separator: String,
    pub file_output: bool,
}
impl CC {
    fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config/config.toml"))
            .build()
            .unwrap();
        settings.try_deserialize()
    }
}

#[derive(Debug)]
pub struct AppConfig {
    pub cc: CC,
    pub consumer_key: String,
    pub consumer_secret: String,
}

// アプリケーションの利用の事前準備
// ・.secretからTwitter APIアクセスのキーを読み込む
// ・アプリケーションで利用するコンフィグオブジェクトを返す
pub fn init() -> AppConfig {

    let bytes = include_bytes!("../.secret");
    let secret = String::from_utf8_lossy(bytes);
    let keys = secret.split('\n').fold(Vec::new(), |mut s, i| {
        s.push(i.to_string());
        s
    });
    let cc = CC::new();
    AppConfig {
        cc: cc.unwrap(),
        consumer_key: keys[0]["consumerKey=".len()..].to_string(),
        consumer_secret: keys[1]["consumerSecret=".len()..].to_string(),
    }
}
