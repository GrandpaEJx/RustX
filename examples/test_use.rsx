// Test explicit imports
use json
use os

data = `{"name": "test", "value": 42}`
obj = json.parse(data)
print("Parsed object:")
print(obj)

env_path = os.env("PATH")
print(`PATH: {env_path}`)
