use super::misc;
use super::alias;
use super::display;

use super::declaration::{LetDeclaration, ConstDeclaration};
use super::misc::HasOrphanIf;
use super::misc::FirstSpecialToken;

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn it_prints_block() {
        assert_serialize!(
            BlockStatement,
            {
                body: Default::default()
            },
            "{}"
        );
    }

    #[test]
    fn it_prints_var() {
        assert_serialize!(VariableStatement, {
            declarations: VariableDeclaratorList::Declarator(VariableDeclarator {
                id: misc::BindingIdentifier {
                    value: "myVar".into(),
                    raw: None,
                    position: None,
                }.into(),
                init: None,
                position: None,
            }),
        }, "{}");
    }

}


// { ... }
node!(pub struct BlockStatement {
    body: Vec<alias::StatementItem>,
});
impl display::NodeDisplay for BlockStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();

        f.punctuator(display::Punctuator::CurlyL);
        for item in self.body.iter() {
            f.node(item)?;
        }
        f.punctuator(display::Punctuator::CurlyR);
        Ok(())
    }
}
impl misc::HasOrphanIf for BlockStatement {}


// var foo, bar;
node!(pub struct VariableStatement {
    declarations: VariableDeclaratorList,
});
impl display::NodeDisplay for VariableStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.declarations)
    }
}
impl misc::HasOrphanIf for VariableStatement {}


// TODO: Enum fix?
pub enum VariableDeclaratorList {
    Declarator(VariableDeclarator),
    List(VariableDeclarator, Box<VariableDeclaratorList>),
}
impl display::NodeDisplay for VariableDeclaratorList {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            VariableDeclaratorList::Declarator(ref n) => f.node(n),
            VariableDeclaratorList::List(ref n, ref list) => {
                f.node(n)?;
                f.node(list)
            }
        }
    }
}


node!(pub struct VariableDeclarator {
    id: misc::Pattern,
    init: Option<alias::Expression>,
});
impl display::NodeDisplay for VariableDeclarator {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.id)?;
        if let Some(ref init) = self.init {
            f.punctuator(display::Punctuator::Eq);
            f.require_precedence(display::Precedence::Assignment).node(
                init,
            )?;
        }
        Ok(())
    }
}


// foo;
node!(pub struct ExpressionStatement {
    expression: alias::Expression,
});
impl display::NodeDisplay for ExpressionStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();

        if let misc::SpecialToken::None = self.expression.first_special_token() {
            f.require_precedence(display::Precedence::Normal).node(
                &self.expression,
            )?;
        } else {
            f.wrap_parens().node(&self.expression)?;
        }
        f.punctuator(display::Punctuator::Semicolon);
        Ok(())
    }
}
impl misc::HasOrphanIf for ExpressionStatement {}


// if () {}
node!(pub struct IfStatement {
    test: alias::Expression,
    consequent: Box<alias::Statement>,
    alternate: Option<Box<alias::Statement>>,
});
impl display::NodeDisplay for IfStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::If);
        f.punctuator(display::Punctuator::ParenL);
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(
                &self.test,
            )?;
        }
        f.punctuator(display::Punctuator::ParenR);

        if self.consequent.orphan_if() {
            f.punctuator(display::Punctuator::CurlyL);
            f.node(&self.consequent)?;
            f.punctuator(display::Punctuator::CurlyR);
        } else {
            f.node(&self.consequent)?;
        }

        if let Some(ref stmt) = self.alternate {
            f.node(stmt)?;
        }
        Ok(())
    }
}
impl misc::HasOrphanIf for IfStatement {
    fn orphan_if(&self) -> bool {
        self.consequent.orphan_if()
    }
}


// for( ; ; ) {}
node!(pub struct ForStatement {
    init: Option<ForInit>,
    test: Option<Box<alias::Expression>>,
    update: Option<Box<alias::Expression>>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for ForStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::For);
        f.punctuator(display::Punctuator::ParenL);
        if let Some(ref init) = self.init {
            let mut f = f.disallow_in();
            f.node(init)?;
        }
        f.punctuator(display::Punctuator::Semicolon);
        if let Some(ref test) = self.test {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(test)?;
        }
        f.punctuator(display::Punctuator::Semicolon);
        if let Some(ref update) = self.update {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(
                update,
            )?;
        }
        f.punctuator(display::Punctuator::ParenR);
        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for ForStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


node_enum!(@node_display pub enum ForInit {
    Var(VariableStatement),
    Let(LetDeclaration),
    Const(ConstDeclaration),

    // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
    // so we need parens here for that too.
    Expression(alias::Expression),
});


// for ... in
node!(pub struct ForInStatement {
    left: ForInInit,
    right: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for ForInStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::For);
        f.punctuator(display::Punctuator::ParenL);
        f.node(&self.left)?;
        f.keyword(display::Keyword::In);
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(
                &self.right,
            )?;
        }
        f.punctuator(display::Punctuator::ParenR);

        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for ForInStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}

node!(pub struct ForInVarPattern {
    pattern: misc::Pattern,
    init: Option<alias::Expression>,
});
impl display::NodeDisplay for ForInVarPattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Var);
        f.node(&self.pattern)?;
        if let Some(ref init) = self.init {
            f.punctuator(display::Punctuator::Eq);
            f.node(init)?;
        }
        Ok(())
    }
}


node!(pub struct ForVarPattern {
    pattern: misc::Pattern,
});
impl display::NodeDisplay for ForVarPattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Var);
        f.node(&self.pattern)
    }
}


node!(pub struct ForLetPattern {
    pattern: misc::Pattern,
});
impl display::NodeDisplay for ForLetPattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Let);
        f.node(&self.pattern)
    }
}


node!(pub struct ForConstPattern {
    pattern: misc::Pattern,
});
impl display::NodeDisplay for ForConstPattern {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Const);
        f.node(&self.pattern)
    }
}


node_enum!(@node_display pub enum ForInInit {
    Var(ForInVarPattern),
    Let(ForLetPattern),
    Const(ForConstPattern),

    // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
    // so we need parens here for that too.
    Complex(misc::LeftHandComplexAssign),
});


// for ... of
node!(pub struct ForOfStatement {
    left: ForOfInit,
    right: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for ForOfStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::For);
        f.punctuator(display::Punctuator::ParenL);
        f.node(&self.left)?;
        f.keyword(display::Keyword::Of);
        f.require_precedence(display::Precedence::Normal).node(
            &self.right,
        )?;
        f.punctuator(display::Punctuator::ParenR);

        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for ForOfStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// for await .. of
node!(pub struct ForAwaitStatement {
    left: ForOfInit,
    right: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for ForAwaitStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::For);
        f.keyword(display::Keyword::Await);
        f.punctuator(display::Punctuator::ParenL);
        f.node(&self.left)?;
        f.keyword(display::Keyword::In);
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(
                &self.right,
            )?;
        }
        f.punctuator(display::Punctuator::ParenR);

        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for ForAwaitStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}

node_enum!(@node_display pub enum ForOfInit {
    Var(ForVarPattern),
    Let(ForLetPattern),
    Const(ForConstPattern),

    // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
    // so we need parens here for that too.
    Complex(misc::LeftHandComplexAssign),
});


// while(...) ;
node!(pub struct WhileStatement {
    test: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for WhileStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::While);
        f.punctuator(display::Punctuator::ParenL);
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(
                &self.test,
            )?;
        }
        f.punctuator(display::Punctuator::ParenR);
        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for WhileStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// do ; while(...) ;
node!(pub struct DoWhileStatement {
    test: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for DoWhileStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Do);

        f.node(&self.body)?;
        f.keyword(display::Keyword::While);
        f.punctuator(display::Punctuator::ParenL);
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(
                &self.test,
            )?;
        }
        f.punctuator(display::Punctuator::ParenR);
        f.punctuator(display::Punctuator::Semicolon);
        Ok(())
    }
}
impl misc::HasOrphanIf for DoWhileStatement {}


// switch (...) { ...    }
node!(pub struct SwitchStatement {
    discriminant: Box<alias::Expression>,
    cases: Vec<SwitchCase>,
});
impl display::NodeDisplay for SwitchStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Switch);
        f.punctuator(display::Punctuator::ParenL);
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(
                &self.discriminant,
            )?;
        }
        f.punctuator(display::Punctuator::ParenR);
        f.punctuator(display::Punctuator::CurlyL);
        for c in self.cases.iter() {
            f.node(c)?;
        }
        f.punctuator(display::Punctuator::CurlyR);
        Ok(())
    }
}
impl misc::HasOrphanIf for SwitchStatement {}


// case foo:
// default:
node!(pub struct SwitchCase {
    test: Option<Box<alias::Expression>>,
    consequent: Vec<alias::StatementItem>,
});
impl display::NodeDisplay for SwitchCase {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();

        if let Some(ref expr) = self.test {
            f.keyword(display::Keyword::Case);
            f.require_precedence(display::Precedence::Normal).node(expr)?;
        } else {
            f.keyword(display::Keyword::Default);
        }
        f.punctuator(display::Punctuator::Colon);

        for stmt in self.consequent.iter() {
            f.node(stmt)?;
        }

        Ok(())
    }
}


// with(...) ;
node!(pub struct WithStatement {
    object: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for WithStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::With);
        f.punctuator(display::Punctuator::ParenL);
        f.require_precedence(display::Precedence::Normal).node(
            &self.object,
        )?;
        f.punctuator(display::Punctuator::ParenR);
        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for WithStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// foo: while(false) ;
node!(pub struct LabelledStatement {
    label: misc::LabelIdentifier,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for LabelledStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.label)?;
        f.punctuator(display::Punctuator::Colon);
        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for LabelledStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// throw foo;
node!(pub struct ThrowStatement {
    argument: Box<alias::Expression>,
});
impl display::NodeDisplay for ThrowStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();
        f.keyword(display::Keyword::Throw);
        f.require_precedence(display::Precedence::Normal).node(
            &self.argument,
        )?;

        Ok(())
    }
}
impl misc::HasOrphanIf for ThrowStatement {}


// try {} catch(foo) {}
node!(pub struct TryCatchStatement {
    block: BlockStatement,
    handler: CatchClause,
});
impl display::NodeDisplay for TryCatchStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Try);
        f.node(&self.block)?;
        f.node(&self.handler)
    }
}
impl misc::HasOrphanIf for TryCatchStatement {}


// try {} catch(foo) {} finally {}
node!(pub struct TryCatchFinallyStatement {
    block: BlockStatement,
    handler: CatchClause,
    finalizer: BlockStatement,
});
impl display::NodeDisplay for TryCatchFinallyStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Try);
        f.node(&self.block)?;

        f.node(&self.handler)?;

        f.keyword(display::Keyword::Finally);
        f.node(&self.finalizer)
    }
}
impl misc::HasOrphanIf for TryCatchFinallyStatement {}


// try {} finally {}
node!(pub struct TryFinallyStatement {
    block: BlockStatement,
    finalizer: BlockStatement,
});
impl display::NodeDisplay for TryFinallyStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Try);
        f.node(&self.block)?;

        f.keyword(display::Keyword::Finally);
        f.node(&self.finalizer)
    }
}
impl misc::HasOrphanIf for TryFinallyStatement {}


node!(pub struct CatchClause {
    // Missing param is experimental
    param: Option<misc::Pattern>,
    body: BlockStatement,
});
impl display::NodeDisplay for CatchClause {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Catch);
        if let Some(ref pat) = self.param {
            f.punctuator(display::Punctuator::ParenL);
            f.node(pat)?;
            f.punctuator(display::Punctuator::ParenR);
        }
        f.node(&self.body)
    }
}


// continue;
// continue foo;
node!(pub struct ContinueStatement {
    label: Option<misc::LabelIdentifier>,
});
impl display::NodeDisplay for ContinueStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Continue);
        if let Some(ref label) = self.label {
            f.node(label)?;
        }
        Ok(())
    }
}
impl misc::HasOrphanIf for ContinueStatement {}


// break;
// break foo;
node!(pub struct BreakStatement {
    label: Option<misc::LabelIdentifier>,
});
impl display::NodeDisplay for BreakStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Break);
        if let Some(ref label) = self.label {
            f.node(label)?;
        }
        Ok(())
    }
}
impl misc::HasOrphanIf for BreakStatement {}


// return;
// return foo;
node!(pub struct ReturnStatement {
    argument: Option<Box<alias::Expression>>,
});
impl display::NodeDisplay for ReturnStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Return);
        if let Some(ref expr) = self.argument {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(expr)?;
        }
        Ok(())
    }
}
impl misc::HasOrphanIf for ReturnStatement {}


// debugger;
node!(pub struct DebuggerStatement {});
impl display::NodeDisplay for DebuggerStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Debugger);
        f.punctuator(display::Punctuator::Semicolon);
        Ok(())
    }
}
impl misc::HasOrphanIf for DebuggerStatement {}

// ;
node!(pub struct EmptyStatement {});
impl display::NodeDisplay for EmptyStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::Semicolon);
        Ok(())
    }
}
impl misc::HasOrphanIf for EmptyStatement {}
