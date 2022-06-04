use log::debug;

#[derive(Debug)]
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
        // TODO: 認証データダンプからアクセストークンとシークレットを取得チャレンジ
        let keys = load_access_tokens();

        // 取得したAccessTokenをはめてインスタンスを返却
        let twitter = Twitter {
            consummer_key: consummer_key.into(),
            consummer_secret: consummer_secret.into(),
            access_token: keys.0.to_string(),
            access_token_secret: keys.1.to_string(),
            account: User {
                ..Default::default()
            },
        };
        twitter
    }

    // 利用ユーザの認証を行う
    pub fn authenticate(&mut self) {
        debug!("Twitter Authenticate");

        let account = self.verify_credentials();
        // まだアカウント情報がないor認証に失敗する場合はPIN認証のプロセスを実施
        if !account.existed() && !self.is_authorized() {
            self.oauth_process();
            self.account = self.verify_credentials();
        } else {
            self.account = account;
        }

        println!("{}", self.consummer_key);
        println!("{}", self.consummer_secret);
        println!("{}", self.access_token);
        println!("{}", self.access_token_secret);

        // verifyCredentials() -> User
    }

    // 認証済みかどうかを確認する
    // TODO: Twitterの連携アプリ認証が取り消されていた場合、falseを返す。
    fn is_authorized(&self) -> bool {
        // TODO: 現在のアクセストークンの有効性をチェック(get_access_tokenができるかどうか)
        false
    }

    // TODO: Twitterのユーザ認証を行い、アクセストークンを設定する(PIN認証のプロセスを実施)
    fn oauth_process(&mut self) {
        debug!("OAuth authentication start.");
        // Twitter APIでアクセストークンを取得
        let keys = self.get_access_tokens();
        self.access_token = keys.0.to_string();
        self.access_token_secret = keys.1.to_string();
        debug!("OAuth authentication success.");

        self.store_access_tokens();
        debug!("OAuth authentication end.");
    }

    // TODO: アクセストークンを取得
    fn get_access_tokens(&self) -> (&'static str, &'static str) {
        let request_token = ("tekito-", "https://getAuthorizationURL"); // TODO: twitter.getOAuthRequestToken()
        println!("request token: {}", request_token.0);
        // TODO: PINの入力待ち

        let mut access_tokens = ("hoge", "huga"); // TODO: デフォルト値にする

        // while "" == access_tokens.0 {
        // ブラウザで認証ページを表示
        if webbrowser::open(&request_token.1.to_string()).is_err() {
            println!("Please access this URL: {}", &request_token.1);
        }

        print!("Enter the PIN(if available) or just hit enter.[PIN]:");
        // String pin = br.readLine()

        // if (pin.len() > 0) {
        //   accessToken = twitter.getOAuthAccessToken(requestToken, pin)
        // }
        // else {
        //   accessToken = twitter.getOAuthAccessToken()
        // }
        // }

        return access_tokens;
    }

    // TODO: AccessTokenを永続化
    fn store_access_tokens(&self) {
        // ファイルに保存
    }

    // アクセス可能なユーザ情報を取得
    fn verify_credentials(&self) -> User {
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

// TODO:ローカルに保存済みのアクセストークンを取得する
fn load_access_tokens() -> (&'static str, &'static str) {
    return ("localhoge", "localhuga");
}

#[derive(Debug, PartialEq)]
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
            println!("まだデータ無いよ");
            false
        } else {
            true
        }
    }
}
