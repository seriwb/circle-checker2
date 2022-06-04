#[derive(Debug)]
pub struct Twitter {
    consummer_key: String,
    consummer_secret: String,
    access_token: String,
    access_token_secret: String,
    account: User,
}
impl Twitter {
    pub fn new(consummer_key: impl Into<String>, consummer_secret: impl Into<String>) -> Twitter {
        let mut twitter = Twitter {
            consummer_key: consummer_key.into(),
            consummer_secret: consummer_secret.into(),
            access_token: "".to_string(),
            access_token_secret: "".to_string(),
            account: User {..Default::default()},
        };
        // getAccessToken
        // storeAccessToken
        twitter
    }
    pub fn authenticate(&mut self) {
        println!("Twitter Authenticate");

        // !isAuthorized()
        if false {

            // storeAccessToken()
        }
        println!("{}", self.consummer_key);
        println!("{}", self.consummer_secret);
        println!("{}", self.access_token);
        println!("{}", self.access_token_secret);

        // verifyCredentials() -> User
    }
    // isAuthorized()// 認証済みかどうかを確認する
    // getAccessToken()
    // storeAccessToken() //AccessTokenを永続化
    // verifyCredentials()// 認証可能なAccessTokenを保持しているかを確認 -> ユーザ情報
    // twitter.getUserLists(userinfo.getScreenName())
    // twitter.getUserListMembers(targetList.id, -1L)
    // twitter.getFriendsList(userinfo.id, -1L) // フォローユーザのリスト
}

#[derive(Debug)]
pub struct User {
  Name: String,
  ScreenName: String, //id
  URL: String,
}
impl Default for User {
  fn default() -> User {
      User {
          Name: "".to_string(),
          ScreenName: "".to_string(),
          URL: "".to_string(),
      }
  }
}
impl User {

}
