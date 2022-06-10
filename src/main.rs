mod bootstrap;
mod twitter;

use std::{
    fs::{File, self},
    io::{BufRead, BufReader, Write},
};

use chrono::{Local};
use egg_mode::user::TwitterUser;
use log::debug;
use regex::Regex;
use twitter::Twitter;

const HEADERS: [&str; 8] = [
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
const DIR: &str = "output";

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = bootstrap::init();
    let filters = load_filter();

    let mut twitter = Twitter::new(config.consumer_key, config.consumer_secret);
    let token = twitter.authenticate().await;

    let users_lists = twitter.get_user_lists(&token).await;
    if config.cc.file_output {
        mkdir(&DIR);
    }

    for target in config.cc.target_list {
        let circles: Vec<CircleInfo> = if target == "" {
            // タイムラインのデータを取得
            let users = twitter.get_friends_list(&token).await;
            // フィルタリングしたデータをCircleInfoに格納
            users
                .iter()
                .filter_map(|user| check_user_info(user, &filters).ok())
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
                .filter_map(|user| check_user_info(user, &filters).ok())
                .collect::<Vec<CircleInfo>>()
        };

        let title = if target == "" {
            "タイムライン"
        } else {
            &target
        };
        println!("{}", title);

        if config.cc.file_output {
            // 結果をファイル出力
            file_output(&circles, &title, config.cc.separator.as_str());
        } else {
            // 結果を画面出力
            output(&circles, config.cc.separator.as_str());
        }
    }
}

// フィルタ設定の読み込み
fn load_filter() -> Vec<String> {
    let file = File::open("config/filter.txt").expect("filter.txtがありません");
    let mut filters = Vec::new();
    for line in BufReader::new(file).lines() {
        if let Ok(filter) = line {
            filters.push(filter);
        }
    }
    debug!("{:?}", &filters);
    filters
}

fn check_user_info(user: &TwitterUser, filters: &Vec<String>) -> Result<CircleInfo, String> {
    // フィルタ群でマッチング
    for filter in filters {
        let re_f = format!(r"({})(.*)", &filter);
        let re = Regex::new(&re_f).unwrap();
        let caps = re.captures(&user.name);
        if caps.is_none() {
            continue;
        }
        let cap_strs = caps.unwrap();
        return Ok(CircleInfo {
            twitter_id: user.screen_name.clone(),
            twitter_name: user.name.clone(),
            twitter_url: match user.url.clone() {
                Some(url) => url,
                None => "".to_string(),
            },
            match_string: cap_strs[1].to_string(),
            space_string: cap_strs[2].to_string(),
            profile_image_url: user.profile_image_url.clone(),
            // pinned_tweet_url: "TODO".to_string(),
            // pinned_image_urls: vec!["TODO 2".to_string()],
        });
    }
    return Err("non".to_string());
}


fn mkdir(file_name: &str) -> bool {
    match fs::create_dir(file_name) {
        Err(_e) => false,
        Ok(_) => true,
    }
}

fn file_output(lists: &Vec<CircleInfo>, title: &str, separator: &str) {
    let identifier = match separator {
        "," => "csv",
        "\t" => "tsx",
        _ => "txt,",
    };
    let date_text = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let file_path = format!("{DIR}/{title}_{date_text}.{identifier}");
    let mut file = File::create(&file_path).expect(format!("作成失敗: {}", &file_path).as_str());

    let line = HEADERS.join(&separator);
    writeln!(file, "{}", line).expect(format!("書き込み失敗: {}", &line).as_str());

    for (i, info) in lists.iter().enumerate() {
        let row = i + 2; // ヘッダー行を除くので2列目から
        let line = info.to_string(&separator, &row);
        writeln!(file, "{}", line).expect(format!("書き込み失敗: {}", &line).as_str());
    }
    println!("{file_path}");
}

fn output(lists: &Vec<CircleInfo>, separator: &str) {
    println!("{}", HEADERS.join(&separator));
    for (i, info) in lists.iter().enumerate() {
        let row = i + 2; // ヘッダー行を除くので2列目から
        println!("{}", info.to_string(&separator, &row));
    }
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
    fn to_string(&self, separator: &str, row: &usize) -> String {
        let line = vec![
            self.twitter_id.clone(),
            self.twitter_name.clone(),
            format!("=IMAGE($H{})", &row),
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
        line.iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .join(&separator)
    }
}
