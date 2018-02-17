extern crate jsparse;
use std::io::Read;
// extern crate flame;
// extern crate cpuprofiler;

// use cpuprofiler::PROFILER;

struct Framer {
    // t: flame::SpanGuard
}
impl Framer {
    fn new() -> Framer {
        // flame::start("cpu-heavy calculation");

        // PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();

        Framer { }
    }
}
impl Drop for Framer {
    fn drop(&mut self) {
        // PROFILER.lock().unwrap().stop().unwrap();
        // flame::end("cpu-heavy calculation");

        // flame::dump_html(&mut ::std::fs::File::create("flame-graph.html").unwrap()).unwrap()
    }
}

use std::fs::File;

fn main() {
    let mut it = std::env::args();
    let _name = it.next().unwrap();
    let filename = it.next().unwrap();

    let s = {
        let mut f = File::open(filename).unwrap();

        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        // std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };

    let _f = Framer::new();
    let _: () = jsparse::parser::parse_root(&s[..]);
}
