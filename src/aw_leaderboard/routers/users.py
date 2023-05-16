from typing import Annotated, Optional

from fastapi import (
    APIRouter,
    Depends,
    Form,
    HTTPException,
    Request,
    status,
)
from fastapi.security import OAuth2PasswordBearer, OAuth2PasswordRequestForm
from fastapi.security.utils import get_authorization_scheme_param

from ..db import Database, UserExistsError, get_db
from ..models import Token, User
from ..secrets import create_access_token, jwt_decode

router = APIRouter(prefix="/users", tags=["users"])

# OAuth2 with password (and hashing), Bearer with JWT tokens
oauth2_scheme = OAuth2PasswordBearer(tokenUrl="token")


# Same as above, but returns None if no token is provided
async def optional_oauth2_scheme(request: Request):
    authorization: Optional[str] = request.headers.get("Authorization")
    scheme, param = get_authorization_scheme_param(authorization)
    if not authorization or scheme.lower() != "bearer":
        return None
    return param


TokenDep = Annotated[str, Depends(oauth2_scheme)]
TokenOptDep = Annotated[Optional[str], Depends(optional_oauth2_scheme)]
DatabaseDep = Annotated[Database, Depends(get_db)]


@router.post("/register", response_model=User)
async def register_user(
    db: DatabaseDep,
    username: str = Form(...),
    password: str = Form(...),
    email: str = Form(...),
) -> User:
    print(f"Registering user {username} with email {email}")
    try:
        return (await db.create_user(username, password, email)).dict()
    except UserExistsError as e:
        raise HTTPException(
            status_code=status.HTTP_409_CONFLICT,
            detail=e.message,
        )


@router.post("/token", response_model=Token)
async def login_for_access_token(
    form_data: Annotated[OAuth2PasswordRequestForm, Depends()],
    db: DatabaseDep,
) -> Token:
    user = await db.authenticate_user(form_data.username, form_data.password)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect username or password",
            headers={"WWW-Authenticate": "Bearer"},
        )
    access_token = create_access_token(user.dict().get("username"))
    return {"access_token": access_token, "token_type": "bearer"}


async def get_current_user(token: TokenDep, db: DatabaseDep) -> User:
    user = await db.get_user_by_username(token.username)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid authentication credentials",
            headers={"WWW-Authenticate": "Bearer"},
        )
    return user


async def get_current_user_opt(token: TokenOptDep, db: DatabaseDep) -> Optional[User]:
    if token is None:
        return None
    token_data = jwt_decode(token)
    user = await db.get_user_by_username(token_data.username)
    return user
