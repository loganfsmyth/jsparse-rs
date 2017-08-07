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
#[cfg(test)]
mod tests_script {
    use super::*;
    use ast::general::ReferenceIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(Script::default(), "");
    }

    #[test]
    fn it_prints_items() {
        assert_serialize!(
            Script {
                directives: vec!["use strict".into()],
                body: vec![ReferenceIdentifier::from("someVar").into()],
                position: None,
            },
            "'use strict';someVar;"
        );
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
#[cfg(test)]
mod tests_module {
    use super::*;
    use ast::general::ReferenceIdentifier;
    use ast::modules::ExportLocalBindings;

    #[test]
    fn it_prints() {
        assert_serialize!(Module::default(), "");
    }

    #[test]
    fn it_prints_items() {
        assert_serialize!(
            Module {
                directives: vec!["use strict".into()],
                body: vec![
                    ReferenceIdentifier::from("someVar").into(),
                    ExportLocalBindings::default().into(),
                ],
                position: None,
            },
            "'use strict';someVar;export{};"
        );
    }
}
