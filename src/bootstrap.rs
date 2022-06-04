use config::Config;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CC {
    pub target_list: String,
    pub tweet_maxcount: u32,
    pub loop_waittime: u32,
}
#[derive(Debug)]
pub struct AppConfig {
    pub cc: CC,
    pub access_token: String,
    pub access_token_secret: String,
}

/**
 * 初期化処理
 * <ul>
 * <li>config/config.tomlに記載された設定値の読み込み</li>
 * <li>Twitter API利用の初期処理</li>
 * </ul>
 */
pub fn init() -> AppConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name("config/config.toml"))
        .build()
        .unwrap();
    let config_map = settings //.get_table("cc")
        .try_deserialize::<HashMap<String, HashMap<String, String>>>()
        .unwrap();
    println!("{:?}", config_map);

    let bytes = include_bytes!("../.secret");

    let secret = String::from_utf8_lossy(bytes);
    // 配列で確保する例
    let test = secret.split('\n').fold(Vec::new(), |mut s, i| {
        s.push(i.to_string());
        s
    });
    // println!("{}", &test[0]["consumerKey=".len()..]);
    // println!("{}", &test[1]["consumerSecret=".len()..]);

    // 直で利用する例（空列も表示している）
    for v in secret.split('\n') {
        println!("{}", v);
    }

    // TODO: ファイル読み込み
    // let filterring_lists: [] = load_filter();

    AppConfig {
        cc: CC {
            target_list: config_map["cc"]["target_list"].to_string(),
            tweet_maxcount: config_map["cc"]["tweet_maxcount"].parse().unwrap(),
            loop_waittime: config_map["cc"]["loop_waittime"].parse().unwrap(),
        },
        access_token: test[0]["consumerKey=".len()..].to_string(),
        access_token_secret: test[1]["consumerSecret=".len()..].to_string(),
    }
}

// フィルタ設定の読み込み
// fn load_filter() -> Vec<String> {}
