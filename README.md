# NOIR ATELIER WebAssembly

Лабораторные работы №2 и №3: перенос статического сайта магазина одежды в клиентское WebAssembly-приложение без Blazor и подключение REST API.

Стек: Rust + Yew + Trunk для WASM-клиента, FastAPI + SQLite для REST API.

## Что перенесено из ЛР1

| Раздел | Было в статике | Стало в WASM-приложении |
| --- | --- | --- |
| Главная | `index.html` | маршрут `/` |
| Каталог | `catalog.html`, поиск, избранное | маршрут `/catalog`, поиск и избранное через `localStorage` |
| Лукбук | `lookbook.html` | маршрут `/lookbook` |
| О бренде | `about.html`, черновик заявки | маршрут `/about`, черновик через `localStorage` |
| Новый интерактив | отсутствовал | маршрут `/calculator`, расчёты в Rust/WASM |

## Архитектура

- `frontend-yew/index.html` — точка входа Rust/Yew клиента.
- `frontend-yew/src/app.rs` — маршруты приложения.
- `frontend-yew/src/components/` — общий каркас, навигация, тема оформления.
- `frontend-yew/src/pages/` — страницы магазина.
- `frontend-yew/src/calculator.rs` — логика клиентского калькулятора образа, компилируемая в WASM.
- `frontend-yew/src/services/storage.rs` — работа с `localStorage`.
- `frontend-yew/static/styles.css` — перенесённые глобальные стили, включая адаптацию под `portrait` и `landscape`.
- `backend/main.py` — REST API: register/login, JWT, CRUD fashion-записей, публикация.
- `data/noir_atelier.sqlite` — локальная SQLite база, создаётся автоматически.

## ЛР3: REST API

| Метод | Путь | Auth | Назначение |
| --- | --- | --- | --- |
| POST | `/api/auth/register` | нет | регистрация пользователя |
| POST | `/api/auth/login` | нет | вход и получение JWT |
| GET | `/api/outfits` | нет | опубликованные образы |
| GET | `/api/outfits/mine` | Bearer | мои черновики и публикации |
| POST | `/api/outfits` | Bearer | создать черновик |
| PUT | `/api/outfits/{id}` | Bearer | редактировать свою запись |
| DELETE | `/api/outfits/{id}` | Bearer | удалить свою запись |
| POST | `/api/outfits/{id}/publish` | Bearer | опубликовать |
| POST | `/api/outfits/{id}/unpublish` | Bearer | снять с публикации |

Данные сохраняются через параметризованные SQLite-запросы (`?`), без SQL-конкатенации пользовательского ввода.

## Сборка и запуск

Для ЛР2 отдельно:

```bash
cd frontend-yew
trunk serve
```

Для ЛР3:

```bash
cd frontend-yew
trunk build --release
cd ..
python3 -m venv .venv
.venv/bin/pip install -r backend/requirements.txt
.venv/bin/uvicorn backend.main:app --host 127.0.0.1 --port 8000
```

Открыть клиент: `http://127.0.0.1:8000`.

Swagger API: `http://127.0.0.1:8000/docs`.
