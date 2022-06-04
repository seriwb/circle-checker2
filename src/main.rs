mod bootstrap;
mod twitter;

use twitter::Twitter;

fn main() {
    let config = bootstrap::init();
    println!(
        "{} {} {}",
        config.cc.target_list, config.cc.tweet_maxcount, config.cc.loop_waittime
    );

    // TODO: Twitter認証
    let mut twitter = Twitter::new(config.access_token, config.access_token_secret);
    twitter.authenticate();

    // TODO: Twitterから情報取得
    // list users
    // TODO: 結果を画面出力
    tsv_header();
}

#[derive(Debug)]
struct CircleInfo {
    twitter_ame: String,
    twitter_id: String,
    twitter_url: String,
    match_string: String,
    space_string: String,
    profile_image_url: String,
    pinned_tweet_url: String,
    pinned_image_urls: Vec<String>,
}
impl CircleInfo {}

fn ountput(lists: &Vec<CircleInfo>) {
}

fn tsv_header() {
    let headers = [
        "Twitter ID",
        "Twitter Name",
        "アイコン",
        "一致イベント名",
        "スペース番号",
        "画像1",
        "画像2",
        "画像3",
        "画像4",
        "Twitter URL",
        "Twitter Link",
        "固定されたツイート",
        "プロフィール画像",
        "固定されたツイートの画像",
    ];
    println!("{}", headers.join("\t"));
}
