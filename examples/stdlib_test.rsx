// Force JIT compilation
rust {
    // This forces project builder execution
}

print("Testing Standard Library Modules...")

// JSON
data = json.parse("{\"a\": 1, \"b\": \"hello\"}")
print("JSON Parse:", data)
str_val = json.stringify(data)
print("JSON Stringify:", str_val)

// Time
start = time.now()
time.sleep(100)
end = time.now()
print("Time elapsed (approx 0.1s):", end - start)

// OS
args = os.args()
print("Args:", args)
// env_path = os.env("PATH") // might be long
// print("PATH length:", env_path.len())

// HTTP (requires internet, or check error)
print("Testing HTTP (may fail if no internet)...")
// try { // No try-catch yet, manual error handling or expect crash if fail?
// Let's use a reliable URL or just skip if dangerous.
// http.get("http://example.com")
// }
print("HTTP skipped for CI safety, but available as http.get()")
