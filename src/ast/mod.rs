#[macro_use]
pub mod misc;
pub mod alias;
pub mod jsx;
pub mod flow;
pub mod expression;
pub mod statement;
pub mod declaration;
pub mod literal;

// pub use misc;

// TODO
// Typescript?

pub mod nodes {
  pub type MemberExpression = Box<expression::Member>;
  struct MemberExpressionBuilder {

  }
  impl MemberExpression {
    fn build() -> MemberExpressionBuilder {
      MemberExpressionBuilder {}
    }
  }
}
