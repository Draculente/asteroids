from sqlalchemy.orm import relationship 
from sqlalchemy.orm import Mapped
from sqlalchemy.orm import mapped_column
from sqlalchemy import ForeignKey
import json
from typing import List
from . import db

class Base: 
    __table_args__ = {
        "mysql_engine": "InnoDB",
        "mysql_charset": "utf8",
        "mysql_collate": "utf8_unicode_ci",
    }
    def toJSON(self):
        dict = self.as_dict()
        return json.dumps(dict)
    def as_dict(self):
       return {c.name: getattr(self, c.name) for c in self.__table__.columns}


class ItemLevel(Base, db.Model):
    __tablename__ = "item_level"
    game_id: Mapped[int] = mapped_column(ForeignKey("game.id"), primary_key=True)
    item_id: Mapped[int] = mapped_column(ForeignKey("item.id"), primary_key=True)
    level = db.Column(db.Integer, nullable=False, default=1)
    item: Mapped["Item"] = relationship()

    def as_dict(self):
        dict = super().as_dict()
        dict["item"] = self.item.as_dict()
        return dict

class Item(Base, db.Model):
    __tablename__ = "item"
    id = db.Column(db.Integer, primary_key=True, autoincrement=False)
    name = db.Column(db.String(50), nullable=False, unique=True)
    description = db.Column(db.String(500), nullable=False, server_default='')
    price = db.Column(db.Integer, nullable=False)

class Game(Base, db.Model):
    __tablename__ = "game"
    id = db.Column(db.Integer, primary_key=True)
    score = db.Column(db.Integer, nullable=False, default=0)
    coins = db.Column(db.Integer, nullable=False, default=0)
    lives = db.Column(db.Integer, nullable=False, default=3)
    ended = db.Column(db.Boolean, nullable=False, default=False)
    enemy_spawn_timeout = db.Column(db.Integer, nullable=False)
    user_id = db.Column(db.Integer, db.ForeignKey('user.id'), nullable=False)
    items: Mapped[List[ItemLevel]] = relationship(cascade="all")
    
    def as_dict(self):
        dict = super().as_dict()
        dict["items"] = [item_level.as_dict() for item_level in self.items]
        return dict

class User(Base, db.Model):
    __tablename__ = "user"
    id = db.Column(db.Integer, primary_key=True)
    username = db.Column(db.String(50), nullable=False, unique=True)
    password = db.Column(db.String(255), nullable=False, server_default='')
    games: Mapped[List["Game"]] = relationship(cascade="all, delete-orphan", lazy=True)