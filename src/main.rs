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




// trait Offset: Default {}

// trait Reader {
//     type Offset: Offset;
// }

// impl Offset for usize {}

// impl<'a> Reader for &'a [u8] {
//     type Offset = usize;
// }

// // OK
// // struct Header<R: Reader>(R, usize);

// // Bad
// struct Header<R: Reader>(R, R::Offset);

// impl <R: Reader<Offset=usize>> Header<R> {
//     fn new(r: R) -> Self {
//         Header(r, 0)
//     }
// }

// fn test<R1: Reader, R2: Reader>(_: Header<R1>, _: Header<R2>) {}

// fn main() {
//     let buf1 = [0u8];
//     {
//       let slice1 = &buf1[..];
//       {
//         let header1 = Header::new(slice1);
//         {
//           let buf2 = [0u8];
//           {
//             let slice2 = &buf2[..];
//             {
//               let header2 = Header::new(slice2);

//               test(header1, header2);
//             }
//           }
//         }
//       }
//     }
// }