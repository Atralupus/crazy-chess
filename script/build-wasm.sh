cargo build --release --target wasm32-unknown-unknown -p client
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/client.wasm
cp ./index.html ./out/index.html
cd out
python3 -m http.server
