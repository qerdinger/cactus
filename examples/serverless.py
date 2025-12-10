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

@wattr(
    auth=authenticate,
    protocol=ApiProtocol.HTTP,
    method=ApiMethod.GET)
def entrypoint():
    return "Hello World"

print(entrypoint())