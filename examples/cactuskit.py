"""
The Cactus Library

Licensed under MIT - qerdinger @ 2025

Hosted on GitHub : https://github.com/qerdinger/cactus
"""

from enum import Enum
import time as tm
import sys

DELIMITER = ";"
SIZE_TYPE = int
PAYLOAD_TYPE = dict | str

class HttpStatus(Enum):
    HTTP_OK = 200,
    HTTP_NOT_FOUND = 404

class ApiProtocol(Enum):
    HTTP = 0
    WS = 1

class ApiMethod(Enum):
    GET = 0
    POST = 1

class CactusResponse:
    def __init__(self, payload : PAYLOAD_TYPE, status_code : HttpStatus):
        self._payload = payload
        self._status_code = status_code
        self._timestamp = tm.time()

    def __repr__(self):
        return DELIMITER.join([
            f"Time={self.get_timestamp()}",
            f"Status={self.get_status_code()}",
            f"PSize={self.get_payload_size()}b",
            f"PHash={self.get_payload_hash()}",
            f"Size={self.get_size()}b",
        ])

    def get_payload(self) -> PAYLOAD_TYPE:
        return self._payload

    def get_payload_size(self) -> SIZE_TYPE:
        return sys.getsizeof(self.get_payload())

    def get_size(self) -> SIZE_TYPE:
        return sys.getsizeof(self)

    def get_payload_hash(self) -> SIZE_TYPE:
        if isinstance(self._payload, dict):
            return hash(frozenset(self._payload.items()))
        return hash(self.get_payload())

    def get_status_code(self) -> HttpStatus:
        return self._status_code

    def get_timestamp(self) -> tm.time:
        return self._timestamp


def is_initialised() -> bool:
    True

def auth_required(auth_mthd : object) -> bool:
    if auth_mthd is None:
        return False
    
    return not auth_mthd()

def make_res(x : PAYLOAD_TYPE, status_code=HttpStatus.HTTP_OK) -> CactusResponse:
    return CactusResponse(x, status_code)

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
            res = func(*args, **kwargs)
            if isinstance(res, PAYLOAD_TYPE):
                return make_res(res)
            elif isinstance(res, tuple) and isinstance(res[0], int) and isinstance(res[1], PAYLOAD_TYPE):
                return make_res(res[1], status_code=res[0])
            else:
                raise Exception(f"Signature not supported [{res}]")
        return wrapper

    return decorator
