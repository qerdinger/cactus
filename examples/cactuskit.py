"""
The Cactus Library

Licensed under MIT - qerdinger @ 2025

Hosted on GitHub : https://github.com/qerdinger/cactus
"""

from enum import Enum
import time as tm
import sys

DELIMITER = ";"

class HttpStatus(Enum):
    HTTP_OK = 200

class ApiProtocol(Enum):
    HTTP = 0
    WS = 1

class ApiMethod(Enum):
    GET = 0
    POST = 1

class CactusResponse:
    def __init__(self, payload : any, status_code : int):
        self._payload = payload
        self._status_code = status_code
        self._timestamp = tm.time()

    def __repr__(self):
        s = [
            f"Time={self.get_timestamp()}",
            f"Status={self.get_status_code()}",
            f"PSize={self.get_payload_size()}b",
            f"PHash={self.get_payload_hash()}",
            f"Size={self.get_size()}b",
        ]
        return DELIMITER.join(s)

    def get_payload(self) -> any:
        return self._payload

    def get_payload_size(self) -> int:
        return sys.getsizeof(self.get_payload())

    def get_size(self) -> int:
        return sys.getsizeof(self)

    def get_payload_hash(self) -> int:
        return hash(self.get_payload())

    def get_status_code(self) -> int:
        return self._status_code

    def get_timestamp(self) -> tm.time:
        return self._timestamp


def is_initialised() -> bool:
    True

def auth_required(auth_mthd : object):
    if auth_mthd is None:
        return False
    
    return not auth_mthd()

def make_res(x : object):
    return CactusResponse(x, HttpStatus.HTTP_OK)

"""
Cactus Web Handler
"""
def wattr(
    protocol=ApiMethod.GET,
    method=ApiProtocol.HTTP,

    auth=None,
    middleware=None
    ):

    def init_declaration(f : object):
        f._is_declared = True

    def decorator(func):
        init_declaration(func)

        def wrapper(*args, **kwargs):
            if auth_required(auth):
                return "Authentification required"
            return make_res(func(*args, **kwargs))
        return wrapper

    return decorator
