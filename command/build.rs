use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const GCRS: &str = "Error writing out to generated-commands.rs";

fn main() {
    let out_dir = env::var("OUT_DIR").expect("Could not get OUT_DIR");
    let dest_path = Path::new(&out_dir).join("generated-commands.rs");
    let mut f = File::create(&dest_path).expect("Could not create generated-commands.rs");

    let commands_dir = {
        let root = env::var("CARGO_MANIFEST_DIR").expect("Could not get CARGO_MANIFEST_DIR");
        let mut pathbuf = PathBuf::from(&root);
        pathbuf.push("src");
        pathbuf
    };

    // Write out module imports
    write!(f, "mods! [ ").expect(GCRS);
    let mut one = false;
    for item in get_mods(&commands_dir) {
        if one {
            write!(f, ", ").expect(GCRS);
        }
        write!(f, "{}", item).expect(GCRS);
        one = true;
    }
    writeln!(f, " ];").expect(GCRS);

    // Write out struct exports
    for item in get_mods(&commands_dir) {
        writeln!(f, "pub use {}::*;", item).expect(GCRS);
    }
}

fn get_mods(path: &Path) -> impl Iterator<Item=String> {
    let set = {
        let mut set = HashSet::new();
        set.insert("lib");
        set
    };

    path.read_dir()
        .expect("Could not read command dir")
        .filter_map(|x| Some(String::from(x.ok()?.path().file_stem()?.to_str()?)))
        .filter(move |x| !set.contains(x.as_str()))
}
