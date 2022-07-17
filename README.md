# Basic chess engine 

Based on the example here of integrating Chess.js and Chessboard.js: https://github.com/0hq/starter_chess_engine 

Backend is in Rust and compiles to WASM

---

Instructions for use:

- Download the repo.  
- Go into the "engine" directory and run `cargo build --target wasm32-unknown-unknown --release` in the terminal to compile the Rust backend
- Go into the "frontend" directory and run `mv ../engine/target/wasm32-unknown-unknown/release/engine.wasm assets` to move the compiled Rust backend into the assets folder
- Run `python3 -m http.server` or similar to serve the site
  
---

Chess.js: https://github.com/jhlywa/chess.js/blob/master/README.md   

Chessboard.js: https://chessboardjs.com/  
