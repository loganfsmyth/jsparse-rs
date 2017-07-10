type MaybePosition = Option<Box<NodePosition>>;

struct NodePosition {
	start: usize,
	end: usize,
	range: PositionRange,
}
struct PositionRange {
	start: (usize, usize),
	end: (usize, usize),
}



struct Script {
	body: Vec<StatementItem>,
	position: MaybePosition,
}

struct Module {
	body: Vec<ModuleStatementItem>,
	position: MaybePosition,
}

enum StatementItem {
	Statement(Statement),
	Declaration(Declaration),
}

enum ModuleStatementItem {
	Statement(Statement),
	ModuleDeclaration(Declaration),
}

enum Statement {
	Block(BlockStatement),
	Variable(VariableStatement),
	Empty(EmptyStatement),
	Expression(ExpressionStatement),
	If(IfStatement),
	For(ForStatement),
	ForIn(ForInStatement),
	ForOf(ForOfStatement),
	While(WhileStatement),
	DoWhile(DoWhileStatement),
	Switch(SwitchStatement),
	Continue(ContinueStatement),
	Break(BreakStatement),
	Return(ReturnStatement),
	With(WithStatement),
	Labelled(LabelledStatement),
	Throw(ThrowStatement),
	Try(TryStatement),
	Debugger(DebuggerStatement),
}

impl From<BlockStatement> for Statement {
	fn from(stmt: BlockStatement) -> Self {
		Statement::Block(stmt)
	}
}

enum Declaration {
	Function(FunctionDeclaration),
	Class(ClassDeclaration),
	Lexical(LexicalDeclaration),
}
enum ModuleDeclaration {
	Function(FunctionDeclaration),
	Class(ClassDeclaration),
	Lexical(LexicalDeclaration),
	Import(ImportDeclaration),
	Export(ExportDeclaration),
}

struct BlockStatement {
	body: Vec<StatementItem>,
	position: MaybePosition,
}

struct VariableStatement {
	declarations: Vec<VariableDeclarator>,
	position: MaybePosition,
}
struct VariableDeclarator {
	id: Pattern,
	init: Option<Expression>,
	position: MaybePosition,
}

struct LexicalDeclaration {
	kind: LexicalKind,
	declarations: Vec<LexicalDeclarator>,
	position: MaybePosition,
}
struct LexicalDeclarator {
	id: Pattern,
	init: Option<Expression>,
	position: MaybePosition,
}
enum LexicalKind {
	Let,
	Const,
}


enum Pattern {
	Identifier(Identifier),
	Object(ObjectPattern),
	Array(ArrayPattern),
}

struct Identifier {
	id: String,
	position: MaybePosition,
}
struct PropertyIdentifier {
	id: String,
	position: MaybePosition,
}

struct ObjectPattern {
	properties: Vec<ObjectPatternProperty>,
	rest: Option<Box<Pattern>>,
	position: MaybePosition,
}
struct ObjectPatternProperty {
	// foo (= expr)?
	// prop: foo (= expr)?
	// prop: {a} (= expr)?
	name: Option<Identifier>,
	id: Pattern,
	init: Option<Expression>,
	position: MaybePosition,
}

struct ArrayPattern {
	elements: Vec<Option<ArrayPatternElement>>,
	rest: Option<Box<Pattern>>,
	position: MaybePosition,
}
struct ArrayPatternElement {
	// foo (= expr)?
	// {a} (= expr)?
	id: Pattern,
	init: Option<Expression>,
	position: MaybePosition,
}

struct ExpressionStatement {
	expression: Expression,
	position: MaybePosition,
}

struct IfStatement {
	test: Expression,
	consequence: Box<Statement>,
	alternate: Option<Box<Statement>>,
	position: MaybePosition,
}

enum ForInit {
	Variable(VariableDeclarator),
	Lexical(LexicalDeclarator),
	Pattern(Pattern),
	Expression(Expression),
}

struct ForStatement {
	init: Option<ForInit>,
	test: Option<Box<Expression>>,
	update: Option<Box<Expression>>,
	body: Box<Statement>,
	position: MaybePosition,
}

enum ForXInit {
	// These aren't allowed to have inits
	Variable(VariableDeclarator),
	Lexical(LexicalDeclarator),
	Pattern(Pattern),
	Expression(Box<Expression>), // May result in runtime errors, even if it parses
}

struct ForInStatement {
	left: ForXInit,
	right: Box<Expression>,
	body: Box<Statement>,
	position: MaybePosition,
}

struct ForOfStatement {
	kind: ForOfKind,
	left: ForXInit,
	right: Box<Expression>,
	body: Box<Statement>,
	position: MaybePosition,
}

enum ForOfKind {
	Normal,
	Async,
}

struct WhileStatement {
	test: Box<Expression>,
	body: Box<Statement>,
	position: MaybePosition,
}

struct DoWhileStatement {
	test: Box<Expression>,
	body: Box<Statement>,
	position: MaybePosition,
}
struct SwitchStatement {
	discriminant: Box<Expression>,
	cases: Vec<SwitchCase>,
	position: MaybePosition,
}

struct SwitchCase {
	test: Option<Box<Expression>>,
	consequent: Vec<StatementItem>,
	position: MaybePosition,
}

struct ContinueStatement {
	label: Option<Identifier>,
	position: MaybePosition,
}

struct BreakStatement {
	label: Option<Identifier>,
	position: MaybePosition,
}

struct ReturnStatement {
	argument: Option<Box<Expression>>,
	position: MaybePosition,
}

struct WithStatement {
	object: Box<Expression>,
	body: Box<Statement>,
	position: MaybePosition,
}

struct LabelledStatement {
	label: Identifier,
	body: Box<Statement>,
	position: MaybePosition,
}

struct ThrowStatement {
	argument: Box<Expression>,
	position: MaybePosition,
}

struct TryStatement {
	block: BlockStatement,
	handler: Option<CatchClause>,
	finalizer: Option<BlockStatement>,
	position: MaybePosition,
}
struct CatchClause {
	param: Pattern,
	body: BlockStatement,
	position: MaybePosition,
}

struct DebuggerStatement {
	position: MaybePosition,
}
struct EmptyStatement {
	position: MaybePosition,
}


struct DefaultFunctionDeclaration {
	id: Option<Identifier>,
	params: FunctionParams,
	body: BlockStatement,
	fn_kind: FunctionKind,
	position: MaybePosition,
}

struct FunctionDeclaration {
	id: Identifier,
	params: FunctionParams,
	body: BlockStatement,
	fn_kind: FunctionKind,
	position: MaybePosition,
}
struct FunctionParams {
	params: Vec<FunctionParam>,
	rest: Option<Pattern>,
	position: MaybePosition,
}
struct FunctionParam {
	decorators: Vec<Decorator>,
	id: Pattern,
	init: Option<Box<Expression>>,
}

struct DefaultClassDeclaration {
	decorators: Vec<Decorator>,
	id: Option<Identifier>,
	extends: Option<Box<Expression>>,
	body: ClassBody,
	position: MaybePosition,
}
struct ClassDeclaration {
	decorators: Vec<Decorator>,
	id: Identifier,
	extends: Option<Box<Expression>>,
	body: ClassBody,
	position: MaybePosition,
}
struct ClassBody {
	items: Vec<ClassItem>,
	position: MaybePosition,
}
enum ClassItem {
	Method(ClassMethod),
	Field(ClassField),
	Empty,
}

enum FunctionKind {
	Normal,
	Generator,
	Async,

	AsyncGenerator, // experimental
}

enum MethodKind {
	Normal,
	Get,
	Set,
}
enum PropertyId {
	Literal(PropertyIdentifier),
	String(StringLiteral),
	Number(NumericLiteral),
	Computed(Box<Expression>),
}

struct ClassMethod {
	pos: FieldPosition,
	kind: MethodKind,
	id: ClassFieldId,
	params: FunctionParams,
	body: BlockStatement,
	fn_kind: FunctionKind,
	decorators: Vec<Decorator>,
	position: MaybePosition,
}
enum FieldPosition {
    Instance,
    Static,
}

// experimental
enum ClassFieldId {
	Public(PropertyId),
	Private(PropertyIdentifier),
}

// experimental
struct ClassField {
	pos: FieldPosition,
	decorators: Vec<Decorator>,
	items: Vec<ClassFieldPair>,
	position: MaybePosition,
}
struct ClassFieldPair {
	id: ClassFieldId,
	value: Expression,
}

// experimental
enum Decorator {
	Property(DecoratorMemberExpression),
	Call(DecoratorCallExpression),

	Expression(Expression), // Backward-compat for older decorator spec
}

// experimental
enum DecoratorMemberExpression {
	Identifier(Identifier),
	Member(Box<DecoratorMemberExpression>, PropertyIdentifier),
}
// experimental
struct DecoratorCallExpression {
	callee: DecoratorMemberExpression,
	arguments: CallArguments,
	position: MaybePosition,
}

enum MetaProperty {
	NewTarget,
	ImportMeta, // experimental
	FunctionSent, // experimental
	FunctionArguments, // experimental
}
struct SuperMemberExpression {
	property: PropertyId,
	position: MaybePosition,
}

// import ... from "";
struct ImportDeclaration {
	specifiers: ImportSpecifiers,
	source: StringLiteral,
	position: MaybePosition,
}
enum ImportSpecifiers {
	// foo
	Named(Identifier),
	// foo, * as bar
	NamedAndNamespace(Identifier, Identifier),
	// * as bar
	Namespace(Identifier),
	// foo, {bar}
	// foo, {bar as bar}
	NamedAndSpecifiers(Identifier, Vec<ImportSpecifier>),
	// {bar}
	// {bar as bar}
	Specifiers(Vec<ImportSpecifier>),
}
enum ImportSpecifier {
	Named(Identifier),
	NamedAndAliased(ModuleIdentifier, Identifier),
}
struct ModuleIdentifier {
	// Identifier with "default"
	id: String,
	position: MaybePosition,
}

struct ExportDeclaration {
	decl_type: ExportType,
	position: MaybePosition,
}
enum ExportType {
	// export default class {}
	DefaultClass(DefaultClassDeclaration),
	// export default function() {}
	DefaultFunction(DefaultFunctionDeclaration),

	// export class foo {}
	Class(ClassDeclaration),
	// export function foo() {}
	Function(FunctionDeclaration),
	// export var foo;
	Variable(VariableStatement),
	Lexical(LexicalDeclaration),

	// export {foo}
	// export {foo as bar}
	LocalSpecifiers(Vec<LocalExportSpecifier>),

	// export {foo} from "";
	// export {foo as bar} from "";
	SourceSpecifiers(Vec<SourceExportSpecifier>, StringLiteral),

	// export * from "";
	All(StringLiteral),

	// export foo from "";
	Named(ModuleIdentifier, StringLiteral), // experimental
	// export foo, * as foo from "";
	NamedAndNamespace(ModuleIdentifier, ModuleIdentifier, StringLiteral), // experimental
	// export * as foo from "";
	Namespace(ModuleIdentifier, StringLiteral), // experimental
	// export foo, {foo} from "";
	// export foo, {foo as bar} from "";
	NamedAndSpecifiers(ModuleIdentifier, Vec<SourceExportSpecifier>, StringLiteral), // experimental
}
enum LocalExportSpecifier {
	Named(Identifier),
	NamedAndAliased(Identifier, ModuleIdentifier),
}
enum SourceExportSpecifier {
	Named(Identifier),
	NamedAndAliased(Identifier, ModuleIdentifier),
}


enum Expression {
	Identifier(Identifier),
	This(ThisExpression),
	Array(ArrayExpression),
	Object(ObjectExpression),
	Null(NullLiteral),
	Boolean(BooleanLiteral),
	Numeric(NumericLiteral),
	String(StringLiteral),

	Function(FunctionExpression),
	Class(ClassExpression),
	Regex(RegularExpressionLiteral),
	Template(TemplateLiteral),

	Member(MemberExpression),
	SuperMember(SuperMemberExpression),
	Binary(BinaryExpression),
	Unary(UnaryExpression),
	Update(UpdateExpression),

	Call(CallExpression),
	New(NewExpression),

	Conditional(ConditionalExpression),
	Sequence(SequenceExpression),
	Arrow(ArrowFunctionExpression),

	Do(DoExpression),
}

struct NullLiteral {}
struct BooleanLiteral {
	value: bool,
	position: MaybePosition,
}
struct NumericLiteral {
	value: f64,
	position: MaybePosition,
}
struct StringLiteral {
	value: String,
	position: MaybePosition,
}

struct ThisExpression {
	position: MaybePosition,
}

struct ArrayExpression {
	elements: Vec<Option<Box<Expression>>>,
	spread: Option<Box<Expression>>,
	position: MaybePosition,
}
struct ObjectExpression {
	properties: Vec<ObjectProperty>,
	spread: Option<Box<Expression>>,
	position: MaybePosition,
}

enum ObjectProperty {
	Method(ObjectMethod),
	Value(PropertyId, Box<Expression>),
}

struct ObjectMethod {
	kind: MethodKind,
	id: PropertyId,
	params: FunctionParams,
	body: BlockStatement,
	fn_kind: FunctionKind,
	position: MaybePosition,
}

struct FunctionExpression {
	decorators: Vec<Decorator>, // experimental
	id: Option<Identifier>,
	params: FunctionParams,
	body: BlockStatement,
	fn_kind: FunctionKind,
	position: MaybePosition,
}
struct ClassExpression {
	decorators: Vec<Decorator>, // experimental
	id: Option<Identifier>,
	extends: Option<Box<Expression>>,
	body: ClassBody,
	position: MaybePosition,
}

struct RegularExpressionLiteral {
	value: String,
	flags: Vec<char>,
	position: MaybePosition,
}

struct TemplateLiteral {
	tag: Option<Box<Expression>>,
	initial: TemplatePart,
	parts: Vec<(Expression, TemplatePart)>,
	position: MaybePosition,
}
struct TemplatePart {
	value: String,
	position: MaybePosition,
}

struct CallExpression {
	callee: Box<Expression>,
	arguments: CallArguments,
	optional: bool,
	position: MaybePosition,
}
struct NewExpression {
	callee: Box<Expression>,
	arguments: CallArguments,
	position: MaybePosition,
}

// experimental
struct ImportExpression {
	argument: Box<Expression>,
	position: MaybePosition,
}

struct CallArguments {
	args: Vec<Box<Expression>>,
	spread: Option<Box<Expression>>,
	position: MaybePosition,
}

struct MemberExpression {
	object: Box<Expression>,
	property: MemberProperty,
	optional: bool,
	position: MaybePosition,
}

enum MemberProperty {
	Normal(PropertyId),
	Private(PrivateProperty),
}

struct PrivateProperty {
	property: PropertyIdentifier,
	position: MaybePosition,
}

struct PrivateExpression {
	property: PropertyIdentifier,
	position: MaybePosition,
}

struct UpdateExpression {
	value: Box<Expression>,
	operator: UpdateOperator,
	position: MaybePosition,
}
enum UpdateOperator {
	PreIncrement,
	PreDecrement,
	PostIncrement,
	PostDecrement,
}

struct UnaryExpression {
	value: Box<Expression>,
	operator: UnaryOperator,
	position: MaybePosition,
}
enum UnaryOperator {
	Delete,
	Void,
	Typeof,
	Positive,
	Negative,
	BitNegate,
	Negate,
	Await,
	Yield,
}

struct BinaryExpression {
	value: Box<Expression>,
	operator: BinaryOperator,
	position: MaybePosition,
}
enum BinaryOperator {
	Compare,
	StrictCompare,
	NegateCompare,
	NegateStrictCompare,
	LessThan,
	LessThanEq,
	GreaterThan,
	GreaterThanEq,
	LeftShift,
	RightShift,
	RightShiftSigned,
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulus,
	BitOr,
	BitAnd,
	BitXor,
	In,
	Instanceof,
	And,
	Or,
	Power,

	Bind, // experimental
}

struct ConditionalExpression {
	test: Box<Expression>,
	alternate: Box<Expression>,
	consequent: Box<Expression>,
	position: MaybePosition,
}

struct AssignmentExpression {
	operator: AssignmentOperator,
	left: Box<Expression>,
	value: Box<Expression>,
	position: MaybePosition,
}
enum AssignmentOperator {
	None,
	Multiply,
	Divide,
	Modulus,
	Add,
	Subtract,
	LeftShift,
	RightShift,
	RightShiftSigned,
	BitAnd,
	BitOr,
	BitXor,
	Power,
}

struct SequenceExpression {
	items: Vec<Expression>,
	position: MaybePosition,
}

enum ArrowFunctionKind {
	Normal,
	Async,
	Generator, // experimental
	AsyncGenerator, // experimental
}
enum ArrowFunctionBody {
	Expression(Box<Expression>),
	Block(BlockStatement),
}

struct ArrowFunctionExpression {
	params: FunctionParams,
	body: ArrowFunctionBody,
	fn_kind: ArrowFunctionKind,
	position: MaybePosition,
}

struct DoExpression {
	body: BlockStatement,
	position: MaybePosition,
}


// Flow
// Typescript?








