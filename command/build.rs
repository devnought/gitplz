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

    let set = {
        let mut set = HashSet::new();
        set.insert("command");
        set.insert("worktype");
        set.insert("lib");
        set
    };

    let commands_dir = {
        let root = env::var("CARGO_MANIFEST_DIR").expect("Could not get CARGO_MANIFEST_DIR");
        let mut pathbuf = PathBuf::from(&root);
        pathbuf.push("src");
        pathbuf
    };

    let iter = commands_dir
        .read_dir()
        .expect("Could not read command dir")
        .filter_map(|x| Some(String::from(x.ok()?.path().file_stem()?.to_str()?)))
        .filter(|x| !set.contains(x.as_str()));

    write!(f, "mods! [ ").expect(GCRS);
    let mut one = false;
    for item in iter {
        if one {
            write!(f, ", ").expect(GCRS);
        }
        write!(f, "{}", item).expect(GCRS);
        one = true;
    }
    write!(f, " ];").expect(GCRS);
}
