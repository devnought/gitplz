extern crate proc_macro;

use proc_macro::TokenStream;
use std::{collections::HashSet, path::Path};

#[proc_macro]
pub fn make_mods(_item: TokenStream) -> TokenStream {
    let f = Path::new("command/src");
    let iter = get_mods(f);

    let res = iter
        .map(|x| format!("mod {}; pub use crate::{}::*;", x, x))
        .collect::<Vec<_>>()
        .join(" ");

    res.parse().unwrap()
}

fn get_mods(path: &Path) -> impl Iterator<Item = String> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
