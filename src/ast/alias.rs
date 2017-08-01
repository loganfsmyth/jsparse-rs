use super::statement;
use super::declaration;
use super::expression;
use super::jsx;
use super::misc;
use super::literal;
use super::display;
use super::modules;

use super::misc::HasInOperator;
use super::misc::FirstSpecialToken;


node_enum!(pub enum Function {
    DefaultDeclaration(modules::ExportDefaultFunctionDeclaration),
    Declaration(declaration::FunctionDeclaration),
    Expression(expression::FunctionExpression),
    ClassMethod(misc::ClassMethod),
    ObjectMethod(expression::ObjectMethod),
    Arrow(expression::ArrowFunctionExpression),
});


node_enum!(pub enum Method {
    ClassMethod(misc::ClassMethod),
    ObjectMethod(expression::ObjectMethod),
});


node_enum!(pub enum ModuleStatementItem {
    // Statements
    Block(statement::BlockStatement),
    Variable(statement::VariableStatement),
    Empty(statement::EmptyStatement),
    Expression(statement::ExpressionStatement),
    If(statement::IfStatement),
    For(statement::ForStatement),
    ForIn(statement::ForInStatement),
    ForOf(statement::ForOfStatement),
    ForAwait(statement::ForAwaitStatement),
    While(statement::WhileStatement),
    DoWhile(statement::DoWhileStatement),
    Switch(statement::SwitchStatement),
    Continue(statement::ContinueStatement),
    Break(statement::BreakStatement),
    Return(statement::ReturnStatement),
    With(statement::WithStatement),
    Labelled(statement::LabelledStatement),
    Throw(statement::ThrowStatement),
    TryCatch(statement::TryCatchStatement),
    TryCatchFinally(statement::TryCatchFinallyStatement),
    TryFinally(statement::TryFinallyStatement),
    Debugger(statement::DebuggerStatement),

    // Declarations
    Function(declaration::FunctionDeclaration),
    Class(declaration::ClassDeclaration),
    Let(declaration::LetDeclaration),
    Const(declaration::ConstDeclaration),

    // ExportDeclaration
    ExportDefaultClass(modules::ExportDefaultClassDeclaration),
    ExportDefaultFunction(modules::ExportDefaultFunctionDeclaration),
    ExportDefaultExpression(modules::ExportDefaultExpression),
    ExportClass(modules::ExportClassDeclaration),
    ExportFunction(modules::ExportFunctionDeclaration),
    ExportVariable(modules::ExportVarStatement),
    ExportLet(modules::ExportLetDeclaration),
    ExportConst(modules::ExportConstDeclaration),
    ExportLocalBindings(modules::ExportLocalBindings),
    ExportSourceSpecifiers(modules::ExportSourceSpecifiers),
    ExportAll(modules::ExportAllSpecifiers),
    ExportNamed(modules::ExportNamedSpecifier),
    ExportNamedAndNamespace(modules::ExportNamedAndNamespace),
    ExportNamespace(modules::ExportNamespace),
    ExportNamedAndSpecifiers(modules::ExportNamedAndSpecifiers),

    // ImportDeclaration
    ImportNamed(modules::ImportNamedDeclaration),
    ImportNamedAndNamespace(modules::ImportNamedAndNamespaceDeclaration),
    ImportNamespace(modules::ImportNamespaceDeclaration),
    ImportNamedAndSpecifiers(modules::ImportNamedAndSpecifiersDeclaration),
    ImportSpecifiers(modules::ImportSpecifiersDeclaration),
});


node_enum!(pub enum StatementItem {
    // Statements
    Block(statement::BlockStatement),
    Variable(statement::VariableStatement),
    Empty(statement::EmptyStatement),
    Expression(statement::ExpressionStatement),
    If(statement::IfStatement),
    For(statement::ForStatement),
    ForIn(statement::ForInStatement),
    ForOf(statement::ForOfStatement),
    ForAwait(statement::ForAwaitStatement),
    While(statement::WhileStatement),
    DoWhile(statement::DoWhileStatement),
    Switch(statement::SwitchStatement),
    Continue(statement::ContinueStatement),
    Break(statement::BreakStatement),
    Return(statement::ReturnStatement),
    With(statement::WithStatement),
    Labelled(statement::LabelledStatement),
    Throw(statement::ThrowStatement),
    TryCatch(statement::TryCatchStatement),
    TryCatchFinally(statement::TryCatchFinallyStatement),
    TryFinally(statement::TryFinallyStatement),
    Debugger(statement::DebuggerStatement),

    // Declarations
    Function(declaration::FunctionDeclaration),
    Class(declaration::ClassDeclaration),
    Let(declaration::LetDeclaration),
    Const(declaration::ConstDeclaration),
});
impl From<Statement> for StatementItem {
    fn from(stmt: Statement) -> StatementItem {
        match stmt {
            Statement::Block(n) => n.into(),
            Statement::Variable(n) => n.into(),
            Statement::Empty(n) => n.into(),
            Statement::Expression(n) => n.into(),
            Statement::If(n) => n.into(),
            Statement::For(n) => n.into(),
            Statement::ForIn(n) => n.into(),
            Statement::ForOf(n) => n.into(),
            Statement::ForAwait(n) => n.into(),
            Statement::While(n) => n.into(),
            Statement::DoWhile(n) => n.into(),
            Statement::Switch(n) => n.into(),
            Statement::Continue(n) => n.into(),
            Statement::Break(n) => n.into(),
            Statement::Return(n) => n.into(),
            Statement::With(n) => n.into(),
            Statement::Labelled(n) => n.into(),
            Statement::Throw(n) => n.into(),
            Statement::TryCatch(n) => n.into(),
            Statement::TryCatchFinally(n) => n.into(),
            Statement::TryFinally(n) => n.into(),
            Statement::Debugger(n) => n.into(),
        }
    }
}


node_enum!(pub enum Statement {
    Block(statement::BlockStatement),
    Variable(statement::VariableStatement),
    Empty(statement::EmptyStatement),
    Expression(statement::ExpressionStatement),
    If(statement::IfStatement),
    For(statement::ForStatement),
    ForIn(statement::ForInStatement),
    ForOf(statement::ForOfStatement),
    ForAwait(statement::ForAwaitStatement),
    While(statement::WhileStatement),
    DoWhile(statement::DoWhileStatement),
    Switch(statement::SwitchStatement),
    Continue(statement::ContinueStatement),
    Break(statement::BreakStatement),
    Return(statement::ReturnStatement),
    With(statement::WithStatement),
    Labelled(statement::LabelledStatement),
    Throw(statement::ThrowStatement),
    TryCatch(statement::TryCatchStatement),
    TryCatchFinally(statement::TryCatchFinallyStatement),
    TryFinally(statement::TryFinallyStatement),
    Debugger(statement::DebuggerStatement),
});
impl misc::HasOrphanIf for Statement {
    fn orphan_if(&self) -> bool {
        match *self {
            Statement::Block(ref node) => node.orphan_if(),
            Statement::Variable(ref node) => node.orphan_if(),
            Statement::Empty(ref node) => node.orphan_if(),
            Statement::Expression(ref node) => node.orphan_if(),
            Statement::If(ref node) => node.orphan_if(),
            Statement::For(ref node) => node.orphan_if(),
            Statement::ForIn(ref node) => node.orphan_if(),
            Statement::ForOf(ref node) => node.orphan_if(),
            Statement::ForAwait(ref node) => node.orphan_if(),
            Statement::While(ref node) => node.orphan_if(),
            Statement::DoWhile(ref node) => node.orphan_if(),
            Statement::Switch(ref node) => node.orphan_if(),
            Statement::Continue(ref node) => node.orphan_if(),
            Statement::Break(ref node) => node.orphan_if(),
            Statement::Return(ref node) => node.orphan_if(),
            Statement::With(ref node) => node.orphan_if(),
            Statement::Labelled(ref node) => node.orphan_if(),
            Statement::Throw(ref node) => node.orphan_if(),
            Statement::TryCatch(ref node) => node.orphan_if(),
            Statement::TryCatchFinally(ref node) => node.orphan_if(),
            Statement::TryFinally(ref node) => node.orphan_if(),
            Statement::Debugger(ref node) => node.orphan_if(),
        }
    }
}


node_enum!(pub enum Expression {
    Binding(misc::BindingIdentifier),
    This(expression::ThisExpression),
    Array(expression::ArrayExpression),
    Object(expression::ObjectExpression),
    Null(literal::Null),
    Boolean(literal::Boolean),
    Numeric(literal::Numeric),
    String(literal::String),
    Function(expression::FunctionExpression),
    Class(expression::ClassExpression),
    Regex(literal::RegExp),
    Template(expression::TemplateLiteral),
    Member(expression::MemberExpression),
    SuperMember(expression::SuperMemberExpression),
    Binary(expression::BinaryExpression),
    Unary(expression::UnaryExpression),
    Update(expression::UpdateExpression),
    Call(expression::CallExpression),
    New(expression::NewExpression),
    ImportCall(expression::ImportCallExpression),
    SuperCall(expression::SuperCallExpression),
    Conditional(expression::ConditionalExpression),
    Assign(expression::AssignmentExpression),
    AssignUpdate(expression::AssignmentUpdateExpression),
    Sequence(expression::SequenceExpression),
    Arrow(expression::ArrowFunctionExpression),
    Do(expression::DoExpression),
    JSX(jsx::Element),
});
impl misc::FirstSpecialToken for Expression {
    fn first_special_token(&self) -> misc::SpecialToken {
        match *self {
            Expression::Binding(ref node) => node.first_special_token(),
            Expression::This(ref node) => node.first_special_token(),
            Expression::Array(ref node) => node.first_special_token(),
            Expression::Object(ref node) => node.first_special_token(),
            Expression::Null(ref node) => node.first_special_token(),
            Expression::Boolean(ref node) => node.first_special_token(),
            Expression::Numeric(ref node) => node.first_special_token(),
            Expression::String(ref node) => node.first_special_token(),
            Expression::Function(ref node) => node.first_special_token(),
            Expression::Class(ref node) => node.first_special_token(),
            Expression::Regex(ref node) => node.first_special_token(),
            Expression::Template(ref node) => node.first_special_token(),
            Expression::Member(ref node) => node.first_special_token(),
            Expression::SuperMember(ref node) => node.first_special_token(),
            Expression::Binary(ref node) => node.first_special_token(),
            Expression::Unary(ref node) => node.first_special_token(),
            Expression::Update(ref node) => node.first_special_token(),
            Expression::Call(ref node) => node.first_special_token(),
            Expression::New(ref node) => node.first_special_token(),
            Expression::ImportCall(ref node) => node.first_special_token(),
            Expression::SuperCall(ref node) => node.first_special_token(),
            Expression::Conditional(ref node) => node.first_special_token(),
            Expression::Assign(ref node) => node.first_special_token(),
            Expression::AssignUpdate(ref node) => node.first_special_token(),
            Expression::Sequence(ref node) => node.first_special_token(),
            Expression::Arrow(ref node) => node.first_special_token(),
            Expression::Do(ref node) => node.first_special_token(),
            Expression::JSX(ref node) => node.first_special_token(),
        }
    }
}
impl misc::HasInOperator for Expression {
    fn has_in_operator(&self) -> bool {
        match *self {
            Expression::Binding(ref node) => node.has_in_operator(),
            Expression::This(ref node) => node.has_in_operator(),
            Expression::Array(ref node) => node.has_in_operator(),
            Expression::Object(ref node) => node.has_in_operator(),
            Expression::Null(ref node) => node.has_in_operator(),
            Expression::Boolean(ref node) => node.has_in_operator(),
            Expression::Numeric(ref node) => node.has_in_operator(),
            Expression::String(ref node) => node.has_in_operator(),
            Expression::Function(ref node) => node.has_in_operator(),
            Expression::Class(ref node) => node.has_in_operator(),
            Expression::Regex(ref node) => node.has_in_operator(),
            Expression::Template(ref node) => node.has_in_operator(),
            Expression::Member(ref node) => node.has_in_operator(),
            Expression::SuperMember(ref node) => node.has_in_operator(),
            Expression::Binary(ref node) => node.has_in_operator(),
            Expression::Unary(ref node) => node.has_in_operator(),
            Expression::Update(ref node) => node.has_in_operator(),
            Expression::Call(ref node) => node.has_in_operator(),
            Expression::New(ref node) => node.has_in_operator(),
            Expression::ImportCall(ref node) => node.has_in_operator(),
            Expression::SuperCall(ref node) => node.has_in_operator(),
            Expression::Conditional(ref node) => node.has_in_operator(),
            Expression::Assign(ref node) => node.has_in_operator(),
            Expression::AssignUpdate(ref node) => node.has_in_operator(),
            Expression::Sequence(ref node) => node.has_in_operator(),
            Expression::Arrow(ref node) => node.has_in_operator(),
            Expression::Do(ref node) => node.has_in_operator(),
            Expression::JSX(ref node) => node.has_in_operator(),
        }
    }
}


node_enum!(pub enum ExportDeclaration {
    DefaultClass(modules::ExportDefaultClassDeclaration),
    DefaultFunction(modules::ExportDefaultFunctionDeclaration),
    DefaultExpression(modules::ExportDefaultExpression),
    Class(modules::ExportClassDeclaration),
    Function(modules::ExportFunctionDeclaration),
    Variable(modules::ExportVarStatement),
    Let(modules::ExportLetDeclaration),
    Const(modules::ExportConstDeclaration),
    LocalBindings(modules::ExportLocalBindings),
    SourceSpecifiers(modules::ExportSourceSpecifiers),
    All(modules::ExportAllSpecifiers),

    // experimental
    Named(modules::ExportNamedSpecifier),
    NamedAndNamespace(modules::ExportNamedAndNamespace),
    Namespace(modules::ExportNamespace),
    NamedAndSpecifiers(modules::ExportNamedAndSpecifiers),
});


node_enum!(pub enum ImportDeclaration {
    Named(modules::ImportNamedDeclaration),
    NamedAndNamespace(modules::ImportNamedAndNamespaceDeclaration),
    Namespace(modules::ImportNamespaceDeclaration),
    NamedAndSpecifiers(modules::ImportNamedAndSpecifiersDeclaration),
    Specifiers(modules::ImportSpecifiersDeclaration),
});
