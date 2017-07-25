use super::statement;
use super::declaration;
use super::expression;
use super::flow;
use super::jsx;
use super::misc;
use super::literal;

pub enum Function {
    DefaultDeclaration(declaration::ExportDefaultFunctionDeclaration),
    Declaration(declaration::FunctionDeclaration),
    Expression(expression::FunctionExpression),
    ClassMethod(misc::ClassMethod),
    ObjectMethod(expression::ObjectMethod),
    Arrow(expression::ArrowFunctionExpression),
}
pub enum Method {
    ClassMethod(misc::ClassMethod),
    ObjectMethod(expression::ObjectMethod),
}

pub enum ModuleStatementItem {
    Statement(Statement),
    Declaration(Declaration),
    Import(declaration::ImportDeclaration),
    Export(declaration::ExportDeclaration),
}

pub enum StatementItem {
    Statement(Statement),
    Declaration(Declaration),
}

pub enum Declaration {
    Function(declaration::FunctionDeclaration),
    Class(declaration::ClassDeclaration),
    Lexical(declaration::LexicalDeclaration),

    // Flow extension
    FlowTypeDeclareModule(flow::DeclareModuleDeclaration),
    FlowTypeDeclareFunction(flow::DeclareFunctionDeclaration),
    FlowTypeDeclareClass(flow::DeclareClassDeclaration),
    FlowTypeDeclareVariable(flow::DeclareVariableDeclaration),
    FlowTypeDeclareAlias(flow::DeclareAliasDeclaration),
    FlowTypeAlias(flow::AliasDeclaration),
}

pub enum Statement {
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
    Try(statement::TryStatement),
    Debugger(statement::DebuggerStatement),
}
impl misc::HasOrphanIf for ExpressionStatement {
    fn orphan_if(&self) -> bool {
        match self {
            Block(ref node) => node.orphan_if(),
            Variable(ref node) => node.orphan_if(),
            Empty(ref node) => node.orphan_if(),
            Expression(ref node) => node.orphan_if(),
            If(ref node) => node.orphan_if(),
            For(ref node) => node.orphan_if(),
            ForIn(ref node) => node.orphan_if(),
            ForOf(ref node) => node.orphan_if(),
            While(ref node) => node.orphan_if(),
            DoWhile(ref node) => node.orphan_if(),
            Switch(ref node) => node.orphan_if(),
            Continue(ref node) => node.orphan_if(),
            Break(ref node) => node.orphan_if(),
            Return(ref node) => node.orphan_if(),
            With(ref node) => node.orphan_if(),
            Labelled(ref node) => node.orphan_if(),
            Throw(ref node) => node.orphan_if(),
            Try(ref node) => node.orphan_if(),
            Debugger(ref node) => node.orphan_if(),
        }
    }
}

pub enum Expression {
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
    Regex(expression::RegularExpressionLiteral),
    Template(expression::TemplateLiteral),

    Member(expression::MemberExpression),
    SuperMember(expression::SuperMemberExpression),
    Binary(expression::BinaryExpression),
    Unary(expression::UnaryExpression),
    Update(expression::UpdateExpression),

    Call(expression::CallExpression),
    New(expression::NewExpression),

    Conditional(expression::ConditionalExpression),
    Sequence(expression::SequenceExpression),
    Arrow(expression::ArrowFunctionExpression),

    Do(expression::DoExpression),

    FlowTypeCast(flow::CastExpression),

    JSX(jsx::Element),
}
impl misc::FirstSpecialToken for Expression {
    fn first_special_token(&self) -> misc::SpecialToken {
        match self {
            Binding(ref node) => node.first_special_token(),
            This(ref node) => node.first_special_token(),
            Array(ref node) => node.first_special_token(),
            Object(ref node) => node.first_special_token(),
            Null(ref node) => node.first_special_token(),
            Boolean(ref node) => node.first_special_token(),
            Numeric(ref node) => node.first_special_token(),
            String(ref node) => node.first_special_token(),
            Function(ref node) => node.first_special_token(),
            Class(ref node) => node.first_special_token(),
            Regex(ref node) => node.first_special_token(),
            Template(ref node) => node.first_special_token(),
            Member(ref node) => node.first_special_token(),
            SuperMember(ref node) => node.first_special_token(),
            Binary(ref node) => node.first_special_token(),
            Unary(ref node) => node.first_special_token(),
            Update(ref node) => node.first_special_token(),
            Call(ref node) => node.first_special_token(),
            New(ref node) => node.first_special_token(),
            Conditional(ref node) => node.first_special_token(),
            Sequence(ref node) => node.first_special_token(),
            Arrow(ref node) => node.first_special_token(),
            Do(ref node) => node.first_special_token(),
            FlowTypeCast(ref node) => node.first_special_token(),
            JSX(ref node) => node.first_special_token(),
        }
    }
}
impl misc::HasInOperator for Expression {
    fn has_in_operator(&self) -> misc::SpecialToken {
        match self {
            Binding(ref node) => node.has_in_operator(),
            This(ref node) => node.has_in_operator(),
            Array(ref node) => node.has_in_operator(),
            Object(ref node) => node.has_in_operator(),
            Null(ref node) => node.has_in_operator(),
            Boolean(ref node) => node.has_in_operator(),
            Numeric(ref node) => node.has_in_operator(),
            String(ref node) => node.has_in_operator(),
            Function(ref node) => node.has_in_operator(),
            Class(ref node) => node.has_in_operator(),
            Regex(ref node) => node.has_in_operator(),
            Template(ref node) => node.has_in_operator(),
            Member(ref node) => node.has_in_operator(),
            SuperMember(ref node) => node.has_in_operator(),
            Binary(ref node) => node.has_in_operator(),
            Unary(ref node) => node.has_in_operator(),
            Update(ref node) => node.has_in_operator(),
            Call(ref node) => node.has_in_operator(),
            New(ref node) => node.has_in_operator(),
            Conditional(ref node) => node.has_in_operator(),
            Sequence(ref node) => node.has_in_operator(),
            Arrow(ref node) => node.has_in_operator(),
            Do(ref node) => node.has_in_operator(),
            FlowTypeCast(ref node) => node.has_in_operator(),
            JSX(ref node) => node.has_in_operator(),
        }
    }
}
