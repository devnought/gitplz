use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated-commands.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(b"mods! [ checkout, branch_delete, branch_find, reset, status ];")
        .expect("Error writing out to generated-commands.rs");
}
