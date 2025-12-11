"""
The Cactus Library

Licensed under MIT - qerdinger @ 2025

Hosted on GitHub : https://github.com/qerdinger/cactus
"""

from enum import Enum

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
    
    def get_payload(self) -> any:
        return self._payload
    
    def get_status_code(self) -> int:
        return self._status_code

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
    def decorator(func):
        def wrapper(*args, **kwargs):
            if auth_required(auth):
                return "Authentification required"
            return make_res(func(*args, **kwargs))
        return wrapper
    return decorator