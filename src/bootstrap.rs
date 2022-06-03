use config::Config;
use std::collections::HashMap;

// pub struct Config {
//   x: i32,
// }
// impl Config {
//   fn new(x: i32) -> Config {
//     Config { x: x }
//   }
// }
// pub fn init() -> Config {
//     println!("Hello, world!");
//     return Config::new(1);
// }
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
    // consumerSecret=をトリム
    println!("{}", &test[1][15..]);

    // 直で利用する例
    for v in secret.split('\n') {
      println!("{}", v);
    }
}
