# Serverless Python Script

# cactuskit.core
# cactuskit.protocol
from cactuskit import ApiMethod, ApiProtocol, HttpStatus, cactuize
import time

"""
Basic Entrypoint
Input: [No Input]
Output: String
"""

def authenticate():
    return True

@cactuize()
def simple_entrypoint():
    return f"Hello World from {simple_entrypoint}"

GLOBAL_I = 0
DB = {}

@cactuize()
def simple_entrypoint_delayed():
    time.sleep(1)
    import os
    global GLOBAL_I,DB
    
    GLOBAL_I += 1
    DB[GLOBAL_I] = time.time()
    
    return f"""Hello World\nTime:{time.time()}\nGlobalI : {GLOBAL_I}\n{DB}\nfrom pid : {os.getpid()}"""

@cactuize(
    auth=authenticate,
    protocol=ApiProtocol.HTTP,
    method=ApiMethod.GET)
def entrypoint(name):
    return (HttpStatus.HTTP_CUSTOM(201), {
        "content": f"Hello {name}"
    })

@cactuize()
def en_lang():
    print("English")
    return (HttpStatus.HTTP_OK, "English")

@cactuize()
def fr_lang():
    print("French")
    return "French"

@cactuize()
def not_found():
    return (HttpStatus.HTTP_NOT_FOUND, "Not found!")

@cactuize()
def access_denied():
    return (HttpStatus(403), "Access denied!")

#print(simple_entrypoint().get_status_code(), simple_entrypoint().get_payload())
#print(entrypoint().get_payload())
#print("EP:", entrypoint("Bob"))
#print(simple_entrypoint().get_payload())
#print("SE:", simple_entrypoint())
#print("EN:", en_lang())
#print("FR:", fr_lang())
#print("NF:", not_found())
#print("AD:", access_denied())

#rslt = entrypoint("Patrick")
#print(rslt)
#print(rslt.get_payload())
#print(rslt.get_status_code())

