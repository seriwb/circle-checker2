use config::{Config, ConfigError};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CC {
    pub target_list: Vec<String>,
    pub separator: String,
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

/**
 * 初期化処理
 * <ul>
 * <li>config/config.tomlに記載された設定値の読み込み</li>
 * <li>Twitter API利用の初期処理</li>
 * </ul>
 */
pub fn init() -> AppConfig {

    let bytes = include_bytes!("../.secret");
    let secret = String::from_utf8_lossy(bytes);
    // 配列で確保する例
    let test = secret.split('\n').fold(Vec::new(), |mut s, i| {
        s.push(i.to_string());
        s
    });
    // 直で利用する例（空列も表示している）
    // for v in secret.split('\n') {
    //     println!("{}", v);
    // }

    // TODO: ファイル読み込み
    // let filterring_lists: [] = load_filter();

    let cc = CC::new();
    AppConfig {
        cc: cc.unwrap(),
        consumer_key: test[0]["consumerKey=".len()..].to_string(),
        consumer_secret: test[1]["consumerSecret=".len()..].to_string(),
    }
}

// フィルタ設定の読み込み
// fn load_filter() -> Vec<String> {}
