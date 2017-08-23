extern crate bindgen;

fn main() {
    let generated = bindgen::builder()
        .header("xcb_cursor/csrc/xcb_cursor/cursor.h")
        .no_unstable_rust()
        .ctypes_prefix("libc")
        .clang_arg("-I")
        .clang_arg("xcb_cursor/csrc/xcb_cursor")
        .generate().unwrap();
    generated.write_to_file("src/gen.rs").unwrap();
    // TODO Static linking feature
}
