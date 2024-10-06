# fetch-data-from-steam

## 目的

Rustの学習の一環として、外部APIにリクエストを送信し、レスポンスの値を使った処理を行うコードを書くことにしました。

今回は Steam Web API を用いて、Steamのユーザー情報、フレンド情報、ゲーム情報などを取得し、それらのデータを表示するCLIアプリケーションを作成します。

## 実行方法

以下を実行する。

```sh
STEAM_API_KEY=${my_api_key} STEAM_USER_ID=${my_user_id} cargo run
```

## デバッグ情報の確認

- ログ出力は[env_logger](https://docs.rs/env_logger/latest/env_logger/)クレートを利用しています
- アプリケーション実行時に、`RUST_LOG=debug`を環境変数に設定するとデバッグログをCLIに出力できます

## Steam Web API

https://steamcommunity.com/dev

### 利用にはAPIキーの作成が必要

APIキーはこちらのページから作成します。

https://steamcommunity.com/dev/apikey

APIキーの作成には、[Steamガードモバイル認証](https://help.steampowered.com/ja/faqs/view/7EFD-3CAE-64D3-1C31)の設定と ドメイン名の設定が必要です。

### ユーザー情報の取得

[GetPlayerSummaries (v0002)](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetPlayerSummaries_.28v0002.29) を使用します。

### フレンドリストの取得

[GetFriendList (v0001)](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetFriendList_.28v0001.29) を使用します。

### 所持しているゲーム情報の取得

[GetOwnedGames (v0001)](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetOwnedGames_.28v0001.29) を使用します。
