from pydantic import BaseModel


class User(BaseModel):
    username: str
    email: str
    hashed_password: str
    disabled: bool = False


class Event(BaseModel):
    timestamp: str
    duration: int
    data: dict


class Token(BaseModel):
    access_token: str
    token_type: str
