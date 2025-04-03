from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

app = FastAPI()

# "static"ディレクトリを静的ファイルとしてマウント
app.mount("/", StaticFiles(directory="static", html=True), name="static")
