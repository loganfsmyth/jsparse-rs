use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence, HasOrphanIf, FirstSpecialToken, SpecialToken};

use ast::patterns::{LeftHandComplexAssign, Pattern};

use ast::alias;

#[cfg(test)]
mod tests {
    use super::*;
    use ast::general::BindingIdentifier;

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
            declarations: DeclaratorList::Last(VariableDeclarator {
                id: BindingIdentifier {
                    value: "myVar".into(),
                    raw: None,
                    position: None,
                }.into(),
                init: None,
                position: None,
            }),
        }, "var myVar;");
    }

}


// { ... }
node!(pub struct BlockStatement {
    pub body: Vec<alias::StatementItem>,
});
// display_dsl!(BlockStatement: @in { @[body] });

impl NodeDisplay for BlockStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();

        f.punctuator(Punctuator::CurlyL);
        for item in self.body.iter() {
            f.node(item)?;
        }
        f.punctuator(Punctuator::CurlyR);
        Ok(())
    }
}
impl HasOrphanIf for BlockStatement {}


// var foo, bar;
node!(pub struct VariableStatement {
    pub declarations: VariableDeclaratorList,
});
// display_dsl!(VariableStatement: var @declarations ;);

impl NodeDisplay for VariableStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Var);
        f.node(&self.declarations)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl HasOrphanIf for VariableStatement {}

type VariableDeclaratorList = DeclaratorList<VariableDeclarator>;
node!(pub struct VariableDeclarator {
    pub id: Pattern,
    pub init: Option<alias::Expression>,
});
// display_dsl!(VariableDeclarator: @id @?init[= @]);

impl NodeDisplay for VariableDeclarator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        if let Some(ref init) = self.init {
            f.punctuator(Punctuator::Eq);
            f.require_precedence(Precedence::Assignment).node(
                init,
            )?;
        }
        Ok(())
    }
}

// TODO: Enum fix?
pub enum DeclaratorList<T: NodeDisplay> {
    Last(T),
    List(T, Box<DeclaratorList<T>>),
}
impl<T: NodeDisplay> NodeDisplay for DeclaratorList<T> {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        match *self {
            DeclaratorList::Last(ref n) => f.node(n),
            DeclaratorList::List(ref n, ref list) => {
                f.node(n)?;
                f.punctuator(Punctuator::Comma);
                f.node(list)
            }
        }
    }
}

// let foo, bar;
node!(pub struct LetDeclaration {
    pub declarators: LetDeclaratorList,
});
impl NodeDisplay for LetDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Let);
        f.node(&self.declarators)
    }
}


type LetDeclaratorList = DeclaratorList<LetDeclarator>;
node!(pub struct LetDeclarator {
    pub id: Pattern,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for LetDeclarator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        if let Some(ref init) = self.init {
            f.punctuator(Punctuator::Eq);
            f.require_precedence(Precedence::Assignment).node(
                init,
            )?;
        }
        Ok(())
    }
}


// const foo = 4, bar = 5;
node!(pub struct ConstDeclaration {
    pub declarators: ConstDeclaratorList,
});
impl NodeDisplay for ConstDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Const);
        f.node(&self.declarators)
    }
}


type ConstDeclaratorList = DeclaratorList<ConstDeclarator>;
node!(pub struct ConstDeclarator {
    pub id: Pattern,
    pub init: alias::Expression,
});
impl NodeDisplay for ConstDeclarator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        f.punctuator(Punctuator::Eq);
        f.require_precedence(Precedence::Assignment).node(
            &self.init,
        )
    }
}

// foo;
node!(pub struct ExpressionStatement {
    pub expression: alias::Expression,
});
impl NodeDisplay for ExpressionStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();

        if let SpecialToken::None = self.expression.first_special_token() {
            f.require_precedence(Precedence::Normal).node(
                &self.expression,
            )?;
        } else {
            f.wrap_parens().node(&self.expression)?;
        }
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl HasOrphanIf for ExpressionStatement {}


// if () {}
node!(pub struct IfStatement {
    pub test: alias::Expression,
    pub consequent: Box<alias::Statement>,
    pub alternate: Option<Box<alias::Statement>>,
});
impl NodeDisplay for IfStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::If);
        f.punctuator(Punctuator::ParenL);
        {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(
                &self.test,
            )?;
        }
        f.punctuator(Punctuator::ParenR);

        if self.consequent.orphan_if() {
            f.punctuator(Punctuator::CurlyL);
            f.node(&self.consequent)?;
            f.punctuator(Punctuator::CurlyR);
        } else {
            f.node(&self.consequent)?;
        }

        if let Some(ref stmt) = self.alternate {
            f.node(stmt)?;
        }
        Ok(())
    }
}
impl HasOrphanIf for IfStatement {
    fn orphan_if(&self) -> bool {
        self.consequent.orphan_if()
    }
}


// for( ; ; ) {}
node!(pub struct ForStatement {
    pub init: Option<ForInit>,
    pub test: Option<Box<alias::Expression>>,
    pub update: Option<Box<alias::Expression>>,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for ForStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For);
        f.punctuator(Punctuator::ParenL);
        if let Some(ref init) = self.init {
            let mut f = f.disallow_in();
            f.node(init)?;
        }
        f.punctuator(Punctuator::Semicolon);
        if let Some(ref test) = self.test {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(test)?;
        }
        f.punctuator(Punctuator::Semicolon);
        if let Some(ref update) = self.update {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(
                update,
            )?;
        }
        f.punctuator(Punctuator::ParenR);
        f.node(&self.body)
    }
}
impl HasOrphanIf for ForStatement {
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
    pub left: ForInInit,
    pub right: Box<alias::Expression>,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for ForInStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For);
        f.punctuator(Punctuator::ParenL);
        f.node(&self.left)?;
        f.keyword(Keyword::In);
        {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(
                &self.right,
            )?;
        }
        f.punctuator(Punctuator::ParenR);

        f.node(&self.body)
    }
}
impl HasOrphanIf for ForInStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}

node!(pub struct ForInVarPattern {
    pub pattern: Pattern,
    pub init: Option<alias::Expression>,
});
impl NodeDisplay for ForInVarPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Var);
        f.node(&self.pattern)?;
        if let Some(ref init) = self.init {
            f.punctuator(Punctuator::Eq);
            f.node(init)?;
        }
        Ok(())
    }
}


node!(pub struct ForVarPattern {
    pub pattern: Pattern,
});
impl NodeDisplay for ForVarPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Var);
        f.node(&self.pattern)
    }
}


node!(pub struct ForLetPattern {
    pub pattern: Pattern,
});
impl NodeDisplay for ForLetPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Let);
        f.node(&self.pattern)
    }
}


node!(pub struct ForConstPattern {
    pub pattern: Pattern,
});
impl NodeDisplay for ForConstPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Const);
        f.node(&self.pattern)
    }
}


node_enum!(@node_display pub enum ForInInit {
    Var(ForInVarPattern),
    Let(ForLetPattern),
    Const(ForConstPattern),

    // TODO: Technically in sloppy mode someone could do "let[..]" here as a member expression,
    // so we need parens here for that too.
    Complex(LeftHandComplexAssign),
});


// for ... of
node!(pub struct ForOfStatement {
    pub left: ForOfInit,
    pub right: Box<alias::Expression>,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for ForOfStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For);
        f.punctuator(Punctuator::ParenL);
        f.node(&self.left)?;
        f.keyword(Keyword::Of);
        f.require_precedence(Precedence::Normal).node(
            &self.right,
        )?;
        f.punctuator(Punctuator::ParenR);

        f.node(&self.body)
    }
}
impl HasOrphanIf for ForOfStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// for await .. of
node!(pub struct ForAwaitStatement {
    pub left: ForOfInit,
    pub right: Box<alias::Expression>,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for ForAwaitStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For);
        f.keyword(Keyword::Await);
        f.punctuator(Punctuator::ParenL);
        f.node(&self.left)?;
        f.keyword(Keyword::In);
        {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(
                &self.right,
            )?;
        }
        f.punctuator(Punctuator::ParenR);

        f.node(&self.body)
    }
}
impl HasOrphanIf for ForAwaitStatement {
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
    Complex(LeftHandComplexAssign),
});


// while(...) ;
node!(pub struct WhileStatement {
    pub test: Box<alias::Expression>,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for WhileStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::While);
        f.punctuator(Punctuator::ParenL);
        {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(
                &self.test,
            )?;
        }
        f.punctuator(Punctuator::ParenR);
        f.node(&self.body)
    }
}
impl HasOrphanIf for WhileStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// do ; while(...) ;
node!(pub struct DoWhileStatement {
    pub test: Box<alias::Expression>,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for DoWhileStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Do);

        f.node(&self.body)?;
        f.keyword(Keyword::While);
        f.punctuator(Punctuator::ParenL);
        {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(
                &self.test,
            )?;
        }
        f.punctuator(Punctuator::ParenR);
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl HasOrphanIf for DoWhileStatement {}


// switch (...) { ...    }
node!(pub struct SwitchStatement {
    pub discriminant: Box<alias::Expression>,
    pub cases: Vec<SwitchCase>,
});
impl NodeDisplay for SwitchStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Switch);
        f.punctuator(Punctuator::ParenL);
        {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(
                &self.discriminant,
            )?;
        }
        f.punctuator(Punctuator::ParenR);
        f.punctuator(Punctuator::CurlyL);
        for c in self.cases.iter() {
            f.node(c)?;
        }
        f.punctuator(Punctuator::CurlyR);
        Ok(())
    }
}
impl HasOrphanIf for SwitchStatement {}


// case foo:
// default:
node!(pub struct SwitchCase {
    pub test: Option<Box<alias::Expression>>,
    pub consequent: Vec<alias::StatementItem>,
});
impl NodeDisplay for SwitchCase {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();

        if let Some(ref expr) = self.test {
            f.keyword(Keyword::Case);
            f.require_precedence(Precedence::Normal).node(expr)?;
        } else {
            f.keyword(Keyword::Default);
        }
        f.punctuator(Punctuator::Colon);

        for stmt in self.consequent.iter() {
            f.node(stmt)?;
        }

        Ok(())
    }
}


// with(...) ;
node!(pub struct WithStatement {
    pub object: Box<alias::Expression>,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for WithStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::With);
        f.punctuator(Punctuator::ParenL);
        f.require_precedence(Precedence::Normal).node(
            &self.object,
        )?;
        f.punctuator(Punctuator::ParenR);
        f.node(&self.body)
    }
}
impl HasOrphanIf for WithStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// identifiers used as labels
node!(pub struct LabelIdentifier {
    pub value: string::String,
    pub raw: string::String,
});
impl NodeDisplay for LabelIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.identifier(&self.value, Some(&self.raw))
    }
}


// foo: while(false) ;
node!(pub struct LabelledStatement {
    pub label: LabelIdentifier,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for LabelledStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.label)?;
        f.punctuator(Punctuator::Colon);
        f.node(&self.body)
    }
}
impl HasOrphanIf for LabelledStatement {
    fn orphan_if(&self) -> bool {
        self.body.orphan_if()
    }
}


// throw foo;
node!(pub struct ThrowStatement {
    pub argument: Box<alias::Expression>,
});
impl NodeDisplay for ThrowStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.allow_in();
        f.keyword(Keyword::Throw);
        f.require_precedence(Precedence::Normal).node(
            &self.argument,
        )?;

        Ok(())
    }
}
impl HasOrphanIf for ThrowStatement {}


// try {} catch(foo) {}
node!(pub struct TryCatchStatement {
    pub block: BlockStatement,
    pub handler: CatchClause,
});
impl NodeDisplay for TryCatchStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Try);
        f.node(&self.block)?;
        f.node(&self.handler)
    }
}
impl HasOrphanIf for TryCatchStatement {}


// try {} catch(foo) {} finally {}
node!(pub struct TryCatchFinallyStatement {
    pub block: BlockStatement,
    pub handler: CatchClause,
    pub finalizer: BlockStatement,
});
impl NodeDisplay for TryCatchFinallyStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Try);
        f.node(&self.block)?;

        f.node(&self.handler)?;

        f.keyword(Keyword::Finally);
        f.node(&self.finalizer)
    }
}
impl HasOrphanIf for TryCatchFinallyStatement {}


// try {} finally {}
node!(pub struct TryFinallyStatement {
    pub block: BlockStatement,
    pub finalizer: BlockStatement,
});
impl NodeDisplay for TryFinallyStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Try);
        f.node(&self.block)?;

        f.keyword(Keyword::Finally);
        f.node(&self.finalizer)
    }
}
impl HasOrphanIf for TryFinallyStatement {}


node!(pub struct CatchClause {
    // Missing param is experimental
    pub param: Option<Pattern>,
    pub body: BlockStatement,
});
impl NodeDisplay for CatchClause {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Catch);
        if let Some(ref pat) = self.param {
            f.punctuator(Punctuator::ParenL);
            f.node(pat)?;
            f.punctuator(Punctuator::ParenR);
        }
        f.node(&self.body)
    }
}


// continue;
// continue foo;
node!(pub struct ContinueStatement {
    pub label: Option<LabelIdentifier>,
});
impl NodeDisplay for ContinueStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Continue);
        if let Some(ref label) = self.label {
            f.node(label)?;
        }
        Ok(())
    }
}
impl HasOrphanIf for ContinueStatement {}


// break;
// break foo;
node!(pub struct BreakStatement {
    pub label: Option<LabelIdentifier>,
});
impl NodeDisplay for BreakStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Break);
        if let Some(ref label) = self.label {
            f.node(label)?;
        }
        Ok(())
    }
}
impl HasOrphanIf for BreakStatement {}


// return;
// return foo;
node!(pub struct ReturnStatement {
    pub argument: Option<Box<alias::Expression>>,
});
impl NodeDisplay for ReturnStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Return);
        if let Some(ref expr) = self.argument {
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(expr)?;
        }
        Ok(())
    }
}
impl HasOrphanIf for ReturnStatement {}


// debugger;
node!(pub struct DebuggerStatement {});
impl NodeDisplay for DebuggerStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Debugger);
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl HasOrphanIf for DebuggerStatement {}

// ;
node!(pub struct EmptyStatement {});
impl NodeDisplay for EmptyStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl HasOrphanIf for EmptyStatement {}
