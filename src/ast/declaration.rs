use super::misc;
use super::alias;
use super::display;

// TODO: Enum fix?
pub enum DeclaratorList<T: display::NodeDisplay> {
    Last(T),
    List(T, Box<DeclaratorList<T>>),
}
impl<T: display::NodeDisplay> display::NodeDisplay for DeclaratorList<T> {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        match *self {
            DeclaratorList::Last(ref n) => f.node(n),
            DeclaratorList::List(ref n, ref list) => {
                f.node(n)?;
                f.punctuator(display::Punctuator::Comma);
                f.node(list)
            }
        }
    }
}

// let foo, bar;
node!(pub struct LetDeclaration {
    declarators: DeclaratorList<LetDeclarator>,
});
impl display::NodeDisplay for LetDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Let);
        f.node(&self.declarators)
    }
}
node!(pub struct LetDeclarator {
    id: misc::Pattern,
    init: Option<alias::Expression>,
});
impl display::NodeDisplay for LetDeclarator {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.id)?;
        if let Some(ref init) = self.init {
            f.punctuator(display::Punctuator::Eq);
            f.require_precedence(display::Precedence::Assignment).node(
                init,
            )?;
        }
        Ok(())
    }
}


// const foo = 4, bar = 5;
node!(pub struct ConstDeclaration {
    declarators: DeclaratorList<ConstDeclarator>,
});
impl display::NodeDisplay for ConstDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.keyword(display::Keyword::Const);
        f.node(&self.declarators)
    }
}


node!(pub struct ConstDeclarator {
    id: misc::Pattern,
    init: alias::Expression,
});
impl display::NodeDisplay for ConstDeclarator {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.id)?;
        f.punctuator(display::Punctuator::Eq);
        f.require_precedence(display::Precedence::Assignment).node(
            &self.init,
        )
    }
}

// function name() {}
node!(pub struct FunctionDeclaration {
    kind: misc::FunctionKind,
    id: misc::BindingIdentifier,
    params: misc::FunctionParams,
    body: misc::FunctionBody,
});
impl display::NodeDisplay for FunctionDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        f.node(&self.kind)?;
        f.keyword(display::Keyword::Function);
        f.node(&self.id)?;
        f.node(&self.params)?;
        f.node(&self.body)
    }
}


// class name {}
node!(pub struct ClassDeclaration {
    decorators: Vec<misc::Decorator>, // experimental
    id: misc::BindingIdentifier,
    extends: Option<Box<alias::Expression>>,
    body: misc::ClassBody,
});
impl display::NodeDisplay for ClassDeclaration {
    fn fmt(&self, f: &mut display::NodeFormatter) -> display::NodeDisplayResult {
        for dec in self.decorators.iter() {
            f.node(dec)?;
        }

        f.keyword(display::Keyword::Class);

        f.node(&self.id)?;

        if let Some(ref expr) = self.extends {
            f.keyword(display::Keyword::Extends);
            f.require_precedence(display::Precedence::LeftHand).node(
                expr,
            )?;
        }

        f.node(&self.body)
    }
}
