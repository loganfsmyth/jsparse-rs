use std::string;
use super::misc;
use super::flow;
use super::alias;
use super::statement;
use super::literal;
use super::display;

enum DeclaratorList<T: display::NodeDisplay> {
    Last(T),
    List(T, Box<DeclaratorList<T>>),
}
impl<T: display::NodeDisplay> display::NodeDisplay for DeclaratorList<T> {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            DeclaratorList::Last(ref decl) => f.node(decl),
            DeclaratorList::List(ref decl, ref list) => {
                f.node(decl)?;
                f.token(display::Token::Comma)?;
                f.node(list)
            }
        }
    }
}

nodes!{
	// let foo, bar;
	pub struct LetDeclaration {
		declarators: DeclaratorList<LetDeclarator>,
	}
	impl display::NodeDisplay for LetDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Let)?;
			f.node(&self.declarators)
		}
	}
	pub struct LetDeclarator {
		id: misc::Pattern,
		init: Option<alias::Expression>,
	}
	impl display::NodeDisplay for LetDeclarator {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.id)?;
			if let Some(ref init) = self.init {
				f.token(display::Token::Eq)?;
				f.node(init)?;
			}
			Ok(())
		}
	}


	// const foo = 4, bar = 5;
	pub struct ConstDeclaration {
		declarators: DeclaratorList<ConstDeclarator>,
	}
	impl display::NodeDisplay for ConstDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Const)?;
			f.node(&self.declarators)
		}
	}
	pub struct ConstDeclarator {
		id: misc::Pattern,
		init: alias::Expression,
	}
	impl display::NodeDisplay for ConstDeclarator {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.node(&self.id)?;
			f.token(display::Token::Eq)?;
			f.node(&self.init)
		}
	}

	// function name() {}
	pub struct FunctionDeclaration {
		id: misc::BindingIdentifier,
		params: misc::FunctionParams,
		body: misc::FunctionBody,
		fn_kind: misc::FunctionKind,

		// Flow extension
		type_parameters: Option<flow::Parameters>,
		return_type: Option<Box<flow::Annotation>>,
	}
	impl display::NodeDisplay for FunctionDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			f.token(display::Token::Function)?;
			f.node(&self.id)?;
			if let Some(ref type_parameters) = self.type_parameters {
				f.node(type_parameters)?;
			}
			f.node(&self.params)?;
			if let Some(ref return_type) = self.return_type {
				f.node(return_type)?;
			}
			f.node(&self.body)
		}
	}

	// class name {}
	pub struct ClassDeclaration {
		decorators: Vec<misc::Decorator>, // experimental
		id: misc::BindingIdentifier,
		extends: Option<Box<alias::Expression>>,
		implements: Option<flow::BindingIdentifierAnnotationList>,
		body: misc::ClassBody,

		type_parameters: Option<flow::Parameters>,
	}
	impl display::NodeDisplay for ClassDeclaration {
		fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
			// TODO
			Ok(())
		}
	}
}
