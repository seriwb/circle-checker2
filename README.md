# Circle Checker2

**「やばい！サークルチェック全然してない！」**

自分がイベント直前に陥るいつものやつを少しでも緩和するために作った、
Twitterのユーザ名からイベント参加情報を抽出するツールです。

設定ファイル（filter.txt）をいじることで、どんなパターンにも対応できます。

> 以前作成した[Circle Checker](https://github.com/seriwb/circle-checker)の改良版です


## 使い方

### 事前準備

1. [releases](https://github.com/seriwb/circle-checker2/releases/latest)からzipファイルを取得し、適当な場所に展開
2. リストを対象にする場合は、config/config.tomlの`target_list = [""]`にチェックするリスト名を入力  
（フォローユーザを対象にする場合は未入力のままにしておく）
3. config/filter.txtにユーザ名に含まれる可能性のあるイベント名を列挙  
（デフォルトはコミケ用で、イベント名指定には正規表現が利用可能です） 
> 以下のファイルをfilter.txtにリネームすることで、特定イベントのチェックができます。
> - filter_comiket.txt : コミケ用のフィルターサンプルです
> - filter_comitia.txt : コミティア用のフィルターサンプルです

#### 設定値のサンプル

```groovy
target_list = [""]                       // タイムラインの情報を取得
target_list = ["", "リスト名"]           // タイムラインと「リスト名」の情報を取得
target_list = ["リスト１", "リスト１"]   // 「リスト１」と「リスト２」の情報を取得
```


### 実行手順

コンソールからzipファイルを展開した場所まで移動し、以下のコマンドを実行してください。

```
./circle-checker
```

初回はTwitter認証のページがデフォルトブラウザで開くので、認証後表示されたPINをコンソールに入力してください。

※Twitterアカウント情報をリセットしたい場合は、tokenファイルを削除してください。


### 注意

filter.txtはUTF-8形式で保存してください。  
イベント名の指定には正規表現が利用できますが、()を使うとエラーになるため、()は使用しないようお願いします。

また、実行後に以下のようなメッセージが表示された場合は、Twitterの規制にひっかかっていますので、
15分以上待ってから再度実行してください。

```
Rate limit exceeded
```


## 機能

- タイムラインと複数のリストを同時にチェックすることができます
- 固定ツイートの情報を取得できます
- 出力結果をヘッダーと一緒にGoogle Spreadsheetsに貼り付けることで、アイコンと固定ツイートの画像が表示されます
  - ヘッダー：`Twitter ID	Twitter Name	アイコン	一致イベント名	スペース番号	画像1	画像2	画像3	画像4	Twitter URL	Twitter Link	固定されたツイート	プロフィール画像	固定されたツイートの画像`



## 今後の予定

- TLに流れてくるRTを対象にするオプション追加


## 要望＆バグ報告

要望やバグ報告などあれば、メール、GitHubのIssue、ブログへのコメントなどでご連絡ください。


## License

MIT License



# 開発情報

- Rustのインストール

## 各種手順

```
# ローカル実行
cargo run

# ビルド
cargo build

# リリースビルド
cargo build -r
```
