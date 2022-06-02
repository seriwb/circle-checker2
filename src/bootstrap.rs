use config::Config;
use std::collections::HashMap;

/**
 * 初期化処理
 * <ul>
 * <li>config/config.tomlに記載された設定値の読み込み</li>
 * <li>DBの初期データ作成</li>
 * <li>Twitter API利用の初期処理</li>
 * </ul>
 */
pub fn init() {
    let settings = Config::builder()
        .add_source(config::File::with_name("config/config.toml"))
        .build()
        .unwrap();

    println!(
        "{:?}",
        settings//.get_table("cc")
            .try_deserialize::<HashMap<String, HashMap<String, String>>>()
            .unwrap()
    );
}