// Main file testing imports
print("Starting main...")

import "examples/imports/lib.rsx" as lib

print("Lib loaded.")
print("PI from lib:", lib.PI)

r = 10
a = lib.area(r)
print("Area of radius 10:", a)

msg = lib.greet("RustX")
print("Greeting:", msg)

// Verify isolation (PI should be in lib, not global)
// print(PI) // This should error if uncommented
