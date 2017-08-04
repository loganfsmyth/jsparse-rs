use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence, HasInOperator, FirstSpecialToken, SpecialToken};

use ast::functions::Directive;
use ast::alias;

node_enum!(@node_display pub enum Ast {
    Script(Script),
    Module(Module),
});


node!(pub struct Script {
    directives: Vec<Directive>,
    body: Vec<alias::StatementItem>,
});
impl NodeDisplay for Script {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();
        for d in self.directives.iter() {
            f.node(d)?;
        }
        for item in self.body.iter() {
            f.node(item)?;
        }

        Ok(())
    }
}


node!(pub struct Module {
    directives: Vec<Directive>,
    body: Vec<alias::ModuleStatementItem>,
});
impl NodeDisplay for Module {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();
        for d in self.directives.iter() {
            f.node(d)?;
        }
        for item in self.body.iter() {
            f.node(item)?;
        }

        Ok(())
    }
}
