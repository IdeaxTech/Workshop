# Supabase勉強会

python(FastAPI)×Supabaseを用いたバックエンドの勉強会

## プロジェクトの実行まで

1. このリポジトリをcloneする

```zsh
git clone https://github.com/IdeaxTech/Workshop.git
```

2. 必要なライブラリ等のインストール

```zsh
pip install -r requirements.txt
```

3. 環境変数の設定
- .envを作成
- SUPABASE_URL, SUPABASE_KEYを設定

4. 実行
```zsh
uvicorn main:app --reload
```

5. 実際にAPIリクエストを行う
- ターミナルで以下のコマンドを実行  

```zsh
curl -X POST "http://127.0.0.1:8000/users/" \
-H "Content-Type: application/json" \
-d '{"name": "sample", "email": "sample@example.com"}'
```

