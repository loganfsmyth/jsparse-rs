#[warn(unused_imports)]
use custom_derive;
#[warn(unused_imports)]
use enum_derive;

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

custom_derive!{
    #[derive(EnumFromInner)]
    pub enum Function {
        DefaultDeclaration(modules::ExportDefaultFunctionDeclaration),
        Declaration(declaration::FunctionDeclaration),
        Expression(expression::FunctionExpression),
        ClassMethod(misc::ClassMethod),
        ObjectMethod(expression::ObjectMethod),
        Arrow(expression::ArrowFunctionExpression),
    }
}
impl display::NodeDisplay for Function {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            Function::DefaultDeclaration(ref n) => f.node(n),
            Function::Declaration(ref n) => f.node(n),
            Function::Expression(ref n) => f.node(n),
            Function::ClassMethod(ref n) => f.node(n),
            Function::ObjectMethod(ref n) => f.node(n),
            Function::Arrow(ref n) => f.node(n),
        }
    }
}

custom_derive!{
    #[derive(EnumFromInner)]
    pub enum Method {
        ClassMethod(misc::ClassMethod),
        ObjectMethod(expression::ObjectMethod),
    }
}
impl display::NodeDisplay for Method {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            Method::ClassMethod(ref n) => f.node(n),
            Method::ObjectMethod(ref n) => f.node(n),
        }
    }
}

custom_derive!{
    #[derive(EnumFromInner)]
    pub enum ModuleStatementItem {
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
    }
}
impl display::NodeDisplay for ModuleStatementItem {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            // Statements
            ModuleStatementItem::Block(ref n) => f.node(n),
            ModuleStatementItem::Variable(ref n) => f.node(n),
            ModuleStatementItem::Empty(ref n) => f.node(n),
            ModuleStatementItem::Expression(ref n) => f.node(n),
            ModuleStatementItem::If(ref n) => f.node(n),
            ModuleStatementItem::For(ref n) => f.node(n),
            ModuleStatementItem::ForIn(ref n) => f.node(n),
            ModuleStatementItem::ForOf(ref n) => f.node(n),
            ModuleStatementItem::ForAwait(ref n) => f.node(n),
            ModuleStatementItem::While(ref n) => f.node(n),
            ModuleStatementItem::DoWhile(ref n) => f.node(n),
            ModuleStatementItem::Switch(ref n) => f.node(n),
            ModuleStatementItem::Continue(ref n) => f.node(n),
            ModuleStatementItem::Break(ref n) => f.node(n),
            ModuleStatementItem::Return(ref n) => f.node(n),
            ModuleStatementItem::With(ref n) => f.node(n),
            ModuleStatementItem::Labelled(ref n) => f.node(n),
            ModuleStatementItem::Throw(ref n) => f.node(n),
            ModuleStatementItem::TryCatch(ref n) => f.node(n),
            ModuleStatementItem::TryCatchFinally(ref n) => f.node(n),
            ModuleStatementItem::TryFinally(ref n) => f.node(n),
            ModuleStatementItem::Debugger(ref n) => f.node(n),

            // Declarations
            ModuleStatementItem::Function(ref n) => f.node(n),
            ModuleStatementItem::Class(ref n) => f.node(n),
            ModuleStatementItem::Let(ref n) => f.node(n),
            ModuleStatementItem::Const(ref n) => f.node(n),

            // ExportDeclaration
            ModuleStatementItem::ExportDefaultClass(ref n) => f.node(n),
            ModuleStatementItem::ExportDefaultFunction(ref n) => f.node(n),
            ModuleStatementItem::ExportDefaultExpression(ref n) => f.node(n),
            ModuleStatementItem::ExportClass(ref n) => f.node(n),
            ModuleStatementItem::ExportFunction(ref n) => f.node(n),
            ModuleStatementItem::ExportVariable(ref n) => f.node(n),
            ModuleStatementItem::ExportLet(ref n) => f.node(n),
            ModuleStatementItem::ExportConst(ref n) => f.node(n),
            ModuleStatementItem::ExportLocalBindings(ref n) => f.node(n),
            ModuleStatementItem::ExportSourceSpecifiers(ref n) => f.node(n),
            ModuleStatementItem::ExportAll(ref n) => f.node(n),
            ModuleStatementItem::ExportNamed(ref n) => f.node(n),
            ModuleStatementItem::ExportNamedAndNamespace(ref n) => f.node(n),
            ModuleStatementItem::ExportNamespace(ref n) => f.node(n),
            ModuleStatementItem::ExportNamedAndSpecifiers(ref n) => f.node(n),

            // ImportDeclaration
            ModuleStatementItem::ImportNamed(ref n) => f.node(n),
            ModuleStatementItem::ImportNamedAndNamespace(ref n) => f.node(n),
            ModuleStatementItem::ImportNamespace(ref n) => f.node(n),
            ModuleStatementItem::ImportNamedAndSpecifiers(ref n) => f.node(n),
            ModuleStatementItem::ImportSpecifiers(ref n) => f.node(n),
        }
    }
}

custom_derive!{
    #[derive(EnumFromInner)]
    pub enum StatementItem {
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
    }
}
impl display::NodeDisplay for StatementItem {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            // Statements
            StatementItem::Block(ref n) => f.node(n),
            StatementItem::Variable(ref n) => f.node(n),
            StatementItem::Empty(ref n) => f.node(n),
            StatementItem::Expression(ref n) => f.node(n),
            StatementItem::If(ref n) => f.node(n),
            StatementItem::For(ref n) => f.node(n),
            StatementItem::ForIn(ref n) => f.node(n),
            StatementItem::ForOf(ref n) => f.node(n),
            StatementItem::ForAwait(ref n) => f.node(n),
            StatementItem::While(ref n) => f.node(n),
            StatementItem::DoWhile(ref n) => f.node(n),
            StatementItem::Switch(ref n) => f.node(n),
            StatementItem::Continue(ref n) => f.node(n),
            StatementItem::Break(ref n) => f.node(n),
            StatementItem::Return(ref n) => f.node(n),
            StatementItem::With(ref n) => f.node(n),
            StatementItem::Labelled(ref n) => f.node(n),
            StatementItem::Throw(ref n) => f.node(n),
            StatementItem::TryCatch(ref n) => f.node(n),
            StatementItem::TryCatchFinally(ref n) => f.node(n),
            StatementItem::TryFinally(ref n) => f.node(n),
            StatementItem::Debugger(ref n) => f.node(n),

            // Declarations
            StatementItem::Function(ref n) => f.node(n),
            StatementItem::Class(ref n) => f.node(n),
            StatementItem::Let(ref n) => f.node(n),
            StatementItem::Const(ref n) => f.node(n),
        }
    }
}
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

custom_derive!{
    #[derive(EnumFromInner)]
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
        TryCatch(statement::TryCatchStatement),
        TryCatchFinally(statement::TryCatchFinallyStatement),
        TryFinally(statement::TryFinallyStatement),
        Debugger(statement::DebuggerStatement),
    }
}
impl display::NodeDisplay for Statement {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            Statement::Block(ref n) => f.node(n),
            Statement::Variable(ref n) => f.node(n),
            Statement::Empty(ref n) => f.node(n),
            Statement::Expression(ref n) => f.node(n),
            Statement::If(ref n) => f.node(n),
            Statement::For(ref n) => f.node(n),
            Statement::ForIn(ref n) => f.node(n),
            Statement::ForOf(ref n) => f.node(n),
            Statement::ForAwait(ref n) => f.node(n),
            Statement::While(ref n) => f.node(n),
            Statement::DoWhile(ref n) => f.node(n),
            Statement::Switch(ref n) => f.node(n),
            Statement::Continue(ref n) => f.node(n),
            Statement::Break(ref n) => f.node(n),
            Statement::Return(ref n) => f.node(n),
            Statement::With(ref n) => f.node(n),
            Statement::Labelled(ref n) => f.node(n),
            Statement::Throw(ref n) => f.node(n),
            Statement::TryCatch(ref n) => f.node(n),
            Statement::TryCatchFinally(ref n) => f.node(n),
            Statement::TryFinally(ref n) => f.node(n),
            Statement::Debugger(ref n) => f.node(n),
        }
    }
}
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

custom_derive!{
    #[derive(EnumFromInner)]
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
        Regex(literal::RegExp),
        Template(expression::TemplateLiteral),
        Member(expression::MemberExpression),
        SuperMember(expression::SuperMemberExpression),
        Binary(expression::BinaryExpression),
        Unary(expression::UnaryExpression),
        Update(expression::UpdateExpression),
        Call(expression::CallExpression),
        New(expression::NewExpression),
        Conditional(expression::ConditionalExpression),
        Assign(expression::AssignmentExpression),
        AssignUpdate(expression::AssignmentUpdateExpression),
        Sequence(expression::SequenceExpression),
        Arrow(expression::ArrowFunctionExpression),
        Do(expression::DoExpression),
        JSX(jsx::Element),
    }
}
impl display::NodeDisplay for Expression {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            Expression::Binding(ref n) => f.node(n),
            Expression::This(ref n) => f.node(n),
            Expression::Array(ref n) => f.node(n),
            Expression::Object(ref n) => f.node(n),
            Expression::Null(ref n) => f.node(n),
            Expression::Boolean(ref n) => f.node(n),
            Expression::Numeric(ref n) => f.node(n),
            Expression::String(ref n) => f.node(n),
            Expression::Function(ref n) => f.node(n),
            Expression::Class(ref n) => f.node(n),
            Expression::Regex(ref n) => f.node(n),
            Expression::Template(ref n) => f.node(n),
            Expression::Member(ref n) => f.node(n),
            Expression::SuperMember(ref n) => f.node(n),
            Expression::Binary(ref n) => f.node(n),
            Expression::Unary(ref n) => f.node(n),
            Expression::Update(ref n) => f.node(n),
            Expression::Call(ref n) => f.node(n),
            Expression::New(ref n) => f.node(n),
            Expression::Conditional(ref n) => f.node(n),
            Expression::Assign(ref n) => f.node(n),
            Expression::AssignUpdate(ref n) => f.node(n),
            Expression::Sequence(ref n) => f.node(n),
            Expression::Arrow(ref n) => f.node(n),
            Expression::Do(ref n) => f.node(n),
            Expression::JSX(ref n) => f.node(n),
        }
    }
}
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

custom_derive!{
    #[derive(EnumFromInner)]
    pub enum ExportDeclaration {
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
    }
}
impl display::NodeDisplay for ExportDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ExportDeclaration::DefaultClass(ref n) => f.node(n),
            ExportDeclaration::DefaultFunction(ref n) => f.node(n),
            ExportDeclaration::DefaultExpression(ref n) => f.node(n),
            ExportDeclaration::Class(ref n) => f.node(n),
            ExportDeclaration::Function(ref n) => f.node(n),
            ExportDeclaration::Variable(ref n) => f.node(n),
            ExportDeclaration::Let(ref n) => f.node(n),
            ExportDeclaration::Const(ref n) => f.node(n),
            ExportDeclaration::LocalBindings(ref n) => f.node(n),
            ExportDeclaration::SourceSpecifiers(ref n) => f.node(n),
            ExportDeclaration::All(ref n) => f.node(n),
            ExportDeclaration::Named(ref n) => f.node(n),
            ExportDeclaration::NamedAndNamespace(ref n) => f.node(n),
            ExportDeclaration::Namespace(ref n) => f.node(n),
            ExportDeclaration::NamedAndSpecifiers(ref n) => f.node(n),
        }
    }
}


custom_derive!{
    #[derive(EnumFromInner)]
    pub enum ImportDeclaration {
        Named(modules::ImportNamedDeclaration),
        NamedAndNamespace(modules::ImportNamedAndNamespaceDeclaration),
        Namespace(modules::ImportNamespaceDeclaration),
        NamedAndSpecifiers(modules::ImportNamedAndSpecifiersDeclaration),
        Specifiers(modules::ImportSpecifiersDeclaration),
    }
}
impl display::NodeDisplay for ImportDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ImportDeclaration::Named(ref n) => f.node(n),
            ImportDeclaration::NamedAndNamespace(ref n) => f.node(n),
            ImportDeclaration::Namespace(ref n) => f.node(n),
            ImportDeclaration::NamedAndSpecifiers(ref n) => f.node(n),
            ImportDeclaration::Specifiers(ref n) => f.node(n),
        }
    }
}
