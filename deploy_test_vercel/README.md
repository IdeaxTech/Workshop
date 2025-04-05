# Next.jsで作ったwebアプリを公開してみよう

## このプロジェクトを動かす
- このリポジトリをcloneする

```zsh
git clone https://github.com/IdeaxTech/Workshop
```

- リポジトリ内のdeploy_test_vercelに移動

```zsh
cd Workshop/deploy_test_vercel
```

- ローカルでの動作確認

1. パッケージ関連をインストール

```zsh
npm i
```

2. 実行確認

```zsh
npm run dev
```

## 実際に公開してみよう

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
5. Vercel(https://vercel.com/)のサイトを開く

6. GitHubと連携を行う。
7. Add new→projectを選択し、作成したリポジトリを選択後デプロイ成功

8. deploy成功後リポジトリにURLが紐づけられているのでクリック

## コマンドでdeployしてみよう

- 動作確認後、以下のコマンドを実行

```zsh
vercel login
```

- vercelにログインを成功させたら以下のコマンドを実行

```zsh
vercel
```

- このコマンドを実行し、いくつかの質問に答えたらdeploy成功。これ以降は以下のコマンドでデプロイ可能

```zsh
vercel --prod
```




