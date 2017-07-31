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
        assert_serialize!(BlockStatement, { body: Default::default() }, "{}");
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
nodes!(pub struct BlockStatement {
    body: Vec<alias::StatementItem>,
});
impl display::NodeDisplay for BlockStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();

        f.punctuator(display::Punctuator::CurlyL)?;
        for item in self.body.iter() {
            f.node(item)?;
        }
        f.punctuator(display::Punctuator::CurlyR)
    }
}
impl misc::HasOrphanIf for BlockStatement {}


// var foo, bar;
nodes!(pub struct VariableStatement {
    declarations: VariableDeclaratorList,
});
impl display::NodeDisplay for VariableStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.declarations)
    }
}
impl misc::HasOrphanIf for VariableStatement {}


pub enum VariableDeclaratorList {
    Declarator(VariableDeclarator),
    List(VariableDeclarator, Box<VariableDeclaratorList>),
}
impl display::NodeDisplay for VariableDeclaratorList {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            VariableDeclaratorList::Declarator(ref item) => f.node(item),
            VariableDeclaratorList::List(ref item, ref list) => {
                f.node(item)?;
                f.node(list)
            }
        }
    }
}


nodes!(pub struct VariableDeclarator {
    id: misc::Pattern,
    init: Option<alias::Expression>,
});
impl display::NodeDisplay for VariableDeclarator {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.id);
        if let Some(ref init) = self.init {
            f.punctuator(display::Punctuator::Eq)?;
            f.require_precedence(display::Precedence::Assignment).node(init)?;
        }
        Ok(())
    }
}


// foo;
nodes!(pub struct ExpressionStatement {
    expression: alias::Expression,
});
impl display::NodeDisplay for ExpressionStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();

        if let misc::SpecialToken::None = self.expression.first_special_token() {
            f.require_precedence(display::Precedence::Normal).node(&self.expression)?;
        } else {
            f.wrap_parens().node(&self.expression)?;
        }
        f.punctuator(display::Punctuator::Semicolon)
    }
}
impl misc::HasOrphanIf for ExpressionStatement {}


// if () {}
nodes!(pub struct IfStatement {
    test: alias::Expression,
    consequent: Box<alias::Statement>,
    alternate: Option<Box<alias::Statement>>,
});
impl display::NodeDisplay for IfStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::If)?;
        f.punctuator(display::Punctuator::ParenL)?;
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(&self.test)?;
        }
        f.punctuator(display::Punctuator::ParenR)?;

        if self.consequent.orphan_if() {
            f.punctuator(display::Punctuator::CurlyL)?;
            f.node(&self.consequent)?;
            f.punctuator(display::Punctuator::CurlyR)?;
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
nodes!(pub struct ForStatement {
    init: Option<ForInit>,
    test: Option<Box<alias::Expression>>,
    update: Option<Box<alias::Expression>>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for ForStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::For)?;
        f.punctuator(display::Punctuator::ParenL)?;
        if let Some(ref init) = self.init {
            f.node(init)?;
        }
        f.punctuator(display::Punctuator::Semicolon)?;
        if let Some(ref test) = self.test {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(test)?;
        }
        f.punctuator(display::Punctuator::Semicolon)?;
        if let Some(ref update) = self.update {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(update)?;
        }
        f.punctuator(display::Punctuator::ParenR)?;
        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for ForStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


pub enum ForInit {
    Var(VariableStatement),
    Let(LetDeclaration),
    Const(ConstDeclaration),
    Expression(alias::Expression),
}
impl display::NodeDisplay for ForInit {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.disallow_in();

        match *self {
            ForInit::Var(ref item) => f.node(item),
            ForInit::Let(ref item) => f.node(item),
            ForInit::Const(ref item) => f.node(item),
            ForInit::Expression(ref item) => {
                // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
                // so we need parens here for that too.
                f.require_precedence(display::Precedence::Normal).node(item)
            }
        }
    }
}


// for ... in
nodes!(pub struct ForInStatement {
    left: ForInInit,
    right: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for ForInStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::For)?;
        f.punctuator(display::Punctuator::ParenL)?;
        f.node(&self.left)?;
        f.keyword(display::Keyword::In)?;
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(&self.right)?;
        }
        f.punctuator(display::Punctuator::ParenR)?;

        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for ForInStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


pub enum ForInInit {
    Var(VariableDeclarator),
    Let(misc::Pattern),
    Const(misc::Pattern),
    Complex(misc::LeftHandComplexAssign),
}
impl display::NodeDisplay for ForInInit {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ForInInit::Var(ref decl) => f.node(decl),
            ForInInit::Let(ref pat) => f.node(pat),
            ForInInit::Const(ref pat) => f.node(pat),
            ForInInit::Complex(ref pat) => {
                // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
                // so we need parens here for that too.
                f.node(pat)
            }
        }
    }
}


// for ... of
nodes!(pub struct ForOfStatement {
    left: ForOfInit,
    right: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for ForOfStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::For)?;
        f.punctuator(display::Punctuator::ParenL)?;
        f.node(&self.left)?;
        f.keyword(display::Keyword::Of)?;
        f.require_precedence(display::Precedence::Normal).node(&self.right)?;
        f.punctuator(display::Punctuator::ParenR)?;

        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for ForOfStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// for await .. of
nodes!(pub struct ForAwaitStatement {
    left: ForOfInit,
    right: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for ForAwaitStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::For)?;
        f.keyword(display::Keyword::Await)?;
        f.punctuator(display::Punctuator::ParenL)?;
        f.node(&self.left)?;
        f.keyword(display::Keyword::In)?;
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(&self.right)?;
        }
        f.punctuator(display::Punctuator::ParenR)?;

        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for ForAwaitStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


pub enum ForOfInit {
    Var(misc::Pattern),
    Let(misc::Pattern),
    Const(misc::Pattern),
    Complex(misc::LeftHandComplexAssign),
}
impl display::NodeDisplay for ForOfInit {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ForOfInit::Var(ref pat) => f.node(pat),
            ForOfInit::Let(ref pat) => f.node(pat),
            ForOfInit::Const(ref pat) => f.node(pat),
            ForOfInit::Complex(ref pat) => {
                // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
                // so we need parens here for that too.
                f.node(pat)
            }
        }
    }
}


// while(...) ;
nodes!(pub struct WhileStatement {
    test: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for WhileStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::While)?;
        f.punctuator(display::Punctuator::ParenL)?;
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(&self.test)?;
        }
        f.punctuator(display::Punctuator::ParenR)?;
        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for WhileStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// do ; while(...) ;
nodes!(pub struct DoWhileStatement {
    test: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for DoWhileStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Do)?;

        f.node(&self.body)?;
        f.keyword(display::Keyword::While)?;
        f.punctuator(display::Punctuator::ParenL)?;
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(&self.test)?;
        }
        f.punctuator(display::Punctuator::ParenR)?;
        f.punctuator(display::Punctuator::Semicolon)
    }
}
impl misc::HasOrphanIf for DoWhileStatement {}


// switch (...) { ...    }
nodes!(pub struct SwitchStatement {
    discriminant: Box<alias::Expression>,
    cases: Vec<SwitchCase>,
});
impl display::NodeDisplay for SwitchStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Switch)?;
        f.punctuator(display::Punctuator::ParenL)?;
        {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(&self.discriminant)?;
        }
        f.punctuator(display::Punctuator::ParenR)?;
        f.punctuator(display::Punctuator::CurlyL)?;
        for c in self.cases.iter() {
            f.node(c)?;
        }
        f.punctuator(display::Punctuator::CurlyR)
    }
}
impl misc::HasOrphanIf for SwitchStatement {}


// case foo:
// default:
nodes!(pub struct SwitchCase {
    test: Option<Box<alias::Expression>>,
    consequent: Vec<alias::StatementItem>,
});
impl display::NodeDisplay for SwitchCase {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();

        if let Some(ref expr) = self.test {
            f.keyword(display::Keyword::Case)?;
            f.require_precedence(display::Precedence::Normal).node(expr)?;
        } else {
            f.keyword(display::Keyword::Default)?;
        }
        f.punctuator(display::Punctuator::Colon)?;

        for stmt in self.consequent.iter() {
            f.node(stmt)?;
        }

        Ok(())
    }
}


// with(...) ;
nodes!(pub struct WithStatement {
    object: Box<alias::Expression>,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for WithStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::With)?;
        f.punctuator(display::Punctuator::ParenL)?;
        f.require_precedence(display::Precedence::Normal).node(&self.object)?;
        f.punctuator(display::Punctuator::ParenR)?;
        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for WithStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// foo: while(false) ;
nodes!(pub struct LabelledStatement {
    label: misc::LabelIdentifier,
    body: Box<alias::Statement>,
});
impl display::NodeDisplay for LabelledStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.label)?;
        f.punctuator(display::Punctuator::Colon)?;
        f.node(&self.body)
    }
}
impl misc::HasOrphanIf for LabelledStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// throw foo;
nodes!(pub struct ThrowStatement {
    argument: Box<alias::Expression>,
});
impl display::NodeDisplay for ThrowStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        let mut f = f.allow_in();
        f.keyword(display::Keyword::Throw)?;
        f.require_precedence(display::Precedence::Normal).node(&self.argument)?;

        Ok(())
    }
}
impl misc::HasOrphanIf for ThrowStatement {}


// try {} catch(foo) {}
nodes!(pub struct TryCatchStatement {
    block: BlockStatement,
    handler: CatchClause,
});
impl display::NodeDisplay for TryCatchStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Try)?;
        f.node(&self.block)?;
        f.node(&self.handler)
    }
}
impl misc::HasOrphanIf for TryCatchStatement {}


// try {} catch(foo) {} finally {}
nodes!(pub struct TryCatchFinallyStatement {
    block: BlockStatement,
    handler: CatchClause,
    finalizer: BlockStatement,
});
impl display::NodeDisplay for TryCatchFinallyStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Try)?;
        f.node(&self.block)?;

        f.node(&self.handler)?;

        f.keyword(display::Keyword::Finally)?;
        f.node(&self.finalizer)
    }
}
impl misc::HasOrphanIf for TryCatchFinallyStatement {}


// try {} finally {}
nodes!(pub struct TryFinallyStatement {
    block: BlockStatement,
    finalizer: BlockStatement,
});
impl display::NodeDisplay for TryFinallyStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Try)?;
        f.node(&self.block)?;

        f.keyword(display::Keyword::Finally)?;
        f.node(&self.finalizer)
    }
}
impl misc::HasOrphanIf for TryFinallyStatement {}


nodes!(pub struct CatchClause {
    // Missing param is experimental
    param: Option<misc::Pattern>,
    body: BlockStatement,
});
impl display::NodeDisplay for CatchClause {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Catch)?;
        if let Some(ref pat) = self.param {
            f.punctuator(display::Punctuator::ParenL)?;
            f.node(pat)?;
            f.punctuator(display::Punctuator::ParenR)?;
        }
        f.node(&self.body)
    }
}


// continue;
// continue foo;
nodes!(pub struct ContinueStatement {
    label: Option<misc::LabelIdentifier>,
});
impl display::NodeDisplay for ContinueStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Continue)?;
        if let Some(ref label) = self.label {
            f.node(label)?;
        }
        Ok(())
    }
}
impl misc::HasOrphanIf for ContinueStatement {}


// break;
// break foo;
nodes!(pub struct BreakStatement {
    label: Option<misc::LabelIdentifier>,
});
impl display::NodeDisplay for BreakStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Break)?;
        if let Some(ref label) = self.label {
            f.node(label)?;
        }
        Ok(())
    }
}
impl misc::HasOrphanIf for BreakStatement {}


// return;
// return foo;
nodes!(pub struct ReturnStatement {
    argument: Option<Box<alias::Expression>>,
});
impl display::NodeDisplay for ReturnStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Return)?;
        if let Some(ref expr) = self.argument {
            let mut f = f.allow_in();
            f.require_precedence(display::Precedence::Normal).node(expr)?;
        }
        Ok(())
    }
}
impl misc::HasOrphanIf for ReturnStatement {}


// debugger;
nodes!(pub struct DebuggerStatement {});
impl display::NodeDisplay for DebuggerStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Debugger)?;
        f.punctuator(display::Punctuator::Semicolon)
    }
}
impl misc::HasOrphanIf for DebuggerStatement {}

// ;
nodes!(pub struct EmptyStatement {});
impl display::NodeDisplay for EmptyStatement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.punctuator(display::Punctuator::Semicolon)
    }
}
impl misc::HasOrphanIf for EmptyStatement {}
