extern crate serde;
extern crate serde_json;

// use serde_json::{Value

fn main() {
	use std::fs;
	use std::io::Read;

	let paths = fs::read_dir("/Volumes/Fs/flat-json").unwrap();

  println!("here we gooo");

	for path in paths {
		let path = path.unwrap().path();
    // println!("Name: {}", path.display());

		let mut file = fs::File::open(path.clone()).unwrap();

		let mut json = String::new();
		file.read_to_string(&mut json).unwrap();



    // let res: Result<serde_json::Value, _> = serde_json::from_str(&json);
    // if let Ok(p) = res {
    //   // println!("Data");
    // } else {
    //   // println!("woops");
    // }
	}
}


// 286648