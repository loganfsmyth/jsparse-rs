use std::string;

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   LookaheadRestriction};

use ast::patterns::{LeftHandComplexAssign, BindingPattern};

use ast::alias;



// { ... }
node!(#[derive(Default)] pub struct BlockStatement {
    pub body: Vec<alias::StatementItem>,
});
impl NodeDisplay for BlockStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();

        f.node_list(&self.body)?;

        Ok(())
    }
}
impl From<Vec<alias::StatementItem>> for BlockStatement {
    fn from(body: Vec<alias::StatementItem>) -> BlockStatement {
        BlockStatement {
            body,
            position: None,
        }
    }
}
#[cfg(test)]
mod tests_block {
    use super::*;
    use ast::general::ReferenceIdentifier;

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
                    ExpressionStatement::new(ReferenceIdentifier::new("someWord")).into(),
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

impl NodeDisplay for VariableStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Var);
        f.node(&self.declarations)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}

type VariableDeclaratorList = DeclaratorList<VariableDeclarator>;
node!(pub struct VariableDeclarator {
    pub id: BindingPattern,
    pub init: Option<alias::Expression>,
});

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
    use ast::general::{BindingIdentifier, ReferenceIdentifier};

    #[test]
    fn it_prints() {
        assert_serialize!(
            VariableStatement {
                declarations: DeclaratorList::Last(VariableDeclarator {
                    id: BindingIdentifier::from("myVar").into(),
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
                    id: BindingIdentifier::from("myVar").into(),
                    init: ReferenceIdentifier::from("initialVal").into(),
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
    //             init: Some(ReferenceIdentifier {
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
    pub id: BindingPattern,
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
    pub id: BindingPattern,
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
    pub fn new<T: Into<alias::Expression>>(expr: T) -> ExpressionStatement {
        ExpressionStatement {
            expression: expr.into(),
            position: None,
        }
    }
}
impl NodeDisplay for ExpressionStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        {
            let mut f = f.restrict_lookahead(LookaheadRestriction::ExpressionStatement);
            f.require_precedence(Precedence::Normal).node(
                &self.expression,
            )?;
        }

        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
impl<T: Into<alias::Expression>> From<T> for ExpressionStatement {
    fn from(expr: T) -> ExpressionStatement {
        ExpressionStatement {
            expression: expr.into(),
            position: None,
        }
    }
}
#[cfg(test)]
mod tests_expression_statement {
    use super::*;
    use ast::literal;
    use ast::general::ReferenceIdentifier;
    use ast::functions;
    use ast::classes;
    use ast::objects;
    use ast::patterns;
    use ast::expression;

    #[test]
    fn it_prints() {
        assert_serialize!(
            ExpressionStatement {
                expression: ReferenceIdentifier::from("foo").into(),
                position: None,
            },
            "foo;"
        );
    }

    #[test]
    fn it_prints_with_function_parens() {
        assert_serialize!(
            ExpressionStatement {
                expression: functions::FunctionExpression::default().into(),
                position: None,
            },
            "(function(){});"
        );
    }

    #[test]
    fn it_prints_with_class_parens() {
        assert_serialize!(
            ExpressionStatement {
                expression: classes::ClassExpression::default().into(),
                position: None,
            },
            "(class{});"
        );
    }

    #[test]
    fn it_prints_with_object_expression_parens() {
        assert_serialize!(
            ExpressionStatement {
                expression: objects::ObjectExpression::default().into(),
                position: None,
            },
            "({});"
        );
    }

    #[test]
    fn it_prints_with_object_pattern_parens() {
        assert_serialize!(
            ExpressionStatement {
                expression: expression::AssignmentExpression {
                    left: patterns::ObjectAssignmentPattern::default().into(),
                    right: ReferenceIdentifier::from("foo").into(),
                    position: None,
                }.into(),
                position: None,
            },
            "({}=foo);"
        );
    }

    #[test]
    fn it_prints_with_letsquare_parens() {
        assert_serialize!(
            ExpressionStatement {
                expression: expression::MemberExpression {
                    object: ReferenceIdentifier::from("let").into(),
                    property: expression::PropertyAccess::Computed(
                        expression::ComputedPropertyAccess {
                            optional: false,
                            expression: literal::Boolean::from(false).into(),
                            position: None,
                        },
                    ),
                    position: None,
                }.into(),
                position: None,
            },
            "(let[false]);"
        );
    }
}


// if () {}
node!(pub struct IfStatement {
    pub test: alias::Expression,

    // TODO: Technically Annex B allows function declarations in either of these
    pub consequent: Box<alias::Statement>,
    pub alternate: Option<Box<alias::Statement>>,
});
impl NodeDisplay for IfStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_orphan_if(self.alternate.is_none());
        f.keyword(Keyword::If);
        f.wrap_parens().node(&self.test)?;

        if let Some(ref stmt) = self.alternate {
            f.disallow_orphan_if().node(&self.consequent)?;
            f.keyword(Keyword::Else);
            f.node(stmt)?;
        } else {
            f.node(&self.consequent)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests_if {
    use super::*;
    use ast::general::ReferenceIdentifier;

    #[test]
    fn it_prints() {
        assert_serialize!(
            IfStatement {
                test: ReferenceIdentifier::from("myVar").into(),
                consequent: EmptyStatement::default().into(),
                alternate: None,
                position: None,
            },
            "if(myVar);"
        );
    }

    #[test]
    fn it_prints_else() {
        assert_serialize!(
            IfStatement {
                test: ReferenceIdentifier::from("myVar").into(),
                consequent: EmptyStatement::default().into(),
                alternate: EmptyStatement::default().into(),
                position: None,
            },
            "if(myVar);else;"
        );
    }

    #[test]
    fn it_prints_wrapped_if() {
        assert_serialize!(
            IfStatement {
                test: ReferenceIdentifier::from("myVar").into(),
                consequent: IfStatement {
                    test: ReferenceIdentifier::from("myVar2").into(),
                    consequent: EmptyStatement::default().into(),
                    alternate: None,
                    position: None,
                }.into(),
                alternate: EmptyStatement::default().into(),
                position: None,
            },
            "if(myVar){if(myVar2);}else;"
        );
    }
    #[test]
    fn it_prints_wrapped_if_deep() {
        assert_serialize!(
            IfStatement {
                test: ReferenceIdentifier::from("myVar").into(),
                consequent: WhileStatement {
                    test: ReferenceIdentifier::from("myVar2").into(),
                    body: IfStatement {
                        test: ReferenceIdentifier::from("myVar3").into(),
                        consequent: EmptyStatement::default().into(),
                        alternate: None,
                        position: None,
                    }.into(),
                    position: None,
                }.into(),
                alternate: EmptyStatement::default().into(),
                position: None,
            },
            "if(myVar)while(myVar2){if(myVar3);}else;"
        );
    }
}


// for( ; ; ) {}
node!(#[derive(Default)] pub struct ForStatement {
    pub init: Option<ForInit>,
    pub test: Option<Box<alias::Expression>>,
    pub update: Option<Box<alias::Expression>>,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for ForStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For);
        {
            let mut f = f.wrap_parens();
            if let Some(ref init) = self.init {
                let mut f = f.disallow_in();
                f.restrict_lookahead(LookaheadRestriction::ForInit).node(
                    init,
                )?;
            }
            f.punctuator(Punctuator::Semicolon);
            if let Some(ref test) = self.test {
                f.node(test)?;
            }
            f.punctuator(Punctuator::Semicolon);
            if let Some(ref update) = self.update {
                f.node(update)?;
            }
        }
        f.node(&self.body)
    }
}
node_enum!(@node_display pub enum ForInit {
    Var(VariableStatement),
    Let(LetDeclaration),
    Const(ConstDeclaration),
    Expression(alias::Expression),
});

#[cfg(test)]
mod tests_for {
    use super::*;
    use ast::general::ReferenceIdentifier;
    use ast::literal;

    #[test]
    fn it_prints_default() {
        assert_serialize!(ForStatement::default(), "for(;;);");
    }

    #[test]
    fn it_prints_test() {
        assert_serialize!(
            ForStatement {
                init: None,
                test: ReferenceIdentifier::from("myVar").into(),
                update: None,
                body: EmptyStatement::default().into(),
                position: None,
            },
            "for(;myVar;);"
        );
    }

    #[test]
    fn it_prints_complex() {
        assert_serialize!(
            ForStatement {
                init: alias::Expression::from(ReferenceIdentifier::from("init")).into(),
                test: ReferenceIdentifier::from("test").into(),
                update: ReferenceIdentifier::from("update").into(),
                body: BlockStatement {
                    body: vec![
                        literal::Boolean::from(true).into(),
                        literal::Numeric::from(4.5).into(),
                    ],
                    position: None,
                }.into(),
                position: None,
            },
            "for(init;test;update){true;4.5;}"
        );
    }
}


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
            f.restrict_lookahead(LookaheadRestriction::ForInit).node(
                &self.left,
            )?;
            f.keyword(Keyword::In);

            f.node(&self.right)?;
        }

        f.node(&self.body)
    }
}


node!(pub struct ForInVarPattern {
    pub pattern: BindingPattern,
    // TODO: Technically this default init is only allowed if the pattern is an identifier,
    // Should this change to a special pattern type? Annex B feature.
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
    pub pattern: BindingPattern,
});
impl NodeDisplay for ForVarPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Var);
        f.node(&self.pattern)
    }
}


node!(pub struct ForLetPattern {
    pub pattern: BindingPattern,
});
impl NodeDisplay for ForLetPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Let);
        f.node(&self.pattern)
    }
}


node!(pub struct ForConstPattern {
    pub pattern: BindingPattern,
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
            f.restrict_lookahead(LookaheadRestriction::ForOfInit).node(
                &self.left,
            )?;
            f.keyword(Keyword::Of);
            f.node(&self.right)?;
        }

        f.node(&self.body)
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
            f.restrict_lookahead(LookaheadRestriction::ForOfInit).node(
                &self.left,
            )?;
            f.keyword(Keyword::In);
            f.node(&self.right)?;
        }

        f.node(&self.body)
    }
}

node_enum!(@node_display pub enum ForOfInit {
    Var(ForVarPattern),
    Let(ForLetPattern),
    Const(ForConstPattern),
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
        f.wrap_parens().node(&self.test)?;
        f.node(&self.body)
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
        f.wrap_parens().node(&self.test)?;
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}


// switch (...) { ...    }
node!(pub struct SwitchStatement {
    pub discriminant: Box<alias::Expression>,
    pub cases: Vec<SwitchCase>,
});
impl NodeDisplay for SwitchStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Switch);
        f.wrap_parens().node(&self.discriminant)?;

        f.wrap_curly().node_list(&self.cases)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests_switch {
    use super::*;
    use ast::general::ReferenceIdentifier;
    use ast::literal;

    #[test]
    fn it_prints_empty() {
        assert_serialize!(
            SwitchStatement {
                discriminant: ReferenceIdentifier::from("myVar").into(),
                cases: vec![],
                position: None,
            },
            "switch(myVar){}"
        );
    }

    #[test]
    fn it_prints_cases() {
        assert_serialize!(
            SwitchStatement {
                discriminant: ReferenceIdentifier::from("myVar").into(),
                cases: vec![
                    SwitchCase {
                        test: literal::Numeric::from(6.2).into(),
                        consequent: vec![literal::Boolean::from(false).into()],
                        position: None,
                    },
                    SwitchCase {
                        test: None,
                        consequent: vec![literal::Boolean::from(true).into()],
                        position: None,
                    },
                    SwitchCase {
                        test: literal::Numeric::from(1.32).into(),
                        consequent: vec![literal::Boolean::from(true).into()],
                        position: None,
                    },
                ],
                position: None,
            },
            "switch(myVar){case 6.2:false;default:true;case 1.32:true;}"
        );
    }
}


// case foo:
// default:
node!(#[derive(Default)] pub struct SwitchCase {
    pub test: Option<Box<alias::Expression>>,
    pub consequent: Vec<alias::StatementItem>,
});
impl NodeDisplay for SwitchCase {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        if let Some(ref expr) = self.test {
            f.keyword(Keyword::Case);
            f.require_precedence(Precedence::Normal).node(expr)?;
        } else {
            f.keyword(Keyword::Default);
        }
        f.punctuator(Punctuator::Colon);

        f.node_list(&self.consequent)?;

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
        f.wrap_parens().node(&self.object)?;
        f.node(&self.body)
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

    // TODO: Annex B technically allows function declarations here, do we care?
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for LabelledStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.label)?;
        f.punctuator(Punctuator::Colon);
        f.node(&self.body)
    }
}


// throw foo;
node!(pub struct ThrowStatement {
    pub argument: Box<alias::Expression>,
});
impl NodeDisplay for ThrowStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Throw);
        f.require_precedence(Precedence::Normal).node(
            &self.argument,
        )?;

        Ok(())
    }
}


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

#[cfg(test)]
mod tests_try_catch {
    use super::*;
    use ast::general::BindingIdentifier;
    use ast::literal;

    #[test]
    fn it_prints_default() {
        assert_serialize!(TryCatchStatement::default(), "try{}catch{}");
    }

    #[test]
    fn it_prints_with_binding() {
        assert_serialize!(
            TryCatchStatement {
                block: vec![literal::Boolean::from(false).into()].into(),
                handler: CatchClause {
                    param: BindingIdentifier::from("err").into(),
                    body: vec![literal::Boolean::from(true).into()].into(),
                    position: None,
                },
                position: None,
            },
            "try{false;}catch(err){true;}"
        );
    }
}


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

impl From<TryCatchStatement> for TryCatchFinallyStatement {
    fn from(stmt: TryCatchStatement) -> TryCatchFinallyStatement {
        TryCatchFinallyStatement {
            block: stmt.block,
            handler: stmt.handler,
            finalizer: Default::default(),
            position: stmt.position,
        }
    }
}
impl From<TryFinallyStatement> for TryCatchFinallyStatement {
    fn from(stmt: TryFinallyStatement) -> TryCatchFinallyStatement {
        TryCatchFinallyStatement {
            block: stmt.block,
            handler: Default::default(),
            finalizer: stmt.finalizer,
            position: stmt.position,
        }
    }
}

#[cfg(test)]
mod tests_try_catch_finally {
    use super::*;

    #[test]
    fn it_prints_default() {
        assert_serialize!(TryCatchFinallyStatement::default(), "try{}catch{}finally{}");
    }

    #[test]
    fn it_prints_default_from_catch() {
        assert_serialize!(
            TryCatchFinallyStatement::from(TryCatchStatement::default()),
            "try{}catch{}finally{}"
        );
    }

    #[test]
    fn it_prints_default_from_finally() {
        assert_serialize!(
            TryCatchFinallyStatement::from(TryFinallyStatement::default()),
            "try{}catch{}finally{}"
        );
    }
}

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


node!(#[derive(Default)] pub struct CatchClause {
    // Missing param is experimental
    pub param: Option<BindingPattern>,
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


// return;
// return foo;
node!(#[derive(Default)] pub struct ReturnStatement {
    pub argument: Option<Box<alias::Expression>>,
});
impl NodeDisplay for ReturnStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Return);
        if let Some(ref expr) = self.argument {
            f.require_precedence(Precedence::Normal).node(expr)?;
        }
        Ok(())
    }
}


// debugger;
node!(#[derive(Default)] pub struct DebuggerStatement {});
impl NodeDisplay for DebuggerStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Debugger);
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}

// ;
node!(#[derive(Default)] pub struct EmptyStatement {});
impl NodeDisplay for EmptyStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Semicolon);
        Ok(())
    }
}
