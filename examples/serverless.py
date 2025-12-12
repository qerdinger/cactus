# Serverless Python Script

# cactuskit.core
# cactuskit.protocol
from cactuskit import ApiMethod, ApiProtocol, wattr

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
def entrypoint():
    return {
        "content": "Hello World"
    }

@wattr()
def en_lang():
    print("English")

@wattr()
def fr_lang():
    print("French")

#print(simple_entrypoint().get_status_code(), simple_entrypoint().get_payload())
print(entrypoint().get_payload())
print(entrypoint())
print(simple_entrypoint().get_payload())
print(simple_entrypoint())
#print(en_lang())
#print(fr_lang())