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
```
cargo install --locked trunk (only once)
```
```
trunk serve
```

# known issue
it cannot be displayed with the rest of the wasm apps in github pages