// mod alias;
// mod jsx;
// mod flow;
// mod misc;
// mod expression;
// mod statement;
// mod declaration;
// mod literal;

// pub struct AST {
// 	Script(Script),
// 	Module(Module),
// }

// pub struct Script {
// 	directives: Vec<misc::Directive>,
// 	body: Vec<alias::StatementItem>,
// 	position: misc::MaybePosition,
// }
 
// pub struct Module {
// 	directives: Vec<misc::Directive>,
// 	body: Vec<alias::ModuleStatementItem>,
// 	position: misc::MaybePosition,
// }

// TODO
// Typescript?



#[cfg(test)]
mod tests {

	#[bench]
    #[test]
    fn it_works() {
		// use ucd::Codepoint;
		// use std;

  //   	let mut max = '0';
  //   	for i in 0x10000..0x10FFFF {
  //   		if let Some(c) = std::char::from_u32(i) {
  //   			if c.is_id_start() {
  //   				max = c;
  //   			}
  //   		}
  //   	}

  //   	println!("{:X}", max as u32);

  		use std::fs;
  		use std::io::Read;

  		let paths = fs::read_dir("/Volumes/Fs/flat-json").unwrap();

  		for path in paths {
  			let path = path.unwrap().path();

  			let mut file = fs::File::open(path.clone()).unwrap();

  			let mut json = String::new();
  			file.read_to_string(&mut json).unwrap();


  			// println!("Name: {}", path.display());
  		}
	}
}
