// Enforce the structure of enums that store flags.
macro_rules! node_kind {
    (pub enum $name:ident { $($key:ident ,)* }) => {
        #[derive(Debug)]
        pub enum $name {
            $($key ,)*
        }
    };
}

// Enforce the structure of enums that wrap multiple node types into a single item.
macro_rules! node_enum {
    ( ( $(@$label:tt)* ) pub enum $id:ident $body:tt ) => {
        #[derive(Debug)]
        pub enum $id $body

        node_enum!(@impl @from $id $body);
        $(
            node_enum!(@impl @$label $id $body);
        )*
    };
    (@impl @from $name:ident { $( $key:ident($type:ty) ,)* }) => {
        $(
            impl From<$type> for $name {
                fn from(val: $type) -> $name {
                    $name::$key(val)
                }
            }
            impl From<$type> for Box<$name> {
                fn from(val: $type) -> Box<$name> {
                    Box::new($name::$key(val))
                }
            }
            impl From<$type> for Option<$name> {
                fn from(val: $type) -> Option<$name> {
                    Some($name::$key(val))
                }
            }
            impl From<$type> for Option<Box<$name>> {
                fn from(val: $type) -> Option<Box<$name>> {
                    Some(Box::new($name::$key(val)))
                }
            }
        )*
    };
    (@impl @node_display $name:ident { $( $key:ident($type:ty) ,)* }) => {
        impl $crate::ast::display::NodeDisplay for $name {
            fn fmt(&self, f: &mut $crate::ast::display::NodeFormatter)
                -> $crate::ast::display::NodeDisplayResult
            {
                match *self {
                    $(
                        $name::$key(ref n) => f.node(n),
                    )*
                }
            }
        }
    };
    (@impl @leading_comments $name:ident { $( $key:ident($type:ty) ,)* }) => {
        impl $crate::ast::LeadingComments for $name {
            fn leading_comments(&self) -> $crate::ast::CommentIterator {
                match *self {
                    $(
                        $name::$key(ref n) => n.leading_comments(),
                    )*
                }
            }
        }
    };

    (@$label1:ident @$label2:ident @$label3:ident @$label4:ident $($it:tt)*) => {
        node_enum!((@$label1 @$label2 @$label3 @$label4) $($it)*);
    };
    (@$label1:ident @$label2:ident @$label3:ident $($it:tt)*) => {
        node_enum!((@$label1 @$label2 @$label3) $($it)*);
    };
    (@$label1:ident @$label2:ident $($it:tt)*) => {
        node_enum!((@$label1 @$label2) $($it)*);
    };
    (@$label1:ident $($it:tt)*) => {
        node_enum!((@$label1) $($it)*);
    };
    ($($it:tt)*) => {
        node_enum!(() $($it)*);
    };
}

// Enforce structure for AST node structs.
macro_rules! node {
    (@node_display $id:ident) => {
        impl ::std::fmt::Display for $id {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut node_fmt = $crate::ast::display::NodeFormatter::new();

                $crate::ast::display::NodeDisplay::fmt(self, &mut node_fmt).unwrap();

                write!(f, "{}", node_fmt.output)
            }
        }
    };
    (@ensure_debug [ derive( $($t:ident),* )] $item:item) => {
        #[derive(Debug, $($t),*  )] $item
    };
    (pub struct $id:ident { $(pub $field_id:ident: $field_type:ty ,)* }) => {
        #[derive(Debug)]
        pub struct $id {
            // TODO: This 'pub' should be in the declarations themselves.
            $(pub $field_id: $field_type,)*
            pub position: Option<Box<$crate::ast::NodePosition>>,
        }
        node!(@node_display $id);
    };
    (#$meta:tt pub struct $id:ident { $(pub $field_id:ident: $field_type:ty ,)* }) => {
        node!(@ensure_debug $meta pub struct $id {
            // TODO: This 'pub' should be in the declarations themselves.
            $(pub $field_id: $field_type,)*
            pub position: Option<Box<$crate::ast::NodePosition>>,
        });
        node!(@node_display $id);
    };
}

#[allow(unused_macros)]
macro_rules! assert_serialize {
    ($item:expr, $s:expr) => {
        assert_eq!(format!("{}", $item), $crate::std::string::String::from($s));
    };
}


pub mod alias;
pub mod classes;
pub mod decorators;
mod display;
pub mod expression;
pub mod functions;
pub mod general;
pub mod jsx;
pub mod literal;
pub mod modules;
pub mod objects;
pub mod patterns;
pub mod root;
pub mod statement;

use std::iter::Iterator;

#[derive(Debug)]
pub struct NodePosition {
    pub start: usize,
    pub end: usize,
    pub range: PositionRange,
}

#[derive(Debug)]
pub struct PositionRange {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

pub type MaybeTokenPosition = Option<Box<PositionRange>>;
pub type SeparatorTokens = Vec<Separators>;

pub type SeparatorTokensSingleLine = Vec<Separators>;

#[derive(Default, Debug)]
pub struct KeywordData {
    prefix: SeparatorTokens,
    position: MaybeTokenPosition,
}
#[derive(Default, Debug)]
pub struct KeywordSuffixData {
    position: MaybeTokenPosition,
    suffix: SeparatorTokens,
}
#[derive(Default, Debug)]
pub struct KeywordWrappedData {
    position: MaybeTokenPosition,
    suffix: SeparatorTokens,
}


pub trait LeadingComments {
    fn leading_comments(&self) -> CommentIterator;
}

pub trait SeparatorList {
    // fn

}

#[derive(Debug)]
pub enum Separators {
    Comment(CommentNode),
    Whitespace,
}

pub struct CommentNode;

pub struct CommentIterator<'a> {
    it: ::std::slice::Iter<'a, Separators>,
}
impl<'a> CommentIterator<'a> {
    fn new(v: &'a Vec<Separators>) -> CommentIterator<'a> {
        CommentIterator {
            it: v.iter(),
        }
    }
}
impl<'a> Iterator for CommentIterator<'a> {
    type Item = &'a CommentNode;

    fn next(&mut self) -> Option<&'a CommentNode> {
        while let Some(sep) = self.it.next() {
            match *sep {
                Separators::Comment(ref comment) => {
                    return Some(comment);
                }
                _ => {},
            }
        }
        None
    }
}
