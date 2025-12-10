"""
The Cactus Library

Licensed under MIT - qerdinger @ 2025

Hosted on GitHub : https://github.com/qerdinger/cactus
"""

from enum import Enum

class ApiProtocol(Enum):
    HTTP = 0
    WS = 1

class ApiMethod(Enum):
    GET = 0
    POST = 1

def auth_required(auth_mthd : object):
    if auth_mthd is None:
        return False
    
    return not auth_mthd()

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
            return func(*args, **kwargs)
        return wrapper
    return decorator