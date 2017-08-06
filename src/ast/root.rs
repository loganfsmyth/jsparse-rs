use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult};

use ast::functions::Directive;
use ast::alias;

node_enum!(@node_display pub enum Ast {
    Script(Script),
    Module(Module),
});


node!(#[derive(Default)] pub struct Script {
    pub directives: Vec<Directive>,
    pub body: Vec<alias::StatementItem>,
});
impl NodeDisplay for Script {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();
        f.node_list(&self.directives)?;
        f.node_list(&self.body)?;

        Ok(())
    }
}


node!(#[derive(Default)] pub struct Module {
    pub directives: Vec<Directive>,
    pub body: Vec<alias::ModuleStatementItem>,
});
impl NodeDisplay for Module {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();
        f.node_list(&self.directives)?;
        f.node_list(&self.body)?;

        Ok(())
    }
}
