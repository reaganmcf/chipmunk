# Chipmunk in the browser


### Build steps

- `wasm-pack build --target no-modules --out-dir ./pkg`
- `wasm-dis pkg/chipmunk_js_bg.wasm | grep '(import "wbg" "__wbg_CHIPMUNK'`
- Format those symbols into comma separated list
