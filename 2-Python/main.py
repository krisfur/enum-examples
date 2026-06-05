from dataclasses import dataclass
from enum import Enum, auto


# Python's standard enum. auto() assigns the values (starting at 1).
class Status(Enum):
    PENDING = auto()
    RUNNING = auto()
    SUCCEEDED = auto()
    FAILED = auto()


# No built-in tagged union: a set of dataclasses joined by a union type,
@dataclass
class Text:
    length: int


@dataclass
class Image:
    width: int
    height: int


@dataclass
class Video:
    width: int
    height: int
    duration: float


Media = Text | Image | Video


def describe(m: Media) -> str:
    match m:
        case Text(length):
            return f"text: {length} chars"
        case Image(width, height):
            return f"image: {width}x{height}"
        case Video(width, height, duration):
            return f"video: {width}x{height}, {duration}s"


status = Status.RUNNING
print(f"status: {status.name.lower()} ({status.value})")
for m in [Text(280), Image(1920, 1080), Video(1920, 1080, 12.5)]:
    print(describe(m))
