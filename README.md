# weper_cli

## 概要
`weper_cli`は、指定された職種や地域の求人情報を取得するためのCLIツールです。

## weper_cliの実行ファイルの場所

### Linuxの場合
https://github.com/gaku-tanaka01/weper/blob/main/target/release/weper_cli

### Windowsの場合
https://github.com/gaku-tanaka01/weper/blob/main/target/x86_64-pc-windows-gnu/release/weper_cli.exe

## 基本的な使い方
ツールを実行するには、以下のコマンド構文を使用します:

### Linuxの場合
```bash
weper_cli run --main-job <main job tag> --sub-job <sub job tag> --area <prefecture> --count <count>
```

### Windowsの場合
```bash
.\weper_cli.exe run --main-job <main job tag> --sub-job <sub job tag> name--area <prefecture> --count <count>
```

### オプション
- `--main-job <main job tag>`  
  主な職種カテゴリを指定します。 `sub-job`と同時に指定はできません。
- `--sub-job <sub job tag>`  
  副職種カテゴリを指定します。 `main-job`と同時に指定はできません。
- `--area <area_id>`  
  地理的なエリアを指定します。  
  `main-job`または`sub-job`の指定がないとき、自動で東京エリアが指定されます。

- `--count <count>`  
  取得する結果の数を指定します。10桁単位で入力してください　1桁以下は切り捨てされます。


### 主な職種カテゴリ (`main-job`)
主な職種カテゴリの値には以下が含まれます:
- `engineer` エンジニア・技術職（システム/ネットワーク）
- `creative_web` クリエイティブ職（Web）
- `creative_game` クリエイティブ職（ゲーム/マルチメディア）
- `planning_marketing` 企画・マーケティング職
- `sales` 営業職
- `management_cxo` 経営・CxO職
- `account_admin_back_office` 経理・管理・バックオフィス職
- `assistant_office_work` アシスタント・事務職・オフィスワーク
- `service_personnel` サービス職（人材/店舗/医療）
- `specialist_finance` 専門職（金融/不動産/コンサルタント/士業）
- `engineer_elec_mech` エンジニア・技術職（電気/電子/機械/半導体）
- `architect_civil_plant` 建築設計・土木・プラント職

### 副職種カテゴリ (`sub-job`)
副職種カテゴリの値には以下が含まれます:
- `backend` バックエンドエンジニア
- `frontend` フロントエンドエンジニア
- `smartphone_app` スマートフォンアプリエンジニア
- `system_dev_general` システム開発（汎用系）
- `system_dev_embedded` システム開発（制御・組み込み）
- `infra_engineer` インフラエンジニア
- `net_monitor` ネットワーク/サーバ監視・運用・保守・技術サポート
- `network_engineer` ネットワークエンジニア
- `package_dev` パッケージソフト・ミドルウェア開発
- `project_manager` プロジェクトマネージャー（PM）・リーダー
- `corp_sys_engineer` 社内情報システムエンジニア（社内SE）
- `product_manager` プロダクトマネージャー
- `it_consultant` ITコンサルタント・プリセールス
- `data_scientist` データサイエンティスト
- `qa_engineer` QAエンジニア・品質保証
- `other_sys` その他システム関連
- `web_designer` Webデザイナー
- `ui_ux_designer` UI/UXデザイナー
- `web_coder` Webコーダー・HTMLコーダー
- `web_producer` Webプロデューサー・Webディレクター
- `web_content` Webサービス・コンテンツ企画
- `web_writer` Webライター・Web編集・コピーライター
- `marketing_pr` マーケ・広告宣伝・販促・PR
- `business_planning` 事業企画・事業統括
- `web_consultant_seo_sem` Webコンサルタント・SEO・SEM
- `product_planning` 商品企画・サービス企画
- `new_business_sales` 新規開拓営業
- `agency_sales` 代理店営業・パートナーセールス
- `planning_sales` 企画営業・アカウントプランナー
- `route_sales` ルートセールス
- `inside_sales` 内勤営業・インサイドセールス
- `sales_manager` 営業マネージャー・営業管理職
- `sales_planning` 営業企画
- `overseas_sales` 海外営業
- `tech_sales` セールスエンジニア・技術営業（FAE）
- `medical_sales` 医療営業（MR/MS）
- `customer_success` カスタマーサクセス
- `other_sales` その他営業関連
- `hr_admin` 人事・総務
- `finance_accounting` 財務・会計・経理
- `public_relations` 広報・IR
- `legal_compliance` 法務（コンプライアンス）・知的財産・特許
- `general_office` 一般事務・営業事務・庶務・秘書
- `interpreter_translator` 通訳・翻訳
- `logistics_trade` 物流管理・貿易事務
- `customer_service` カスタマーサービス（CS）・ユーザーサポート
- `other_office` その他受付・企画・事務関連
- `store_manager` 店長・販売・店舗管理・接客
- `supervisor_area_manager` スーパーバイザー（SV）・エリアマネージャー
- `medical_care` 医療・福祉・介護サービス
- `career_consultant` キャリアコンサルタント/コーディネーター・カウンセラー
- `purchasing_procurement` 購買・調達・MD・バイヤー・店舗開発
- `call_center_operator` コールセンター運営・オペレーター
- `other_service` その他サービス関連
- `finance_insurance_fp` 金融・保険・ファイナンシャルプランナー（FP）
- `real_estate_consultant` 不動産・住宅コンサルタント
- `strategy_consultant` 経営・戦略コンサルタント
- `accounting_consultant` 財務・会計コンサルタント
- `org_hr_consultant` 組織・人事コンサルタント
- `other_consultant` その他コンサル・専門職関連
- `production_tech_process_dev` 生産技術（設備）・生産管理・プロセス開発
- `auto_construction_machinery` 自動車・建機・輸送機器
- `analog_digital_semi` アナログ/デジタル半導体・システム・回路設計
- `precision_electronic_instrument` 精密機器・電子機器・計測機器
- `other_tech` その他技術関連
- `construction_management` 施工管理・環境調査・分析
- `survey_design_estimation` 測量・設計・積算・計装
- `other_arch_civil_plant` その他建築・土木・プラント関連

### 地域 (`area`)
地域の値には以下が含まれます:
- `tokyo` 東京都
- `kanagawa` 神奈川県
- `chiba` 千葉県
- `saitama` 埼玉県
- `ibaraki` 茨城県
- `tochigi` 栃木県
- `gunma` 群馬県
- `hokkaido` 北海道
- `aomori` 青森県
- `iwate` 岩手県
- `miyagi` 宮城県
- `akita` 秋田県
- `yamagata` 山形県
- `fukushima` 福島県
- `niigata` 新潟県
- `toyama` 富山県
- `ishikawa` 石川県
- `fukui` 福井県
- `yamanashi` 山梨県
- `nagano` 長野県
- `aichi` 愛知県
- `gifu` 岐阜県
- `shizuoka` 静岡県
- `mie` 三重県
- `osaka` 大阪府
- `hyogo` 兵庫県
- `kyoto` 京都府
- `shiga` 滋賀県
- `nara` 奈良県
- `wakayama` 和歌山県
- `tottori` 鳥取県
- `shimane` 島根県
- `okayama` 岡山県
- `hiroshima` 広島県
- `yamaguchi` 山口県
- `tokushima` 徳島県
- `kagawa` 香川県
- `ehime` 愛媛県
- `kochi` 高知県
- `fukuoka` 福岡県
- `saga` 佐賀県
- `nagasaki` 長崎県
- `kumamoto` 熊本県
- `oita` 大分県
- `miyazaki` 宮崎県
- `kagoshima` 鹿児島県
- `okinawa` 沖縄県
- `full_remote` フルリモート
- `overseas` 海外


## 例
東京でエンジニアのバックエンド職種を検索する場合のCLIツールの実行方法は以下の通りです:

### Linuxの場合
```bash
weper_cli run --sub-job backend --area tokyo --count 10
```

### Windowsの場合
```bash
.\weper_cli.exe run --sub-job backend --area tokyo --count 10
```