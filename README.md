# z1info_rust

Add extra information to the binary package through `cargo run [...] z1info=extra_information`

## Use

### add code

```no_run
fn main() {
    z1info_rust::run("z1template");
    // z1info_test::run("z1_info:{z1_info},git_info:{git_info},build_time:{build_time}");
}
```

### run command

`z1info=` must be placed at the end of the command line,`cargo run`will write extra information to tmp file.

- `cargo run p1 p2 z1info=version:1.2.3,compiler:myzero1`

### build

- cargo clean && cargo build
- OR cargo clean && cargo build --release

## Result

### run command or run `builded binary package`

- cargo clean && cargo run p1 p2
- OR run builded binary package

### The Result

<br/>=============== z1info extended data ====================
<br/>| Extended data added to binary file through z1info.
<br/>|--------------- z1info parameter ----------------------
<br/>| z1info=version:1.2.3,compiler:myzero1
<br/>|--------------- git info ------------------------------
<br/>| commit id: 94896476ea1696f9b8764cd845f225e4af586bc4
<br/>|--------------- build time ----------------------------
<br/>| 1621770625
<br/>=========================================================
