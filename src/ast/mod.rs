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
