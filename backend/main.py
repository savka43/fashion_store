from __future__ import annotations

import base64
import hashlib
import hmac
import json
import os
import secrets
import sqlite3
from contextlib import contextmanager
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any

from fastapi import Depends, FastAPI, Header, HTTPException, status
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import FileResponse
from pydantic import BaseModel, EmailStr, Field

ROOT_DIR = Path(__file__).resolve().parent.parent
DATA_DIR = ROOT_DIR / "data"
DATABASE_PATH = Path(os.getenv("DATABASE_PATH", DATA_DIR / "noir_atelier.sqlite"))
FRONTEND_DIST = ROOT_DIR / "frontend-yew" / "dist"
JWT_SECRET = os.getenv("JWT_SECRET_KEY", "noir-atelier-dev-secret")
JWT_ISSUER = "noir-atelier-api"
JWT_AUDIENCE = "noir-atelier-client"
JWT_EXPIRE_MINUTES = int(os.getenv("JWT_EXPIRE_MINUTES", "1440"))

DATABASE_PATH.parent.mkdir(parents=True, exist_ok=True)

app = FastAPI(title="NOIR ATELIER API", version="1.0.0")
app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "http://localhost:8082",
        "http://127.0.0.1:8082",
        "http://localhost:8083",
        "http://127.0.0.1:8083",
        "http://localhost:8000",
        "http://127.0.0.1:8000",
    ],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


class AuthRequest(BaseModel):
    email: EmailStr
    password: str = Field(min_length=6, max_length=128)


class RegisterResponse(BaseModel):
    message: str


class LoginResponse(BaseModel):
    token: str
    email: str


class OutfitCreate(BaseModel):
    title: str = Field(min_length=2, max_length=140)
    content: str = Field(min_length=2, max_length=2000)
    occasion: str = Field(min_length=2, max_length=80)


class OutfitUpdate(OutfitCreate):
    pass


class OutfitOut(BaseModel):
    id: int
    title: str
    content: str
    occasion: str
    is_published: bool = Field(serialization_alias="isPublished")
    created_at: str = Field(serialization_alias="createdAt")
    author_email: str = Field(serialization_alias="authorEmail")


@contextmanager
def db() -> sqlite3.Connection:
    conn = sqlite3.connect(DATABASE_PATH)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA foreign_keys = ON")
    try:
        yield conn
        conn.commit()
    finally:
        conn.close()


def init_db() -> None:
    with db() as conn:
        conn.execute(
            """
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            """
        )
        conn.execute(
            """
            CREATE TABLE IF NOT EXISTS outfits (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                occasion TEXT NOT NULL,
                is_published INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                owner_id INTEGER NOT NULL,
                FOREIGN KEY(owner_id) REFERENCES users(id) ON DELETE CASCADE
            )
            """
        )
        conn.execute("CREATE INDEX IF NOT EXISTS idx_outfits_public ON outfits(is_published, created_at)")
        conn.execute("CREATE INDEX IF NOT EXISTS idx_outfits_owner ON outfits(owner_id, created_at)")


@app.on_event("startup")
def on_startup() -> None:
    init_db()


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def hash_password(password: str) -> str:
    salt = secrets.token_bytes(16)
    digest = hashlib.pbkdf2_hmac("sha256", password.encode("utf-8"), salt, 200_000)
    return f"pbkdf2_sha256${base64.b64encode(salt).decode()}${base64.b64encode(digest).decode()}"


def verify_password(password: str, stored: str) -> bool:
    try:
        algorithm, salt_b64, digest_b64 = stored.split("$", 2)
        if algorithm != "pbkdf2_sha256":
            return False
        salt = base64.b64decode(salt_b64)
        expected = base64.b64decode(digest_b64)
    except ValueError:
        return False
    actual = hashlib.pbkdf2_hmac("sha256", password.encode("utf-8"), salt, 200_000)
    return hmac.compare_digest(actual, expected)


def b64url(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode("ascii")


def b64url_decode(value: str) -> bytes:
    padding = "=" * (-len(value) % 4)
    return base64.urlsafe_b64decode(value + padding)


def create_token(user_id: int, email: str) -> str:
    header = {"alg": "HS256", "typ": "JWT"}
    now = datetime.now(timezone.utc)
    payload = {
        "iss": JWT_ISSUER,
        "aud": JWT_AUDIENCE,
        "sub": str(user_id),
        "email": email,
        "iat": int(now.timestamp()),
        "exp": int((now + timedelta(minutes=JWT_EXPIRE_MINUTES)).timestamp()),
    }
    signing_input = f"{b64url(json.dumps(header, separators=(',', ':')).encode())}.{b64url(json.dumps(payload, separators=(',', ':')).encode())}"
    signature = hmac.new(JWT_SECRET.encode("utf-8"), signing_input.encode("ascii"), hashlib.sha256).digest()
    return f"{signing_input}.{b64url(signature)}"


def decode_token(token: str) -> dict[str, Any]:
    try:
        header_b64, payload_b64, signature_b64 = token.split(".", 2)
        signing_input = f"{header_b64}.{payload_b64}"
        expected = hmac.new(JWT_SECRET.encode("utf-8"), signing_input.encode("ascii"), hashlib.sha256).digest()
        actual = b64url_decode(signature_b64)
        if not hmac.compare_digest(actual, expected):
            raise ValueError("bad signature")
        payload = json.loads(b64url_decode(payload_b64))
    except Exception as exc:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Некорректный JWT.") from exc

    if payload.get("iss") != JWT_ISSUER or payload.get("aud") != JWT_AUDIENCE:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Некорректный JWT.")
    if int(payload.get("exp", 0)) < int(datetime.now(timezone.utc).timestamp()):
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Срок действия JWT истёк.")
    return payload


def current_user(authorization: str | None = Header(default=None)) -> sqlite3.Row:
    if not authorization:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Требуется Authorization: Bearer.")
    scheme, _, token = authorization.partition(" ")
    if scheme.lower() != "bearer" or not token:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Некорректный заголовок Authorization.")
    payload = decode_token(token)
    with db() as conn:
        user = conn.execute("SELECT id, email FROM users WHERE id = ?", (payload["sub"],)).fetchone()
    if user is None:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Пользователь не найден.")
    return user


def outfit_to_out(row: sqlite3.Row) -> OutfitOut:
    return OutfitOut(
        id=row["id"],
        title=row["title"],
        content=row["content"],
        occasion=row["occasion"],
        is_published=bool(row["is_published"]),
        created_at=row["created_at"],
        author_email=row["author_email"],
    )


def get_owned_outfit(conn: sqlite3.Connection, outfit_id: int, user_id: int) -> sqlite3.Row:
    row = conn.execute(
        """
        SELECT o.*, u.email AS author_email
        FROM outfits o
        JOIN users u ON u.id = o.owner_id
        WHERE o.id = ? AND o.owner_id = ?
        """,
        (outfit_id, user_id),
    ).fetchone()
    if row is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Запись не найдена.")
    return row


@app.get("/api/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


@app.post("/api/auth/register", response_model=RegisterResponse, status_code=status.HTTP_201_CREATED)
def register(request: AuthRequest) -> RegisterResponse:
    email = request.email.lower()
    with db() as conn:
        try:
            conn.execute(
                "INSERT INTO users (email, password_hash, created_at) VALUES (?, ?, ?)",
                (email, hash_password(request.password), now_iso()),
            )
        except sqlite3.IntegrityError as exc:
            raise HTTPException(status_code=409, detail="Пользователь с таким email уже существует.") from exc
    return RegisterResponse(message="Пользователь зарегистрирован. Теперь выполните вход.")


@app.post("/api/auth/login", response_model=LoginResponse)
def login(request: AuthRequest) -> LoginResponse:
    email = request.email.lower()
    with db() as conn:
        user = conn.execute("SELECT id, email, password_hash FROM users WHERE email = ?", (email,)).fetchone()
    if user is None or not verify_password(request.password, user["password_hash"]):
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Неверный email или пароль.")
    return LoginResponse(token=create_token(user["id"], user["email"]), email=user["email"])


@app.get("/api/outfits", response_model=list[OutfitOut])
def public_outfits() -> list[OutfitOut]:
    with db() as conn:
        rows = conn.execute(
            """
            SELECT o.*, u.email AS author_email
            FROM outfits o
            JOIN users u ON u.id = o.owner_id
            WHERE o.is_published = 1
            ORDER BY o.created_at DESC
            """
        ).fetchall()
    return [outfit_to_out(row) for row in rows]


@app.get("/api/outfits/mine", response_model=list[OutfitOut])
def my_outfits(user: sqlite3.Row = Depends(current_user)) -> list[OutfitOut]:
    with db() as conn:
        rows = conn.execute(
            """
            SELECT o.*, u.email AS author_email
            FROM outfits o
            JOIN users u ON u.id = o.owner_id
            WHERE o.owner_id = ?
            ORDER BY o.created_at DESC
            """,
            (user["id"],),
        ).fetchall()
    return [outfit_to_out(row) for row in rows]


@app.post("/api/outfits", response_model=OutfitOut, status_code=status.HTTP_201_CREATED)
def create_outfit(payload: OutfitCreate, user: sqlite3.Row = Depends(current_user)) -> OutfitOut:
    with db() as conn:
        cursor = conn.execute(
            """
            INSERT INTO outfits (title, content, occasion, is_published, created_at, owner_id)
            VALUES (?, ?, ?, 0, ?, ?)
            """,
            (payload.title.strip(), payload.content.strip(), payload.occasion.strip(), now_iso(), user["id"]),
        )
        row = get_owned_outfit(conn, cursor.lastrowid, user["id"])
    return outfit_to_out(row)


@app.put("/api/outfits/{outfit_id}", response_model=OutfitOut)
def update_outfit(outfit_id: int, payload: OutfitUpdate, user: sqlite3.Row = Depends(current_user)) -> OutfitOut:
    with db() as conn:
        get_owned_outfit(conn, outfit_id, user["id"])
        conn.execute(
            "UPDATE outfits SET title = ?, content = ?, occasion = ? WHERE id = ? AND owner_id = ?",
            (payload.title.strip(), payload.content.strip(), payload.occasion.strip(), outfit_id, user["id"]),
        )
        row = get_owned_outfit(conn, outfit_id, user["id"])
    return outfit_to_out(row)


@app.delete("/api/outfits/{outfit_id}")
def delete_outfit(outfit_id: int, user: sqlite3.Row = Depends(current_user)) -> dict[str, str]:
    with db() as conn:
        get_owned_outfit(conn, outfit_id, user["id"])
        conn.execute("DELETE FROM outfits WHERE id = ? AND owner_id = ?", (outfit_id, user["id"]))
    return {"message": "Запись удалена."}


@app.post("/api/outfits/{outfit_id}/publish", response_model=OutfitOut)
def publish_outfit(outfit_id: int, user: sqlite3.Row = Depends(current_user)) -> OutfitOut:
    with db() as conn:
        get_owned_outfit(conn, outfit_id, user["id"])
        conn.execute("UPDATE outfits SET is_published = 1 WHERE id = ? AND owner_id = ?", (outfit_id, user["id"]))
        row = get_owned_outfit(conn, outfit_id, user["id"])
    return outfit_to_out(row)


@app.post("/api/outfits/{outfit_id}/unpublish", response_model=OutfitOut)
def unpublish_outfit(outfit_id: int, user: sqlite3.Row = Depends(current_user)) -> OutfitOut:
    with db() as conn:
        get_owned_outfit(conn, outfit_id, user["id"])
        conn.execute("UPDATE outfits SET is_published = 0 WHERE id = ? AND owner_id = ?", (outfit_id, user["id"]))
        row = get_owned_outfit(conn, outfit_id, user["id"])
    return outfit_to_out(row)


@app.get("/{path:path}")
def spa_fallback(path: str) -> FileResponse:
    candidate = FRONTEND_DIST / path
    if path and candidate.is_file():
        return FileResponse(candidate)
    index = FRONTEND_DIST / "index.html"
    if index.is_file():
        return FileResponse(index)
    raise HTTPException(status_code=404, detail="Frontend build not found. Run `trunk build --release`.")
