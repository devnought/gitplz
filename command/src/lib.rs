extern crate color_printer;
extern crate gitlib;

macro_rules! mods {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
        )*
    };
}

include!(concat!(env!("OUT_DIR"), "/generated-commands.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
