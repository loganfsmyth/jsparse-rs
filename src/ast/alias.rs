use super::statement;
use super::declaration;
use super::expression;
use super::jsx;
use super::misc;
use super::literal;
use super::display;
use super::modules;

use super::misc::FirstSpecialToken;


node_enum!(@node_display pub enum Function {
    DefaultDeclaration(modules::ExportDefaultFunctionDeclaration),
    Declaration(declaration::FunctionDeclaration),
    Expression(expression::FunctionExpression),
    ClassMethod(misc::ClassMethod),
    ObjectMethod(expression::ObjectMethod),
    Arrow(expression::ArrowFunctionExpression),
});


node_enum!(@node_display pub enum Method {
    ClassMethod(misc::ClassMethod),
    ObjectMethod(expression::ObjectMethod),
});


node_enum!(@node_display pub enum ModuleStatementItem {
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


node_enum!(@node_display pub enum StatementItem {
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


node_enum!(@node_display @orphan_if pub enum Statement {
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


node_enum!(@node_display @has_in_operator @first_special_token pub enum Expression {
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


node_enum!(@node_display pub enum ExportDeclaration {
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


node_enum!(@node_display pub enum ImportDeclaration {
    Named(modules::ImportNamedDeclaration),
    NamedAndNamespace(modules::ImportNamedAndNamespaceDeclaration),
    Namespace(modules::ImportNamespaceDeclaration),
    NamedAndSpecifiers(modules::ImportNamedAndSpecifiersDeclaration),
    Specifiers(modules::ImportSpecifiersDeclaration),
});
