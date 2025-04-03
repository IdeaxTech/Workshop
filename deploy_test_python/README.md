# Pythonで作ったwebアプリを公開してみよう

## このプロジェクトを動かす
- このリポジトリをcloneする

```zsh
git clone https://github.com/IdeaxTech/Workshop
```

- リポジトリ内のdeploy_test_pythonに移動

```zsh
cd Workshop/deploy_test_python
```

- ローカルでの動作確認

1. requirements.txtをインストール

```zsh
pip install -r requirements.txt
```

2. 実行確認

```zsh
uvicorn app:app --reload
```

## 実際に公開してみよう(Render編)

1. GitHubに自分のリポジトリを作成(Public以外選択しない)
2. クローンしたプロジェクトのリモートリポジトリを作成したリポジトリに変更

```zsh
git remote -rf origin
git remote add origin (リポジトリのURL)
```

3. リポジトリにpush

```zsh
git push origin main
```

4. リポジトリに公開されたのを確認
5. Render(https://render.com/)のサイトを開く

6. GitHubと連携を行う。
7. Add newを選択し、作成したリポジトリを選択
8. 設定を変更し、完了する(以下の設定に)

```設定
Language	Python 3
Build Command	pip install -r requirements.txt
Start Command	uvicorn app:app --host 0.0.0.0 --port $PORT (任意のポートで大丈夫です)
```

9. 変更後✅Liveになっていれば公開成功

実際に公開しているURL → https://workshop-j799.onrender.com



