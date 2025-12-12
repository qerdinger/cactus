# Serverless Python Script

# cactuskit.core
# cactuskit.protocol
from cactuskit.python3.cactuskit import ApiMethod, ApiProtocol, HttpStatus, wattr

"""
Basic Entrypoint
Input: [No Input]
Output: String
"""

def authenticate():
    return True

@wattr()
def simple_entrypoint():
    return f"Hello World from {simple_entrypoint}"

@wattr(
    auth=authenticate,
    protocol=ApiProtocol.HTTP,
    method=ApiMethod.GET)
def entrypoint(name):
    return (HttpStatus.HTTP_CUSTOM(201), {
        "content": f"Hello {name}"
    })

@wattr()
def en_lang():
    print("English")
    return (HttpStatus.HTTP_OK, "English")

@wattr()
def fr_lang():
    print("French")
    return "French"

@wattr()
def not_found():
    return (HttpStatus.HTTP_NOT_FOUND, "Not found!")

@wattr()
def access_denied():
    return (HttpStatus(403), "Access denied!")

#print(simple_entrypoint().get_status_code(), simple_entrypoint().get_payload())
#print(entrypoint().get_payload())
print("EP:", entrypoint("Bob"))
#print(simple_entrypoint().get_payload())
print("SE:", simple_entrypoint())
print("EN:", en_lang())
print("FR:", fr_lang())
print("NF:", not_found())
print("AD:", access_denied())

rslt = entrypoint("Patrick")
print(rslt)
print(rslt.get_payload())
print(rslt.get_status_code())
