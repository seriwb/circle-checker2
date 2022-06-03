use config::Config;
use std::collections::HashMap;

pub struct AppConfig {
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

    println!(
        "{:?}",
        settings //.get_table("cc")
            .try_deserialize::<HashMap<String, HashMap<String, String>>>()
            .unwrap()
    );

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
        access_token: test[0]["consumerKey=".len()..].to_string(),
        access_token_secret: test[1]["consumerSecret=".len()..].to_string(),
    }
}


// フィルタ設定の読み込み
// fn load_filter() -> Vec<String> {}