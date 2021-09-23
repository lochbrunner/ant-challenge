from dataclasses import dataclass
from typing import List, Optional
from abc import ABC, abstractmethod

from enum import Enum


class Semantic(Enum):
    ALY = 1
    ENEMY = 2
    SUGAR = 3
    OWN_HILL = 4
    OTHERS_HILL = 5


@dataclass
class ViewRay:
    distance: float
    sematic: Semantic


@dataclass
class Smell:
    strength: float  # 0 to 1
    ally_code: int  # custom codes
    enemy_code: int  # custom codes


@dataclass
class Perception:
    touch: bool  # did the agent hit something
    velocity: float  # current velocity of ant
    view: List[ViewRay]  # fixed order of angles
    smell: List[Smell]  # unordered list of smells


class Activity(Enum):
    NONE = 1
    CARRY = 2
    FIGHT = 3


@dataclass
class Action:
    turn: float         # angle
    accelerate: float
    activity: Activity
    create_smell: Optional[int]  # custom codes


class BaseAnt(ABC):
    @abstractmethod
    def think(self, perception: Perception) -> Action:
        pass
