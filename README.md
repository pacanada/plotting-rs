Ideas:
- [X] Create and remove plots based on expressions
- [X] There are a bunch of clones, try to be smarter
- [X] Make it adaptative depending where you are looking (x )
- [X] Separate functions and parameters
- [X] Separate name and expression
- [X] Make parameters being modifyable (delete and change limits)
- [ ] Add constant to be plotted as well
- [ ] Add reset button
- [ ] Improve format, separate into functions
- [ ] Handle errors without panicking
- [ ] make it work with wasm and web (https://github.com/emilk/eframe_template/blob/main/src/main.rs)


# Run for web
```cmd
cargo build --release --target wasm32-unknown-unknown
```
```cmd
cargo install --locked trunk (only once)
```
```cmd
trunk serve (for local developmet)
```

```cmd
trunk build --release (to save wasm artifacts to dist)
```

# known issue
- It cannot be displayed with the rest of the wasm apps in github pages
- When building with `trunk`, the resulting `index.html` is generated with the relative path considering the page artifacts are in the root, but they are located in `/plotting-rs/`. For now is manually overwriten to fix the path.
- With `wasm-bindgen`, the experience is awfull.