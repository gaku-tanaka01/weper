# weper_cli

`weper_cli` は、職種、勤務地、取得件数を指定して求人情報をCSV形式で取得するコマンドラインツールです。

## 使い方

基本的なコマンド形式は以下の通りです:

weper_cli run --job <職種変数名> --area <勤務地> --count <取得件数>

### オプション

- `--job <職種変数名>`: 職種を指定します。以下の変数名を使用できます:
  - `ENGINEER_SYS_NET`: エンジニア・技術職（システム/ネットワーク）
  - `CREATIVE_WEB`: クリエイティブ職（Web）
  - `CREATIVE_GAME_MEDIA`: クリエイティブ職（ゲーム/マルチメディア）
  - `PLAN_MARKETING`: 企画・マーケティング職
  - `SALES`: 営業職
  - `MGMT_CXO`: 経営・CxO職
  - `ACCOUNT_ADMIN_BACK`: 経理・管理・バックオフィス職
  - `ASSIST_OFFICE`: アシスタント・事務職・オフィスワーク
  - `SERVICE_PERSONNEL`: サービス職（人材/店舗/医療）
  - `SPECIALIST_FINANCE`: 専門職（金融/不動産/コンサルタント/士業）
  - `ENGINEER_ELEC_MECH`: エンジニア・技術職（電気/電子/機械/半導体）
  - `ARCH_CIVIL_PLANT`: 建築設計・土木・プラント職

- `--area <勤務地>`: 勤務地をアルファベットで指定します。大文字小文字は区別されません。特殊な勤務地として `full_remote` (フルリモート)、`overseas` (海外) も指定可能です。

- `--count <取得件数>`: 取得する求人の件数を指定します。1桁以下の数値は切り捨てられます。

### 例：Windowsの場合　weper_cli.exeがあるディレクトリまで行き

```shell
.\weper_cli.exe run --area=tokyo --job=engineer_sys_net --count=1000
```

### 例：Linuxの場合 web_cliがあるディレクトリまで行き

```shell
weper_cli run --area=tokyo --job=engineer_sys_net --count=1000
```

### 出力

コマンドを実行すると、指定した勤務地、職種、
件数をもとに `指定勤務地_指定職種_指定件数_日付.csv` という名前のファイルが生成されます。このファイルには会社名、求人名、オファーの詳細リンクが記載されます。