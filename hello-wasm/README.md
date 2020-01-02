```
// wasm-packをインストールした
cargo install wasm-pack
// hello-wasmを生成
cargo new --lib hello-wasm
// パッケージのビルド
wasm-pack build --scope tetsu5555

// packageをnpmに公開
cd pkg
npm publish --access=public

// packageをwebで利用
cd site
npm install
npm run serve // webpack-dev-server
```