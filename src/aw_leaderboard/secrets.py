import os
from datetime import datetime, timedelta
from typing import Any

import jwt
from pydantic import BaseModel

# Secret key for encoding and decoding JWT
# In a production-level application, you would want to keep this in a secure place, not in the code
SECRET_KEY = os.environ.get("SECRET_KEY", None)
assert (
    SECRET_KEY is not None
), "No secret key set for JWT authentication! Set environment variable SECRET_KEY"
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 120


class TokenData(BaseModel):
    username: str


def create_access_token(username: str):
    to_encode: dict[str, Any] = {"sub": username}
    to_encode.update(
        {"exp": datetime.utcnow() + timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES)}
    )
    encoded_jwt = jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)
    return encoded_jwt


def jwt_decode(token: str) -> TokenData:
    payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
    username: str = payload.get("sub")
    if username is None:
        raise Exception("Invalid token")
    return TokenData(username=username)
