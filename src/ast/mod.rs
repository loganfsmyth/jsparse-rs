#[macro_use]
pub mod misc;

pub mod alias;
pub mod jsx;
pub mod flow;
pub mod expression;
pub mod statement;
pub mod declaration;
pub mod modules;
pub mod literal;
mod display;

// pub use misc;

// TODO
// Typescript?

// pub mod nodes {
//     use super::expression;

//     pub type MemberExpression = Box<expression::Member>;

//     struct MemberExpressionBuilder {}
//     impl MemberExpression {
//         fn build() -> MemberExpressionBuilder {
//             MemberExpressionBuilder {}
//         }
//     }
// }
