from datetime import datetime
from typing import Optional

from app.models.util import Action
from pydantic import PositiveInt
from sqlalchemy import Column, Enum, Text, DateTime
from sqlalchemy.dialects.postgresql import JSONB
from sqlmodel import Field, SQLModel


class DittoEvent(SQLModel, table=True):
    __tablename__ = "dittoevent"
    __table_args__ = {"extend_existing": True}

    time: datetime = Field(
        sa_column=Column("time", DateTime(timezone=True), primary_key=True, nullable=False)
    )
    thing_id: str = Field(sa_column=Column("thing_id", Text, nullable=False))
    action: Action = Field(
        sa_column=Column(
            "action",
            Enum(Action, name="action_enum", create_type=False),
            nullable=False,
        )
    )
    revision: Optional[PositiveInt] = Field(default=None)
    path: str = Field(sa_column=Column("path", Text, nullable=False))
    value: Optional[dict] = Field(default=None, sa_type=JSONB)