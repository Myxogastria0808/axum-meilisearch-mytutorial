# Setup

1. docker-compose.yaml を作成

```yaml
version: "3.8"

services:
  meilisearch:
    container_name: meilisearch
    image: "getmeili/meilisearch:prototype-japanese-184"
    volumes:
      - ./meili_data:/meili_data
    environment:
      #meili cloudを使わない場合、マスターキーは自分で設定する
      - MEILI_MASTER_KEY=master_key
      - MEILI_ENV=development
    ports:
      #データ部分をマウント
      - "7700:7700"
    tty: true
```

2. docker-compose up -d

```sh
docker-compose up -d
```

3. Default Admin API Key を.env ファイルに書く

Default Admin API Key を調べる

```sh
curl   -X GET 'http://localhost:7700/keys'   -H 'Authorization: Bearer <master_key>' | jq
```

.env を作成し、以下の内容を書く

```.env
MEILI_URL=http://localhost:7700
ADMIN_API_KEY=<master_key>

```

4. runner 関数を実行し、meilisearch にデータを与える

5. api 関数に切り替えて実行する
