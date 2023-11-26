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