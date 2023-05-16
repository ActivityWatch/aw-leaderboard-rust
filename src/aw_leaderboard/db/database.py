from typing import Optional
from uuid import uuid4

from passlib.context import CryptContext
from sqlalchemy.exc import IntegrityError
from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine
from sqlalchemy.future import select
from sqlalchemy.orm import sessionmaker

from .models import Base, User

# TODO: Switch back to postgres for prod
# DATABASE_URL = "postgresql+asyncpg://user:password@localhost:5432/aw-leaderboard"
DATABASE_URL = "sqlite+aiosqlite:///./aw-leaderboard.db"

engine = create_async_engine(DATABASE_URL, future=True)
async_session = sessionmaker(engine, class_=AsyncSession, expire_on_commit=False)  # type: ignore

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")


async def get_db():
    db = await Database.create()
    print("db created")
    try:
        yield db
    finally:
        await db.close()


class UserExistsError(Exception):
    def __init__(self, username: str, email: str):
        self.username = username
        self.email = email
        self.message = f"User with username {username} or email {email} already exists"


class Database:
    def __init__(self, db: AsyncSession):
        self.db: AsyncSession = db

    @classmethod
    async def create(cls):
        async with engine.begin() as conn:
            await conn.run_sync(Base.metadata.create_all)
        async with async_session() as db:
            return cls(db)

    async def close(self):
        await self.db.close()

    async def commit(self):
        await self.db.commit()

    async def create_user(self, username: str, password: str, email: str):
        hashed_password = pwd_context.hash(password)
        user = User(
            id=str(uuid4()),
            email=email,
            hashed_password=hashed_password,
            username=username,
        )
        self.db.add(user)
        print("user created")
        try:
            await self.db.commit()
        except IntegrityError:
            raise UserExistsError(username, email)
        return user

    async def authenticate_user(
        self,
        username_or_email: str,
        password: str,
    ) -> Optional[User]:
        user = await self.get_user_by_username(
            username_or_email
        ) or await self.get_user_by_email(username_or_email)
        if not user:
            return None
        if not pwd_context.verify(password, user.hashed_password):
            return None
        return user

    async def get_user(self, user_id: str):
        result = await self.db.execute(select(User).filter(User.id == user_id))
        user = result.scalars().first()
        return user

    async def get_user_by_username(self, username: str):
        result = await self.db.execute(select(User).where(User.username == username))
        user = result.scalars().first()
        if user:
            return User(**user.__dict__)
        else:
            return None

    async def get_user_by_email(self, email: str):
        result = await self.db.execute(select(User).filter(User.email == email))
        user = result.scalars().first()
        return user
