use ast::statement;
use ast::expression;
use ast::jsx;
use ast::literal;
use ast::modules;
use ast::objects;
use ast::classes;
use ast::functions;
use ast::general;


node_enum!(@node_display pub enum Function {
    // TODO: Should the method types be in here? What is the goal of this node type?
    ClassMethod(classes::ClassMethod),
    ObjectMethod(objects::ObjectMethod),
    DefaultDeclaration(functions::ExportDefaultFunctionDeclaration),
    Declaration(functions::FunctionDeclaration),
    Expression(functions::FunctionExpression),
    Arrow(functions::ArrowFunctionExpression),
});


node_enum!(@node_display pub enum Method {
    ClassMethod(classes::ClassMethod),
    ObjectMethod(objects::ObjectMethod),
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
    Function(functions::FunctionDeclaration),
    Class(classes::ClassDeclaration),
    Let(statement::LetDeclaration),
    Const(statement::ConstDeclaration),

    // ExportDeclaration
    ExportDefaultClass(classes::ExportDefaultClassDeclaration),
    ExportDefaultFunction(functions::ExportDefaultFunctionDeclaration),
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
impl<T: Into<Expression>> From<T> for ModuleStatementItem {
    fn from(v: T) -> ModuleStatementItem {
        ModuleStatementItem::Expression(statement::ExpressionStatement::from(v))
    }
}


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
    Function(functions::FunctionDeclaration),
    Class(classes::ClassDeclaration),
    Let(statement::LetDeclaration),
    Const(statement::ConstDeclaration),
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
impl<T: Into<Expression>> From<T> for StatementItem {
    fn from(v: T) -> StatementItem {
        StatementItem::Expression(statement::ExpressionStatement::from(v))
    }
}


node_enum!(@node_display pub enum Statement {
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
impl Default for Statement {
    fn default() -> Statement {
        statement::EmptyStatement::default().into()
    }
}


node_enum!(@node_display pub enum Expression {
    Binding(general::BindingIdentifier),
    This(expression::ThisExpression),
    Array(objects::ArrayExpression),
    Object(objects::ObjectExpression),
    Null(literal::Null),
    Boolean(literal::Boolean),
    Numeric(literal::Numeric),
    String(literal::String),
    Function(functions::FunctionExpression),
    Class(classes::ClassExpression),
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
    Arrow(functions::ArrowFunctionExpression),
    Do(expression::DoExpression),
    JSX(jsx::Element),
});

// impl<T: Into<general::BindingIdentifier>> From<T> for Expression {
//     fn from(v: T) -> Expression {
//         Expression::Binding(v.into())
//     }
// }
// impl<T: Into<general::BindingIdentifier>> From<T> for Box<Expression> {
//     fn from(v: T) -> Box<Expression> {
//         Expression::Binding(v.into()).into()
//     }
// }
// impl<T: Into<expression::ThisExpression>> From<T> for Expression {
//     fn from(v: T) -> Expression {
//         Expression::This(v.into())
//     }
// }
// impl<T: Into<expression::ThisExpression>> From<T> for Box<Expression> {
//     fn from(v: T) -> Box<Expression> {
//         Expression::This(v.into()).into()
//     }
// }


node_enum!(@node_display pub enum ExportDeclaration {
    DefaultClass(classes::ExportDefaultClassDeclaration),
    DefaultFunction(functions::ExportDefaultFunctionDeclaration),
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
