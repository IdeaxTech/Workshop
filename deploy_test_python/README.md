# Python で作ったwebアプリを公開してみよう

## 公開までの手順
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


