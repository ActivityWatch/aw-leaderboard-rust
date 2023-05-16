from sqlalchemy import (
    JSON,
    Boolean,
    Column,
    DateTime,
    ForeignKey,
    Integer,
    String,
)
from sqlalchemy.orm import DeclarativeBase, relationship
from sqlalchemy.dialects.postgresql import UUID as pgUUID

engine = "sqlite"


def UUID():
    if engine == "postgresql":
        # Only supported in PostgreSQL
        return pgUUID(as_uuid=True)
    else:
        return String


class Base(DeclarativeBase):
    pass


class User(Base):
    __tablename__ = "users"

    # UUID not supported by SQLite
    # id = Column(UUID(as_uuid=True), primary_key=True, index=True)
    id: Column[String] = Column(UUID(), primary_key=True, index=True)
    username = Column(String, unique=True, index=True)
    email = Column(String, unique=True, index=True)
    hashed_password = Column(String)
    is_active = Column(Boolean, default=True)


class Event(Base):
    __tablename__ = "events"

    id: Column[String] = Column(UUID(), primary_key=True, index=True)
    user_id: Column[String] = Column(UUID(), ForeignKey("users.id"))
    user = relationship("User")

    timestamp = Column(DateTime)
    duration = Column(Integer)
    data = Column(JSON)
