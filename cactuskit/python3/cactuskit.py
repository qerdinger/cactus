"""
The Cactus Library

Licensed under MIT - qerdinger @ 2025
Hosted on GitHub : https://github.com/qerdinger/cactus

As a matter of fact, the "cactuize" decorator name comes from an inspiration of @khalidbelk (https://github.com/khalidbelk)
"""

from enum import Enum, IntEnum
from functools import wraps
import time as tm
import sys
import json
from typing import Union, Dict

# ---------------------------------------------------------------------
# Constants & Types
# ---------------------------------------------------------------------

DELIMITER = ";"
SIZE_TYPE = int
PAYLOAD_TYPE = Union[Dict, str]

# ---------------------------------------------------------------------
# HTTP Status
# ---------------------------------------------------------------------

class HttpStatus(IntEnum):
    HTTP_OK = 200
    HTTP_NOT_FOUND = 404

    @classmethod
    def HTTP_CUSTOM(cls, code: int) -> "HttpStatus":
        return cls(code)

    @classmethod
    def _missing_(cls, value: int) -> "HttpStatus":
        obj = int.__new__(cls, value)
        obj._name_ = f"HTTP_CUSTOM_{value}"
        obj._value_ = value
        return obj

# ---------------------------------------------------------------------
# API Protocol & Method
# ---------------------------------------------------------------------

class ApiProtocol(Enum):
    HTTP = 0
    WS = 1

class ApiMethod(Enum):
    GET = 0
    POST = 1

# ---------------------------------------------------------------------
# Response Object
# ---------------------------------------------------------------------

class CactusResponse:
    def __init__(self, payload: PAYLOAD_TYPE, status_code: HttpStatus):
        self._payload = payload
        self._status_code = status_code
        self._timestamp = tm.time()

    def __repr__(self) -> str:
        return DELIMITER.join([
            f"Status={self.get_status_code()}",
            f"PSize={self.get_payload_size()}b",
            f"PHash={self.get_payload_hash()}",
            f"Size={self.get_size()}b",
            f"Time={self.get_timestamp():.3f}"
        ])

    def get_payload(self) -> PAYLOAD_TYPE:
        return self._payload

    def get_payload_size(self) -> SIZE_TYPE:
        return sys.getsizeof(self._payload)

    def get_size(self) -> SIZE_TYPE:
        return sys.getsizeof(self)

    def get_payload_hash(self) -> int:
        if isinstance(self._payload, dict):
            return hash(frozenset(self._payload.items()))
        return hash(self._payload)

    def get_status_code(self) -> HttpStatus:
        return self._status_code

    def get_timestamp(self) -> float:
        return self._timestamp
    
class CactusEncoder(json.JSONEncoder):
    def default(self, o):
        if isinstance(o, CactusResponse):
            return o.__dict__
        return super().default(o)


# ---------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------

def auth_required(auth_mthd) -> bool:
    """
    Returns True if authentication is required and failed.
    """
    if auth_mthd is None:
        return False
    return not bool(auth_mthd())

def make_res(payload: PAYLOAD_TYPE,
             status_code: HttpStatus = HttpStatus.HTTP_OK) -> CactusResponse:
    return CactusResponse(payload, status_code)

# ---------------------------------------------------------------------
# Web Handler Decorator
# ---------------------------------------------------------------------

def cactuize(
    protocol: ApiProtocol = ApiProtocol.HTTP,
    method: ApiMethod = ApiMethod.GET,
    auth=None,
    middleware=None
):
    """
    Declares a function as a Cactus entrypoint.
    Metadata is attached to the exported callable (wrapper).
    """

    def decorator(func):

        @wraps(func)
        def wrapper(*args, **kwargs):
            if auth_required(auth):
                return "Authentication required"

            result = func(*args, **kwargs)

            if isinstance(result, PAYLOAD_TYPE):
                return make_res(result)

            if (
                isinstance(result, tuple)
                and len(result) == 2
                and isinstance(result[0], HttpStatus)
                and isinstance(result[1], PAYLOAD_TYPE)
            ):
                return make_res(result[1], status_code=result[0])

            raise TypeError(f"Unsupported handler signature: {result!r}")

        wrapper._is_declared = True

        wrapper._cactus = {
            "protocol": protocol,
            "method": method,
            "auth": auth is not None,
            "middleware": middleware is not None,
        }

        return wrapper

    return decorator