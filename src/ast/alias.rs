use custom_derive;
use enum_derive;

use super::statement;
use super::declaration;
use super::expression;
use super::flow;
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
        Statement(Statement),
        Declaration(Declaration),
        Import(ImportDeclaration),
        Export(ExportDeclaration),
    }
}
impl display::NodeDisplay for ModuleStatementItem {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            ModuleStatementItem::Statement(ref n) => f.node(n),
            ModuleStatementItem::Declaration(ref n) => f.node(n),
            ModuleStatementItem::Import(ref n) => f.node(n),
            ModuleStatementItem::Export(ref n) => f.node(n),
        }
    }
}

custom_derive!{
    #[derive(EnumFromInner)]
    pub enum StatementItem {
        Statement(Statement),
        Declaration(Declaration),
    }
}
impl display::NodeDisplay for StatementItem {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            StatementItem::Statement(ref n) => f.node(n),
            StatementItem::Declaration(ref n) => f.node(n),
        }
    }
}

custom_derive!{
    #[derive(EnumFromInner)]
    pub enum Declaration {
        Function(declaration::FunctionDeclaration),
        Class(declaration::ClassDeclaration),
        Let(declaration::LetDeclaration),
        Const(declaration::ConstDeclaration),

        // Flow extension
        FlowTypeDeclareModule(flow::DeclareModuleDeclaration),
        FlowTypeDeclareFunction(flow::DeclareFunctionDeclaration),
        FlowTypeDeclareClass(flow::DeclareClassDeclaration),
        FlowTypeDeclareVariable(flow::DeclareVariableDeclaration),
        FlowTypeDeclareAlias(flow::DeclareAliasDeclaration),
        FlowTypeAlias(flow::AliasDeclaration),
    }
}
impl display::NodeDisplay for Declaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            Declaration::Function(ref n) => f.node(n),
            Declaration::Class(ref n) => f.node(n),
            Declaration::Let(ref n) => f.node(n),
            Declaration::Const(ref n) => f.node(n),
            Declaration::FlowTypeDeclareModule(ref n) => f.node(n),
            Declaration::FlowTypeDeclareFunction(ref n) => f.node(n),
            Declaration::FlowTypeDeclareClass(ref n) => f.node(n),
            Declaration::FlowTypeDeclareVariable(ref n) => f.node(n),
            Declaration::FlowTypeDeclareAlias(ref n) => f.node(n),
            Declaration::FlowTypeAlias(ref n) => f.node(n),
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
        Labelled(statement::LabelledStatement), // Technically this is only possible with annexB
        Throw(statement::ThrowStatement),
        Try(statement::TryStatement),
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
            Statement::Try(ref n) => f.node(n),
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
            Statement::Try(ref node) => node.orphan_if(),
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
        Regex(literal::RegularExpressionLiteral),
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

        FlowTypeCast(flow::CastExpression),

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
            Expression::FlowTypeCast(ref n) => f.node(n),
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
            Expression::FlowTypeCast(ref node) => node.first_special_token(),
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
            Expression::FlowTypeCast(ref node) => node.has_in_operator(),
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
        FlowDeclaration(flow::AliasDeclaration),
        SourceSpecifiers(modules::ExportSourceSpecifiers),
        SourceSpecifiersFlow(modules::ExportFlowtypeSourceSpecifiers),
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
            ExportDeclaration::FlowDeclaration(ref n) => f.node(n),
            ExportDeclaration::SourceSpecifiers(ref n) => f.node(n),
            ExportDeclaration::SourceSpecifiersFlow(ref n) => f.node(n),
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
    NamedType(modules::ImportNamedTypeDeclaration),
    NamespaceTypeof(modules::ImportNamespaceTypeofDeclaration),
    NamedAndSpecifiersType(modules::ImportNamedAndSpecifiersTypeDeclaration),
    SpecifiersType(modules::ImportSpecifiersTypeDeclaration),
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
            ImportDeclaration::NamedType(ref n) => f.node(n),
            ImportDeclaration::NamespaceTypeof(ref n) => f.node(n),
            ImportDeclaration::NamedAndSpecifiersType(ref n) => f.node(n),
            ImportDeclaration::SpecifiersType(ref n) => f.node(n),
        }
    }
}
