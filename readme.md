Spin LLM Example
---

Example based on the cool tutorial by [Thorsten Hans](https://github.com/thorstenHans):
https://www.youtube.com/watch?v=XLfq3JLI9O4

# Setup

Install [spin](https://developer.fermyon.com/spin/v2/install)

Ensure wasm32 integration is installed in rust.  
Run: `rustup target add wasm32-wasi`.

Initial example created via:
```
spin new pin-llm-demo2 --template http-rust  --accept-defaults
cargo add uuid -F v4
```

# Build

`spin build`

# Run

`spin deploy`

# Run locally with cloud gpu backend

spin cloud-gpu init

copy generated toml to file `cloud-gpu.toml` (do not check this file in - auth token inside!).

Run spin with `spin up --runtime-config-file cloud-gpu.toml`

Start conversation:
```
curl -v -d '{"text":"What is the biggest city in europe by population?"}' http://localhost:3000/
```

Output:
```
London.%
```

Extract `x-conversationid` header.

Start conversation:
```
curl -v -H "x-conversationid: 9adacc40-7fc9-4d6c-9deb-cae00531659a" -d '{"text":"How many people live there?"}' http://localhost:3000/
```

Output:
```
8.9 million.
```