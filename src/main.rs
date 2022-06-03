mod bootstrap;
mod twitter;

use twitter::Twitter;

fn main() {
    let config = bootstrap::init();
    println!("{} {} {}", config.cc.target_list, config.cc.tweet_maxcount, config.cc.loop_waittime);

    // TODO: Twitter認証
    let mut twitter = Twitter::new(config.access_token, config.access_token_secret);
    twitter.authenticate();

    // TODO: Twitterから情報取得

    // TODO: 結果を画面出力
}
