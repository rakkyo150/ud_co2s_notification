# ud_co2s_notification
UD-CO2Sの酸素濃度値とその推移を定期的にDiscordに通知できるスクリプト  
現状では0時0分にデータをリセットして、そこから実行するごとにデータ生成＆グラフ作成を行う処理になっています。  

## 注意
[こちら](https://github.com/rakkyo150/ud-co2s-with-raspberry-pi-and-alexa)と同じく、https://scrapbox.io/oquno/UD-CO2S や https://gist.github.com/oquno/d07f6dbf8cc760f2534d9914efe79801 を参考にしましたが、正直理解の程度は低いです。  
お試しになる際は、ライセンスにある通り、自己責任でお願いします。
また、自分の環境はRaspberry Pi 4なのでそれ以外の環境では動作するか不明です。

## 前提となる環境
Rustによるコンパイルが可能な環境が必要です(`rustc -V`や`cargo -V`で確認できます)。  
スクリプトを実行するラズパイにUD-CO2Sを接続してください。  

## 設定方法
#### クローンします
```bash
$ git clone https://github.com/rakkyo150/ud_co2s_notification
$ cd ud_co2s_notification
```
#### .envファイル作成
次に`.env`ファイルを`ud_co2s_notification`上に作成します。  
`.env`ファイル内では`WEBHOOK_URL`と`DATA_FILE_PATH`の設定をしてください。  
`WEBHOOK_URL`はDiscordのチャンネルの編集->連携サービス->ウェブフックからWebhookを新規作成すればURLを取得できます。  
`DATA_FILE_PATH`ではデータを保存する場所を指定することになります。  
vimなら以下の通りです。
```bash
$ vim .env
# .envの中身
WEBHOOK_URL="https://discord.com/api/webhooks/XXXXXXXXX/XXXXXXXXX/XXXXXXXXXXXXXXXXXXXXXXXX"
DATA_FILE_PATH="/home/user/hogehoge.json"
```
vimなら`esc`->`:wq`で上書き保存すれば完了です。  

#### テスト＆ビルド
```bash
$ cargo run
```
これでテストとビルドの両方ができます。  
Discordに現在の酸素濃度が送られてきたら成功です。  
ちなみに、初回実行時はグラフに何もプロットされないので注意してください。

#### 定期実行の設定
ラズパイなら以下のコマンドから実行間隔をcronで指定することになります。  
実行コマンドでは`cargo run`ではなくビルドされたファイルを実行するコマンドでないと正常に動作しないので注意してください。  
```bash
$ crontab -e
# cronの例
# 毎時0分と30分ごとに実行し、/tmp/ud_co2s.logで出力のログを確認できる
0,30 * * * * cd /home/user/ud_co2s_notification; ./target/debug/ud_co2s_notification >> /tmp/ud_co2s.log
```
