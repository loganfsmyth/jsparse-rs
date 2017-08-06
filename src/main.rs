extern crate jsparse;


fn main() {
    println!("alias::Expression: {}", std::mem::size_of::<jsparse::ast::alias::Expression>());
    println!("String: {}", std::mem::size_of::<String>());

    println!("general::BindingIdentifier: {}", std::mem::size_of::<jsparse::ast::general::BindingIdentifier>());
    println!("expression::ThisExpression: {}", std::mem::size_of::<jsparse::ast::expression::ThisExpression>());
    println!("objects::ArrayExpression: {}", std::mem::size_of::<jsparse::ast::objects::ArrayExpression>());
    println!("objects::ObjectExpression: {}", std::mem::size_of::<jsparse::ast::objects::ObjectExpression>());
    println!("literal::Null: {}", std::mem::size_of::<jsparse::ast::literal::Null>());
    println!("literal::Boolean: {}", std::mem::size_of::<jsparse::ast::literal::Boolean>());
    println!("literal::Numeric: {}", std::mem::size_of::<jsparse::ast::literal::Numeric>());
    println!("literal::String: {}", std::mem::size_of::<jsparse::ast::literal::String>());
    println!("functions::FunctionExpression: {}", std::mem::size_of::<jsparse::ast::functions::FunctionExpression>());
    println!("classes::ClassExpression: {}", std::mem::size_of::<jsparse::ast::classes::ClassExpression>());
    println!("literal::RegExp: {}", std::mem::size_of::<jsparse::ast::literal::RegExp>());
    println!("expression::TemplateLiteral: {}", std::mem::size_of::<jsparse::ast::expression::TemplateLiteral>());
    println!("expression::MemberExpression: {}", std::mem::size_of::<jsparse::ast::expression::MemberExpression>());
    println!("expression::SuperMemberExpression: {}", std::mem::size_of::<jsparse::ast::expression::SuperMemberExpression>());
    println!("expression::BinaryExpression: {}", std::mem::size_of::<jsparse::ast::expression::BinaryExpression>());
    println!("expression::UnaryExpression: {}", std::mem::size_of::<jsparse::ast::expression::UnaryExpression>());
    println!("expression::UpdateExpression: {}", std::mem::size_of::<jsparse::ast::expression::UpdateExpression>());
    println!("expression::CallExpression: {}", std::mem::size_of::<jsparse::ast::expression::CallExpression>());
    println!("expression::NewExpression: {}", std::mem::size_of::<jsparse::ast::expression::NewExpression>());
    println!("expression::ImportCallExpression: {}", std::mem::size_of::<jsparse::ast::expression::ImportCallExpression>());
    println!("expression::SuperCallExpression: {}", std::mem::size_of::<jsparse::ast::expression::SuperCallExpression>());
    println!("expression::ConditionalExpression: {}", std::mem::size_of::<jsparse::ast::expression::ConditionalExpression>());
    println!("expression::AssignmentExpression: {}", std::mem::size_of::<jsparse::ast::expression::AssignmentExpression>());
    println!("expression::AssignmentUpdateExpression: {}", std::mem::size_of::<jsparse::ast::expression::AssignmentUpdateExpression>());
    println!("expression::SequenceExpression: {}", std::mem::size_of::<jsparse::ast::expression::SequenceExpression>());
    println!("functions::ArrowFunctionExpression: {}", std::mem::size_of::<jsparse::ast::functions::ArrowFunctionExpression>());
    println!("expression::DoExpression: {}", std::mem::size_of::<jsparse::ast::expression::DoExpression>());
    println!("jsx::Element: {}", std::mem::size_of::<jsparse::ast::jsx::Element>());
}
