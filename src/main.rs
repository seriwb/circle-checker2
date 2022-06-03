mod bootstrap;
mod twitter;

fn main() {
    let config = bootstrap::init();
    // execute(config);
}

// fn execute(config: Config) {
//     // TODO: Twitter認証
//     let userinfo = twitter::authenticate();

//     // TODO: 設定ファイル読み込み
//     let filterring_lists: [] = load_config();

//     // TODO: Twitterから情報取得

//     // TODO: 結果を画面出力
// }

// /**
//  * configディレクトリ配下の設定を読み込む
//  */
// fn load_config() -> any {}