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
    import os,threading,random
    global GLOBAL_I,DB
    TIMEOUT = random.randrange(5)
    SEED = random.random()
    time.sleep(TIMEOUT)
    
    
    for x in range(100):
        DB["{}{}".format(GLOBAL_I, x)] = GLOBAL_I
    
    GLOBAL_I += 1
    return f"""Hello World\nTime:{time.time()}\nTIMEOUT:{TIMEOUT}\nSEED:{SEED}\nGlobalI : {GLOBAL_I}\nfrom pid : {os.getpid()}--{threading.get_ident()}"""

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

