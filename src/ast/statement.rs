use std::string;

use ast::{MaybeTokenPosition, KeywordData, KeywordWrappedData, SeparatorTokens};

use ast::display::{NodeDisplay, NodeFormatter, NodeDisplayResult, Keyword, Punctuator, Precedence,
                   LookaheadRestriction};

use ast::patterns::{LeftHandComplexAssign, BindingPattern};
use ast::general;
use ast::alias;



// { ... }
node!(#[derive(Default)] pub struct BlockStatement {
    pub token_curly_l: KeywordData,
    pub body: Vec<alias::StatementItem>,
    pub token_curly_r: KeywordData,
});
impl NodeDisplay for BlockStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_curly();

        f.node_list(&self.body)?;

        Ok(())
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
    pub token_var: KeywordData,
    pub declarators: Vec<(VariableDeclarator, KeywordData)>,
    pub last_declarator: VariableDeclarator,
    pub token_semi: KeywordData,
});
impl NodeDisplay for VariableStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Var, &self.token_var);
        f.comma_list(&self.declarators)?;
        f.node(&self.last_declarator)?;
        f.punctuator(Punctuator::Semicolon, &self.token_semi);
        Ok(())
    }
}

node!(pub struct VariableDeclarator {
    pub id: BindingPattern,
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for VariableDeclarator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        f.node(&self.init)
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

// let foo, bar;
node!(pub struct LetDeclaration {
    pub token_let: KeywordData,
    pub declarators: Vec<(LetDeclarator, KeywordData)>,
    pub last_declarator: LetDeclarator,
    pub token_semi: KeywordData,
});
impl NodeDisplay for LetDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Let, &self.token_let);
        f.comma_list(&self.declarators)?;
        f.node(&self.last_declarator)?;
        f.punctuator(Punctuator::Semicolon, &self.token_semi);
        Ok(())
    }
}


node!(pub struct LetDeclarator {
    pub id: BindingPattern,
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for LetDeclarator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        f.node(&self.init)
    }
}


// const foo = 4, bar = 5;
node!(pub struct ConstDeclaration {
    pub token_const: KeywordData,
    pub declarators: Vec<(ConstDeclarator, KeywordData)>,
    pub last_declarator: ConstDeclarator,
    pub token_semi: KeywordData,
});
impl NodeDisplay for ConstDeclaration {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Const, &self.token_const);
        f.comma_list(&self.declarators)?;
        f.node(&self.last_declarator)?;
        f.punctuator(Punctuator::Semicolon, &self.token_semi);
        Ok(())
    }
}


node!(pub struct ConstDeclarator {
    pub id: BindingPattern,
    pub init: general::Initializer,
});
impl NodeDisplay for ConstDeclarator {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.node(&self.id)?;
        f.node(&self.init)
    }
}

// foo;
node!(pub struct ExpressionStatement {
    pub token_prefix: SeparatorTokens,
    pub expression: alias::Expression,
    pub token_semi: KeywordData,
});
impl ExpressionStatement {
    pub fn new<T: Into<alias::Expression>>(expr: T) -> ExpressionStatement {
        ExpressionStatement {
            token_prefix: Default::default(),
            expression: expr.into(),
            token_semi: Default::default(),

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
// impl From<alias::Expression> for ExpressionStatement {
//     fn from(e: alias::Expression) -> ExpressionStatement {
//
//     }
// }

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


// if () ;
node!(pub struct IfStatement {
    pub token_if: KeywordData,
    pub token_paren_l: KeywordData,
    pub test: alias::Expression,
    pub token_paren_r: KeywordData,
    pub consequent: Box<alias::Statement>,
});
impl NodeDisplay for IfStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.wrap_orphan_if();
        f.keyword(Keyword::If, &self.token_if);
        f.wrap_parens().node(&self.test)?;
        f.node(&self.consequent)
    }
}

// if () ; else ;
node!(pub struct IfElseStatement {
    pub token_if: KeywordData,
    pub token_paren_l: KeywordData,
    pub test: alias::Expression,
    pub token_paren_r: KeywordData,
    pub consequent: Box<alias::Statement>,
    pub token_else: KeywordData,
    pub alternate: Box<alias::Statement>,
});
impl NodeDisplay for IfElseStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::If, &self.token_if);
        f.wrap_parens().node(&self.test)?;
        f.disallow_orphan_if().node(&self.consequent)?;
        f.keyword(Keyword::Else, &self.token_else);
        f.node(&self.alternate)
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
    pub token_for: KeywordData,
    pub token_paren_l: KeywordData,
    pub init: Option<ForInit>,
    pub token_init_semi: KeywordWrappedData,
    pub test: Option<alias::Expression>,
    pub token_test_semi: KeywordWrappedData,
    pub update: Option<alias::Expression>,
    pub token_paren_r: KeywordData,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for ForStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For, &self.token_for);
        {
            let mut f = f.wrap_parens();
            f.node(&self.init)?;
            f.punctuator(Punctuator::Semicolon, &self.token_init_semi);
            f.node(&self.test)?;
            f.punctuator(Punctuator::Semicolon, &self.token_test_semi);
            f.node(&self.update)?;
        }
        f.node(&self.body)
    }
}


node_enum!(pub enum ForInit {
    Var(VariableStatement),
    Let(LetDeclaration),
    Const(ConstDeclaration),
    Expression(alias::Expression),
});
impl NodeDisplay for ForInit {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        let mut f = f.restrict_lookahead(LookaheadRestriction::ForInit);
        let mut f = f.disallow_in();

        match *self {
            ForInit::Var(ref n) => f.node(n),
            ForInit::Let(ref n) => f.node(n),
            ForInit::Const(ref n) => f.node(n),
            ForInit::Expression(ref n) => f.node(n),
        }
    }
}

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
    pub token_for: KeywordData,
    pub token_paren_l: KeywordData,
    pub left: ForInInit,
    pub token_in: KeywordWrappedData,
    pub right: Box<alias::Expression>,
    pub token_paren_r: KeywordData,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for ForInStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For, &self.token_for);
        {
            let mut f = f.wrap_parens();
            f.restrict_lookahead(LookaheadRestriction::ForInit).node(
                &self.left,
            )?;
            f.keyword(Keyword::In, &self.token_in);
            f.node(&self.right)?;
        }

        f.node(&self.body)
    }
}


node!(pub struct ForInVarPattern {
    pub token_var: KeywordData,
    pub pattern: BindingPattern,
    // TODO: Technically this default init is only allowed if the pattern is an identifier,
    // Should this change to a special pattern type? Annex B feature.
    pub init: Option<general::Initializer>,
});
impl NodeDisplay for ForInVarPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Var, &self.token_var);
        f.node(&self.pattern)?;
        f.node(&self.init)?;

        Ok(())
    }
}


node!(pub struct ForVarPattern {
    pub token_var: KeywordData,
    pub pattern: BindingPattern,
});
impl NodeDisplay for ForVarPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Var, &self.token_var);
        f.node(&self.pattern)
    }
}


node!(pub struct ForLetPattern {
    pub token_let: KeywordData,
    pub pattern: BindingPattern,
});
impl NodeDisplay for ForLetPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Let, &self.token_let);
        f.node(&self.pattern)
    }
}


node!(pub struct ForConstPattern {
    pub token_const: KeywordData,
    pub pattern: BindingPattern,
});
impl NodeDisplay for ForConstPattern {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Const, &self.token_const);
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
    pub token_for: KeywordData,
    pub token_paren_l: KeywordData,
    pub left: ForOfInit,
    pub token_of: KeywordWrappedData,
    pub right: Box<alias::Expression>,
    pub token_paren_r: KeywordData,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for ForOfStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For, &self.token_for);
        {
            let mut f = f.wrap_parens();
            f.restrict_lookahead(LookaheadRestriction::ForOfInit).node(
                &self.left,
            )?;
            f.keyword(Keyword::Of, &self.token_of);
            f.node(&self.right)?;
        }

        f.node(&self.body)
    }
}


// for await .. of
node!(pub struct ForAwaitStatement {
    pub token_for: KeywordData,
    pub token_await: KeywordData,
    pub token_paren_l: KeywordData,
    pub left: ForOfInit,
    pub token_of: KeywordWrappedData,
    pub right: Box<alias::Expression>,
    pub token_paren_r: KeywordData,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for ForAwaitStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::For, &self.token_for);
        f.keyword(Keyword::Await, &self.token_await);
        {
            let mut f = f.wrap_parens();
            f.restrict_lookahead(LookaheadRestriction::ForOfInit).node(
                &self.left,
            )?;
            f.keyword(Keyword::Of, &self.token_of);
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
    pub token_while: KeywordData,
    pub token_paren_l: KeywordData,
    pub test: Box<alias::Expression>,
    pub token_paren_r: KeywordData,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for WhileStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::While, &self.token_while);
        f.wrap_parens().node(&self.test)?;
        f.node(&self.body)
    }
}


// do ; while(...) ;
node!(pub struct DoWhileStatement {
    pub token_do: KeywordData,
    pub body: Box<alias::Statement>,

    pub token_while: KeywordData,
    pub token_paren_l: KeywordData,
    pub test: Box<alias::Expression>,
    pub token_paren_r: KeywordData,
    pub token_semi: KeywordData,
});
impl NodeDisplay for DoWhileStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Do, &self.token_do);

        f.node(&self.body)?;
        f.keyword(Keyword::While, &self.token_while);
        f.wrap_parens().node(&self.test)?;
        f.punctuator(Punctuator::Semicolon, &self.token_semi);
        Ok(())
    }
}


// switch (...) { ...    }
node!(pub struct SwitchStatement {
    pub token_switch: KeywordData,
    pub token_paren_l: KeywordData,
    pub discriminant: Box<alias::Expression>,
    pub token_paren_r: KeywordData,
    pub token_curly_l: KeywordData,
    pub cases: Vec<SwitchClause>,
    pub token_curly_r: KeywordData,
});
impl NodeDisplay for SwitchStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Switch, &self.token_switch);
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

node_enum!(@node_display pub enum SwitchClause {
    Case(SwitchCase),
    Default(SwitchDefault),
});

// case foo:
// default:
node!(#[derive(Default)] pub struct SwitchCase {
    pub token_case: KeywordData,
    pub test: Box<alias::Expression>,
    pub token_colon: KeywordData,
    pub consequent: Vec<alias::StatementItem>,
});
impl NodeDisplay for SwitchCase {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Case, &self.token_case);
        f.require_precedence(Precedence::Normal).node(&self.test)?;
        f.punctuator(Punctuator::Colon, &self.token_colon);

        f.node_list(&self.consequent)?;

        Ok(())
    }
}

// case foo:
// default:
node!(#[derive(Default)] pub struct SwitchDefault {
    pub token_default: KeywordData,
    pub token_colon: KeywordData,
    pub consequent: Vec<alias::StatementItem>,
});
impl NodeDisplay for SwitchDefault {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Default, &self.token_default);
        f.punctuator(Punctuator::Colon, &self.token_colon);

        f.node_list(&self.consequent)?;

        Ok(())
    }
}


// with(...) ;
node!(pub struct WithStatement {
    pub token_with: KeywordData,
    pub token_paren_l: KeywordData,
    pub object: Box<alias::Expression>,
    pub token_paren_r: KeywordData,
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for WithStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::With, &self.token_with);
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
    pub tokens_prefix: SeparatorTokens,
    pub label: LabelIdentifier,
    pub token_colon: KeywordData,

    // TODO: Annex B technically allows function declarations here, do we care?
    pub body: Box<alias::Statement>,
});
impl NodeDisplay for LabelledStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // f.separators(&self.tokens_prefix);
        f.node(&self.label)?;
        f.punctuator(Punctuator::Colon, &self.token_colon);
        f.node(&self.body)
    }
}


// throw foo;
node!(pub struct ThrowStatement {
    pub token_throw: KeywordData,
    pub argument: Box<alias::Expression>,
});
impl NodeDisplay for ThrowStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Throw, &self.token_throw);
        f.require_precedence(Precedence::Normal).node(&self.argument)?;

        Ok(())
    }
}


// try {} catch(foo) {}
node!(#[derive(Default)] pub struct TryCatchStatement {
    pub token_try: KeywordData,
    pub body: BlockStatement,
    pub catch: CatchClause,
});
impl NodeDisplay for TryCatchStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Try, &self.token_try);
        f.node(&self.body)?;
        f.node(&self.catch)
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
                body: vec![literal::Boolean::from(false).into()].into(),
                catch: CatchClause {
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
    pub token_try: KeywordData,
    pub body: BlockStatement,
    pub catch: CatchClause,
    pub token_finally: KeywordData,
    pub finalizer: BlockStatement,
});
impl NodeDisplay for TryCatchFinallyStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Try, &self.token_try);
        f.node(&self.body)?;

        f.node(&self.catch)?;

        f.keyword(Keyword::Finally, &self.token_finally);
        f.node(&self.finalizer)
    }
}

#[cfg(test)]
mod tests_try_catch_finally {
    use super::*;

    #[test]
    fn it_prints_default() {
        assert_serialize!(TryCatchFinallyStatement::default(), "try{}catch{}finally{}");
    }
}

// try {} finally {}
node!(#[derive(Default)] pub struct TryFinallyStatement {
    pub token_try: KeywordData,
    pub body: BlockStatement,
    pub token_finally: KeywordData,
    pub finalizer: BlockStatement,
});
impl NodeDisplay for TryFinallyStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Try, &self.token_try);
        f.node(&self.body)?;

        f.keyword(Keyword::Finally, &self.token_finally);
        f.node(&self.finalizer)
    }
}


node!(#[derive(Default)] pub struct CatchClause {
    pub token_catch: KeywordData,
    // Missing param is experimental
    pub param: Option<CatchParam>,
    pub body: BlockStatement,
});
impl NodeDisplay for CatchClause {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Catch, &self.token_catch);
        f.node(&self.param)?;
        f.node(&self.body)
    }
}

node!(pub struct CatchParam {
    pub token_paren_l: KeywordData,
    pub argument: BindingPattern,
    pub token_paren_r: KeywordData,
});
impl NodeDisplay for CatchParam {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // TODO paren tokens
        f.wrap_parens().node(&self.argument)?;
        Ok(())
    }
}


// continue;
// continue foo;
node!(#[derive(Default)] pub struct ContinueStatement {
    pub token_continue: KeywordData,
    pub label: Option<LabelValue>,
    pub token_semi: KeywordData,
});
impl NodeDisplay for ContinueStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Continue, &self.token_continue);
        f.node(&self.label)?;
        f.punctuator(Punctuator::Semicolon, &self.token_semi);
        Ok(())
    }
}


// break;
// break foo;
node!(#[derive(Default)] pub struct BreakStatement {
    pub token_break: KeywordData,
    pub label: Option<LabelValue>,
    pub token_semi: KeywordData,
});
impl NodeDisplay for BreakStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Break, &self.token_break);
        f.node(&self.label)?;
        f.punctuator(Punctuator::Semicolon, &self.token_semi);
        Ok(())
    }
}

node!(pub struct LabelValue {
    pub tokens_prefix: SeparatorTokens,
    pub label: LabelIdentifier,
});
impl NodeDisplay for LabelValue {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // f.separators(&self.tokens_prefix);
        f.node(&self.label)?;
        Ok(())
    }
}




// return;
// return foo;
node!(#[derive(Default)] pub struct ReturnStatement {
    pub token_return: KeywordData,
    pub value: Option<ReturnValue>,
    pub token_semi: KeywordData,
});
impl NodeDisplay for ReturnStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Return, &self.token_return);
        f.node(&self.value);
        f.punctuator(Punctuator::Semicolon, &self.token_semi);
        Ok(())
    }
}

node!(pub struct ReturnValue {
    // TODO: No newlines allowed
    pub token_prefix: SeparatorTokens,
    pub expression: Box<alias::Expression>,
});
impl NodeDisplay for ReturnValue {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        // f.separators(&self.token_prefix);
        f.require_precedence(Precedence::Normal).node(&self.expression)?;
        Ok(())
    }
}


// debugger;
node!(#[derive(Default)] pub struct DebuggerStatement {
    pub token_debugger: KeywordData,
    pub token_semi: KeywordData,
});
impl NodeDisplay for DebuggerStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.keyword(Keyword::Debugger, &self.token_debugger);
        f.punctuator(Punctuator::Semicolon, &self.token_semi);
        Ok(())
    }
}

// ;
node!(#[derive(Default)] pub struct EmptyStatement {
    pub token_semi: KeywordData,
});
impl NodeDisplay for EmptyStatement {
    fn fmt(&self, f: &mut NodeFormatter) -> NodeDisplayResult {
        f.punctuator(Punctuator::Semicolon, &self.token_semi);
        Ok(())
    }
}
