mod bootstrap;
mod twitter;

use egg_mode::user::TwitterUser;
use regex::Regex;
use twitter::Twitter;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = bootstrap::init();

    let mut twitter = Twitter::new(config.consumer_key, config.consumer_secret);
    let token = twitter.authenticate().await;

    let users_lists = twitter.get_user_lists(&token).await;
    // println!("{:?}", &users_lists);

    for target in config.cc.target_list {
        let circles: Vec<CircleInfo> = if target == "" {
            // タイムラインのデータを取得
            let users = twitter.get_friends_list(&token).await;
            // フィルタリングしたデータをCircleInfoに格納
            users
                .iter()
                .filter_map(|user| check_user_info(user).ok())
                .collect::<Vec<CircleInfo>>()
        } else {
            // リストのIDを取得
            let list_id = &users_lists.iter().find(|x| *x.name == target);
            if list_id.is_none() {
                println!("リスト\"{}\"は存在しません", &target);
                continue;
            }
            // リストのデータを取得
            let users = twitter
                .get_user_list_members(list_id.unwrap().id, &token)
                .await;
            // フィルタリングしたデータをCircleInfoに格納
            users
                .iter()
                .filter_map(|user| check_user_info(user).ok())
                .collect::<Vec<CircleInfo>>()
        };

        // 結果を画面出力
        let title = if target == "" {
            "タイムライン"
        } else {
            &target
        };
        println!("{}", title);
        ountput(&circles, config.cc.separator.as_str());
    }
}

fn check_user_info(user: &TwitterUser) -> Result<CircleInfo, String> {
    // TODO: フィルタ群でマッチング
    let filters = vec![r"(C9\d|Ｃ９[０-９]|C1\d\d|Ｃ１[０-９][０-９])(.*)".to_string()];
    for filter in filters {
        let hoge = format!(r"{}", &filter);
        let re = Regex::new(&hoge).unwrap();
        let caps = re.captures(&user.name);
        if caps.is_none() {
            continue;
        }
        let cap_strs = caps.unwrap();
        return Ok(CircleInfo {
            twitter_id: user.screen_name.clone(),
            twitter_name: user.name.clone(),
            twitter_url: user.url.clone().unwrap(),
            match_string: cap_strs[1].to_string(),
            space_string: cap_strs[2].to_string(),
            profile_image_url: user.profile_image_url.clone(),
            // pinned_tweet_url: "TODO".to_string(),
            // pinned_image_urls: vec!["TODO 2".to_string()],
        });
    }
    return Err("non".to_string());
}

#[derive(Debug)]
struct CircleInfo {
    twitter_id: String,
    twitter_name: String,
    twitter_url: String,
    match_string: String,
    space_string: String,
    profile_image_url: String,
    // pinned_tweet_url: String,
    // pinned_image_urls: Vec<String>,
}
impl CircleInfo {
    fn output(&self, separator: &str, row: &usize) {
        let line = vec![
            self.twitter_id.clone(),
            self.twitter_name.clone(),
            format!("=IMAGE($M{})", &row),
            self.match_string.clone(),
            self.space_string.clone(),
            // format!("=IMAGE($N{})", &row),
            // format!("=IMAGE($O{})", &row),
            // format!("=IMAGE($P{})", &row),
            // format!("=IMAGE($Q{})", &row),
            format!("https://twitter.com/{}", &self.twitter_id),
            self.twitter_url.clone(),
            // String::from("TODO 固定されたツイート用"),
            self.profile_image_url.clone(),
            // self.pinned_tweet_url.clone(),
        ];
        println!("{}", line.iter().map(|x| x.as_str()).collect::<Vec<&str>>().join(&separator));
    }
}

fn ountput(lists: &Vec<CircleInfo>, separator: &str) {
    print_header(&separator);
    for (i, info) in lists.iter().enumerate() {
        let row = i+2;  // ヘッダー行を除くので2列目から
        info.output(&separator, &row);
    }
}

fn print_header(separator: &str) {
    let headers = [
        "Twitter ID",
        "Twitter Name",
        "アイコン",
        "一致イベント名",
        "スペース番号",
        // "画像1",
        // "画像2",
        // "画像3",
        // "画像4",
        "Twitter URL",
        "Twitter Link",
        // "固定されたツイート",
        "プロフィール画像",
        // "固定されたツイートの画像1",
        // "画像2",
        // "画像3",
        // "画像4",
    ];
    println!("{}", headers.join(&separator));
}
