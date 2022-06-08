use std::{
    fs::File,
    io::{stdout, Read, Write},
};

use egg_mode::{
    auth::verify_tokens,
    list::{list, members, List},
    user::{friends_of, TwitterUser},
    KeyPair, Token,
};
use log::debug;
use scraper::Html;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Twitter {
    consummer_key: String,
    consummer_secret: String,
    access_token: String,
    access_token_secret: String,
    account: User,
}
impl Twitter {
    // ローカルにあるものはここで全部はめる
    pub fn new(consummer_key: impl Into<String>, consummer_secret: impl Into<String>) -> Twitter {
        // 認証データダンプからアクセストークンとシークレットを取得チャレンジ
        let load_data = load_access_tokens();
        if load_data.is_err() {
            debug!("ファイル読み込み結果: {}", load_data.err().unwrap());
            let twitter = Twitter {
                consummer_key: consummer_key.into(),
                consummer_secret: consummer_secret.into(),
                access_token: "".to_string(),
                access_token_secret: "".to_string(),
                account: User {
                    ..Default::default()
                },
            };
            twitter
        } else {
            let data = load_data.unwrap();
            // 取得したAccessTokenをはめてインスタンスを返却
            let twitter = Twitter {
                consummer_key: consummer_key.into(),
                consummer_secret: consummer_secret.into(),
                access_token: data.access_token.to_string(),
                access_token_secret: data.access_token_secret.to_string(),
                account: User {
                    id: data.account.id,
                    name: data.account.name,
                    url: data.account.url,
                },
            };
            twitter
        }
    }

    // 利用ユーザの認証を行う
    pub async fn authenticate(&mut self) -> Token {
        debug!("Twitter Authenticate");

        let result = self.verify_credentials().await;
        // まだアカウント情報がないor認証に失敗する場合はPIN認証のプロセスを実施
        if !result.1.existed() {
            let token = self
                .oauth_process(self.consummer_key.clone(), self.consummer_secret.clone())
                .await;
            return token;
        } else {
            self.account = result.1;
            return result.0;
        }
    }

    // Twitterのユーザ認証を行い、アクセストークンを設定する
    async fn oauth_process(&mut self, consummer_key: String, consummer_secret: String) -> Token {
        debug!("OAuth authentication start.");
        // Twitter APIでアクセストークンを取得
        let token = get_access_tokens(consummer_key, consummer_secret).await;

        match token.0 {
            Token::Access {
                access: ref access_token,
                ..
            } => {
                self.access_token = access_token.key.to_string();
                self.access_token_secret = access_token.secret.to_string();
            }
            _ => unreachable!(),
        }

        let name = &token.2;
        self.account = User {
            id: token.1,
            name: (&name).to_string(),
            url: format!("https://twitter.com/{}", &name),
        };
        debug!("OAuth authentication success.");

        self.store_access_tokens();
        debug!("OAuth authentication end.");
        token.0
    }

    // AccessTokenをファイルに保存（アプリのキーは消す）
    fn store_access_tokens(&self) {
        let mut store_data = self.clone();
        store_data.consummer_key = "".to_string();
        store_data.consummer_secret = "".to_string();
        let mut file = File::create("token").expect("not_found");
        let encoded: Vec<u8> = bincode::serialize(&store_data).unwrap();
        file.write_all(&encoded).expect("cannot write");
    }

    // アクセス可能なユーザ情報を取得
    async fn verify_credentials(&self) -> (Token, User) {
        // https://docs.rs/egg-mode/latest/egg_mode/auth/fn.verify_tokens.html
        let token = Token::Access {
            consumer: KeyPair::new(self.consummer_key.clone(), self.consummer_secret.clone()),
            access: KeyPair::new(self.access_token.clone(), self.access_token_secret.clone()),
        };
        let twitter_user = verify_tokens(&token).await;

        if twitter_user.is_ok() {
            // 認証できたら取得したユーザ情報を設定して返却
            debug!("verified!");
            let data = twitter_user.unwrap();
            (
                token,
                User {
                    id: data.id,
                    name: data.name.to_string(),
                    url: format!("https://twitter.com/{}", data.name.to_string()),
                },
            )
        } else {
            (
                token,
                User {
                    ..Default::default()
                },
            )
        }
    }

    // フォローユーザのリスト
    pub async fn get_friends_list(&self, token: &Token) -> Vec<TwitterUser> {
        let mut users: Vec<TwitterUser> = Vec::new();
        let mut cursors = friends_of(self.account.id, &token).with_page_size(200);
        loop {
            let resp = cursors.call().await.unwrap();
            users = [users, resp.response.users].concat();

            let next = resp.response.next_cursor;
            if next == 0 {
                break;
            }
            cursors.next_cursor = next;
        }
        users
    }

    // 自分がオーナのリストを優先して100件(API制限)取得
    pub async fn get_user_lists(&self, token: &Token) -> Vec<List> {
        let result = list(self.account.id, true, &token).await.unwrap();
        result.response
    }

    // 指定のリストに含まれるユーザ情報を取得
    pub async fn get_user_list_members(&self, list_id: u64, token: &Token) -> Vec<TwitterUser> {
        let mut users: Vec<TwitterUser> = Vec::new();
        let mut cursors =
            members(egg_mode::list::ListID::ID(list_id), &token).with_page_size(200);
        loop {
            let resp = cursors.call().await.unwrap();
            users = [users, resp.response.users].concat();

            let next = resp.response.next_cursor;
            if next == 0 {
                break;
            }
            cursors.next_cursor = next;
        }
        users
    }
}

// アクセストークンを取得(OAuth1.0a PIN認証のプロセスを実施)
async fn get_access_tokens(
    consummer_key: String,
    consummer_secret: String,
) -> (Token, u64, String) {
    let con_token = egg_mode::KeyPair::new(consummer_key, consummer_secret);
    let request_token = egg_mode::auth::request_token(&con_token, "oob")
        .await
        .unwrap();
    let auth_url = egg_mode::auth::authorize_url(&request_token);

    debug!("request token: {}", request_token.key);

    // ブラウザで認証ページを表示
    if webbrowser::open(&auth_url).is_err() {
        println!("Please access this URL: {}", &auth_url);
    }

    loop {
        print!("Enter the PIN(if available) or just hit enter.[PIN]: ");
        stdout().flush().unwrap();
        // PINの入力待ち
        let mut pin = String::new();
        std::io::stdin().read_line(&mut pin).ok();

        if pin.trim().len() > 0 {
            let token_result = egg_mode::auth::access_token(con_token, &request_token, pin)
                .await
                .unwrap();
            return token_result;
        } else {
            continue;
        }
    }
}

// ローカルに保存済みのアクセストークンを取得する
fn load_access_tokens() -> Result<Twitter, String> {
    let file = File::open("token");
    if file.is_err() {
        return Err("damedatta".to_string());
    }
    let mut buffer = Vec::new();
    let _ = file.unwrap().read_to_end(&mut buffer);
    let twitter: Twitter = bincode::deserialize(&buffer).unwrap();

    debug!("load data: {:?}", &twitter);
    return Ok(twitter);
}

// 固定されたツイートのIDを取得する
// -> HTMLが返ってこないので、ヘッドレスブラウザでも入れないと無理
pub fn  get_pinned_tweet_status(screen_name: &String) -> String {
    let url =  format!("https://twitter.com/{}", &screen_name);
    println!("{}", &url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&body);

    let article = scraper::Selector::parse("article").unwrap();
    let pin_svg = scraper::Selector::parse("svg > g > path").unwrap();
    let status_value = scraper::Selector::parse("a > time").unwrap();

    // 最初のarticleを取る
    let article_context = document.select(&article).next().unwrap();
    let pinned = article_context.select(&pin_svg).next();
    if pinned.is_none() {
        return "".to_string();
    }
    let d = pinned.unwrap().value().attr("d").unwrap();
    println!("{}", &d);
    if d != "M20.235 14.61c-.375-1.745-2.342-3.506-4.01-4.125l-.544-4.948 1.495-2.242c.157-.236.172-.538.037-.787-.134-.25-.392-.403-.675-.403h-9.14c-.284 0-.542.154-.676.403-.134.25-.12.553.038.788l1.498 2.247-.484 4.943c-1.668.62-3.633 2.38-4.004 4.116-.04.16-.016.404.132.594.103.132.304.29.68.29H8.64l2.904 6.712c.078.184.26.302.458.302s.38-.118.46-.302l2.903-6.713h4.057c.376 0 .576-.156.68-.286.146-.188.172-.434.135-.59z" {
        return "".to_string();
    }

    let pinned_status = article_context.select(&status_value).next().unwrap();
    let status = pinned_status.parent().unwrap().value().as_element().unwrap().attr("href").unwrap();
    println!("{}", &status);
    if status.trim().is_empty() {
        return "".to_string();
    }
    status.to_string()
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct User {
    id: u64,
    name: String,
    url: String,
}
impl Default for User {
    fn default() -> User {
        User {
            id: 0,
            name: "".to_string(),
            url: "".to_string(),
        }
    }
}
impl User {
    fn existed(&self) -> bool {
        if self == &Default::default() {
            false
        } else {
            true
        }
    }
}
