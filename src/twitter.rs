use std::{
    fs::File,
    io::{stdout, Read, Write},
};

use egg_mode::Token;
use log::{debug, trace};
use serde::{Deserialize, Serialize};

pub struct Config {
    pub token: egg_mode::Token,
    pub user_id: u64,
    pub screen_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
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
                    ..Default::default()
                },
            };
            twitter
        }
    }

    // 利用ユーザの認証を行う
    pub async fn authenticate(&mut self) {
        debug!("Twitter Authenticate");

        let account = self.verify_credentials().await;
        // まだアカウント情報がないor認証に失敗する場合はPIN認証のプロセスを実施
        if !account.existed() && !self.is_authorized() {
            self.oauth_process().await;
            // self.account = self.verify_credentials().await;
        } else {
            self.account = account;
        }
        // verifyCredentials() -> User
    }

    // 認証済みかどうかを確認する
    // TODO: Twitterの連携アプリ認証が取り消されていた場合、falseを返す。
    fn is_authorized(&self) -> bool {
        // TODO: 現在のアクセストークンの有効性をチェック(get_access_tokenができるかどうか)
        false
    }

    // Twitterのユーザ認証を行い、アクセストークンを設定する
    async fn oauth_process(&mut self) {
        debug!("OAuth authentication start.");
        // Twitter APIでアクセストークンを取得
        let token =
            get_access_tokens(self.consummer_key.clone(), self.consummer_secret.clone()).await;
        println!("{:?}", token.0);
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
        // TODO: screen_nameが値になっている
        let name = &token.2;
        self.account = User {
            name: (&name).to_string(),
            screen_name: token.1.to_string(),
            url: format!("https://twitter.com/{}", &name),
        };
        debug!("OAuth authentication success.");

        self.store_access_tokens();
        debug!("OAuth authentication end.");
    }

    // AccessTokenをファイルに保存
    fn store_access_tokens(&self) {
        let mut file = File::create("token").expect("not_found");
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        file.write_all(&encoded).expect("cannot write");
    }

    // アクセス可能なユーザ情報を取得
    async fn verify_credentials(&self) -> User {
        // https://docs.rs/egg-mode/latest/egg_mode/auth/fn.verify_tokens.html

        // TODO: 現在の構造体情報で認証チャレンジ
        if true {
            // TODO: 認証できたら取得したユーザ情報を設定して返却
            println!("set account");
            User {
                ..Default::default()
            }
        } else {
            User {
                ..Default::default()
            }
        }
    }

    // pub fn get_user_lists(userinfo.getScreenName())
    // pub fn get_user_list_members(targetList.id, -1L)
    // pub fn get_friends_list(userinfo.id, -1L) // フォローユーザのリスト
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
    let mut file = File::open("token");
    if file.is_err() {
        return Err("damedatta".to_string());
    }
    let mut buffer = Vec::new();
    let _ = file.unwrap().read_to_end(&mut buffer);
    let twitter: Twitter = bincode::deserialize(&buffer).unwrap();

    trace!("{:?}", &twitter);
    return Ok(twitter);
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    name: String,
    screen_name: String, //id
    url: String,
}
impl Default for User {
    fn default() -> User {
        User {
            name: "".to_string(),
            screen_name: "".to_string(),
            url: "".to_string(),
        }
    }
}
impl User {
    fn existed(&self) -> bool {
        if self == &Default::default() {
            println!("まだデータ無いよ"); // TODO:呼ぶタイミングチェック
            false
        } else {
            true
        }
    }
}
