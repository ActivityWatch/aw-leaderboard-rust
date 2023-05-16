from fastapi import (
    Depends,
    FastAPI,
    HTTPException,
    Request,
)
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates

from .db import Database, get_db
from .models import Event, User
from .routers.leaderboard import router as leaderboard_router
from .routers.users import get_current_user, get_current_user_opt
from .routers.users import router as users_router

# Define your app
app = FastAPI()
app.include_router(users_router)
app.include_router(leaderboard_router)
app.mount("/static", StaticFiles(directory="static"), name="static")

# TODO: replace jinj2 templates with a proper Vue 3 frontend
templates = Jinja2Templates(directory="templates")


@app.get("/", response_class=HTMLResponse)
async def get_index(
    request: Request,
    db: Database = Depends(get_db),
    current_user: User = Depends(get_current_user_opt),
):
    return templates.TemplateResponse(
        "index.html", {"request": request, "user": current_user}
    )


@app.get("/login", response_class=HTMLResponse)
async def get_login(request: Request):
    return templates.TemplateResponse("login.html", {"request": request})


@app.get("/signup", response_class=HTMLResponse)
async def get_signup(request: Request):
    return templates.TemplateResponse("signup.html", {"request": request})


@app.post("/events")
async def upload_events(
    events: list[Event],
    db: Database = Depends(get_db),
    user: User = Depends(get_current_user),
):
    events = [event for event in events]
    db.insert_events(events, user.username)
    await db.commit()

    return {"message": "Events uploaded successfully"}
