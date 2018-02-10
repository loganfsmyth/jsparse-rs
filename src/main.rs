extern crate jsparse;
use std::io::Read;

fn main() {
    // let mut it = std::env::args();
    // let name = it.next().unwrap();
    // let filename = it.next().unwrap();

    let s = {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };

    let _: () = jsparse::parser::parse_root(&s[..]);
}
