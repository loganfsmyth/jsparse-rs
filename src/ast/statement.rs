use std::string;
use std::default;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   HasOrphanIf, FirstSpecialToken, SpecialToken};

use ast::patterns::{LeftHandComplexAssign, Pattern};

use ast::alias;



// { ... }
node!(#[derive(Default)] pub struct BlockStatement {
    pub body: Vec<alias::StatementItem>,
});
// display_dsl!(BlockStatement: @in { @[body] });

impl NodeDisplay for BlockStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();
        let mut f = f.allow_in();

        for item in self.body.iter() {
            f.node(item)?;
        }

        Ok(())
    }
}
impl HasOrphanIf for BlockStatement {}

#[cfg(test)]
mod tests_block {
    use super::*;
    use ast::general::BindingIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(
            BlockStatement {
                body: Default::default(),
                position: None,
            },
            "{}"
        );
    }
    #[test]
    fn it_prints_with_items() {
        assert_serialize!(
            BlockStatement {
                body: vec![
                    ExpressionStatement::new(BindingIdentifier::new("someWord")).into(),
                ],
                position: None,
            },
            "{someWord;}"
        );
    }
}


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
            f.require_precedence(Precedence::Assignment).node(init)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests_var {
    use super::*;
    use ast::general::BindingIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(
            VariableStatement {
                declarations: DeclaratorList::Last(VariableDeclarator {
                    id: BindingIdentifier {
                        value: "myVar".into(),
                        raw: None,
                        position: None,
                    }.into(),
                    init: None,
                    position: None,
                }),
                position: None,
            },
            "var myVar;"
        );
    }

    #[test]
    fn it_prints_with_init() {
        assert_serialize!(
            VariableStatement {
                declarations: DeclaratorList::Last(VariableDeclarator {
                    id: BindingIdentifier {
                        value: "myVar".into(),
                        raw: None,
                        position: None,
                    }.into(),
                    init: Some(
                        BindingIdentifier {
                            value: "initialVal".into(),
                            raw: None,
                            position: None,
                        }.into(),
                    ),
                    position: None,
                }),
                position: None,
            },
            "var myVar=initialVal;"
        );
    }

    // #[test]
    // fn it_prints_with_pattern() {
    //     assert_serialize!(VariableStatement {
    //         declarations: DeclaratorList::Last(VariableDeclarator {
    //             id: patt {
    //                 value: "myVar".into(),
    //                 raw: None,
    //                 position: None,
    //             }.into(),
    //             init: Some(BindingIdentifier {
    //                 value: "initialVal".into(),
    //                 raw: None,
    //                 position: None,
    //             }.into()),
    //             position: None,
    //         }),
    //         position: None,
    //     }, "var myVar=initialVal;");
    // }
}

// TODO: Enum fix?
#[derive(Debug)]
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
            f.require_precedence(Precedence::Assignment).node(init)?;
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
impl ExpressionStatement {
    fn new<T: Into<alias::Expression>>(expr: T) -> ExpressionStatement {
        ExpressionStatement {
            expression: expr.into(),
            position: None,
        }
    }
}
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
        {
            let mut f = f.wrap_parens();
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(&self.test)?;
        }

        if self.consequent.orphan_if() {
            f.wrap_curly().node(&self.consequent)?;
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
impl default::Default for ForStatement {
    fn default() -> ForStatement {
        ForStatement {
            init: None,
            test: None,
            update: None,
            body: Box::new(BlockStatement::default().into()),
            position: None,
        }
    }
}
impl NodeDisplay for ForStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For);
        {
            let mut f = f.wrap_parens();
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
                f.require_precedence(Precedence::Normal).node(update)?;
            }
        }
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
        {
            let mut f = f.wrap_parens();
            f.node(&self.left)?;
            f.keyword(Keyword::In);

            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(&self.right)?;
        }

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
        {
            let mut f = f.wrap_parens();
            f.node(&self.left)?;
            f.keyword(Keyword::Of);
            f.require_precedence(Precedence::Normal).node(&self.right)?;
        }

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
        {
            let mut f = f.wrap_parens();
            f.node(&self.left)?;
            f.keyword(Keyword::In);

            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(&self.right)?;
        }

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
        {
            let mut f = f.wrap_parens();
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(&self.test)?;
        }
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
        {
            let mut f = f.wrap_parens();
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(&self.test)?;
        }
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
        {
            let mut f = f.wrap_parens();
            let mut f = f.allow_in();
            f.require_precedence(Precedence::Normal).node(
                &self.discriminant,
            )?;
        }

        let mut f = f.wrap_curly();
        for c in self.cases.iter() {
            f.node(c)?;
        }

        Ok(())
    }
}
impl HasOrphanIf for SwitchStatement {}


// case foo:
// default:
node!(#[derive(Default)] pub struct SwitchCase {
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
        {
            let mut f = f.wrap_parens();
            f.require_precedence(Precedence::Normal).node(&self.object)?;
        }
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
    pub raw: Option<string::String>,
});
impl LabelIdentifier {
    pub fn new<T: Into<string::String>>(s: T) -> LabelIdentifier {
        LabelIdentifier {
            value: s.into(),
            raw: None,
            position: None,
        }
    }
}
impl NodeDisplay for LabelIdentifier {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.identifier(&self.value, self.raw.as_ref().map(String::as_str))
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
node!(#[derive(Default)] pub struct TryCatchStatement {
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
node!(#[derive(Default)] pub struct TryCatchFinallyStatement {
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
node!(#[derive(Default)] pub struct TryFinallyStatement {
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


node!(#[derive(Default)] pub struct CatchClause {
    // Missing param is experimental
    pub param: Option<Pattern>,
    pub body: BlockStatement,
});
impl NodeDisplay for CatchClause {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Catch);
        if let Some(ref pat) = self.param {
            f.wrap_parens().node(pat)?;
        }
        f.node(&self.body)
    }
}


// continue;
// continue foo;
node!(#[derive(Default)] pub struct ContinueStatement {
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
node!(#[derive(Default)] pub struct BreakStatement {
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
node!(#[derive(Default)] pub struct ReturnStatement {
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
node!(#[derive(Default)] pub struct DebuggerStatement {});
impl NodeDisplay for DebuggerStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Debugger);
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl HasOrphanIf for DebuggerStatement {}

// ;
node!(#[derive(Default)] pub struct EmptyStatement {});
impl NodeDisplay for EmptyStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl HasOrphanIf for EmptyStatement {}
