from fastapi import FastAPI, HTTPException
from supabase import create_client, Client
from pydantic import BaseModel

# FastAPIアプリケーション
app = FastAPI()

# Supabaseの設定
SUPABASE_URL = ""
SUPABASE_KEY = ""
supabase: Client = create_client(SUPABASE_URL. SUPABASE_KEY)

# Pydanticモデル(ユーザー情報)
class User(BaseModel):
    name: str
    email: str

# ユーザーの作成(POST)
@app.post("/users/")
async def create_user(user: User):
    try:
        response = supabase.table("users").insert({"name": user.name, "email": user.email }).execute()
        return {"message": "ユーザーが正常に作成されました", "user": response.data}
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))

# 全ユーザの取得(GET)
@app.get("/users/")
async def get_users():
    try:
        response = supabase.table("users").select("*").execute()
        return {"users": response.data}
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))

# IDによるユーザの取得(GET)
@app.get("/users/{user_id}")
async def get_user(user_id: int):
    try:
        response = supabase.table("users").select("*").eq("id", user_id).execute()
        if not response.data:
            raise HTTPException(status_code=404, detail="ユーザが見つかりません")
        return {"user": response.data[0]}
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))

# ユーザーの削除(DELETE)
@app.delete("/users/{user_id}")
async def delete_user(user_id: int):
    try:
        response = supabase.table("users").delete().eq("id", user_id).execute()
        if not response.data:
            raise HTTPException(status_code=404, detail="ユーザーが見つかりません")
        return {"message": "ユーザーが正常に削除されました"}
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))

# アプリケーションを実行するには: uvicorn main:app --reload
