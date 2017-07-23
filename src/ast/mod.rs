mod alias;
mod jsx;
mod flow;
mod misc;
mod expression;
mod statement;
mod declaration;
mod literal;

pub enum AST {
	Script(Script),
	Module(Module),
}

pub struct Script {
	directives: Vec<misc::Directive>,
	body: Vec<alias::StatementItem>,
	position: misc::MaybePosition,
}
 
pub struct Module {
	directives: Vec<misc::Directive>,
	body: Vec<alias::ModuleStatementItem>,
	position: misc::MaybePosition,
}


// TODO
// Typescript?