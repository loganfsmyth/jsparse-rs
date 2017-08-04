
macro_rules! node_kind {
    (pub enum $name:ident { $($key:ident ,)* }) => {
        pub enum $name {
            $($key ,)*
        }
    };
}

macro_rules! node_enum_impl {
    ( ( $(@$label:tt)* ) pub enum $id:ident $body:tt ) => {
        pub enum $id $body

        node_enum_impl!(@from $id $body);
        $(
            node_enum_impl!(@$label $id $body);
        )*
    };
    (@from $name:ident { $( $key:ident($type:ty) ,)* }) => {
        $(
            impl From<$type> for $name {
                fn from(val: $type) -> $name {
                    $name::$key(val)
                }
            }
        )*
    };
    (@has_in_operator $name:ident { $( $key:ident($type:ty) ,)* }) => {
        impl $crate::ast::display::HasInOperator for $name {
            fn has_in_operator(&self) -> bool {
                match *self {
                    $(
                        $name::$key(ref n) => n.has_in_operator(),
                    )*
                }
            }
        }
    };
    (@node_display $name:ident { $( $key:ident($type:ty) ,)* }) => {
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
    (@first_special_token $name:ident { $( $key:ident($type:ty) ,)* }) => {
        impl $crate::ast::display::FirstSpecialToken for $name {
            fn first_special_token(&self) -> $crate::ast::display::SpecialToken {
                match *self {
                    $(
                        $name::$key(ref n) => n.first_special_token(),
                    )*
                }
            }
        }
    };
    (@orphan_if $name:ident { $( $key:ident($type:ty) ,)* }) => {
        impl $crate::ast::display::HasOrphanIf for $name {
            fn orphan_if(&self) -> bool {
                match *self {
                    $(
                        $name::$key(ref n) => n.orphan_if(),
                    )*
                }
            }
        }
    };
}

macro_rules! node_enum {
    (@$label1:ident @$label2:ident @$label3:ident @$label4:ident $($it:tt)*) => {
        node_enum_impl!((@$label1 @$label2 @$label3 @$label4) $($it)*);
    };
    (@$label1:ident @$label2:ident @$label3:ident $($it:tt)*) => {
        node_enum_impl!((@$label1 @$label2 @$label3) $($it)*);
    };
    (@$label1:ident @$label2:ident $($it:tt)*) => {
        node_enum_impl!((@$label1 @$label2) $($it)*);
    };
    (@$label1:ident $($it:tt)*) => {
        node_enum_impl!((@$label1) $($it)*);
    };
    ($($it:tt)*) => {
        node_enum_impl!(() $($it)*);
    };
}

macro_rules! assert_serialize {
    ($id:ident, { $($key:ident: $val:expr),* $(,)* }, $s:expr) => {
        {
            let o = $id {
                position: None,
                $($key: $val),*
            };

            assert_eq!(format!("{}", o), $crate::std::string::String::from($s));
        }
    };
}

macro_rules! node_display {
    ($id:ident) => {
        impl ::std::fmt::Display for $id {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut node_fmt = $crate::ast::display::NodeFormatter::new();

                $crate::ast::display::NodeDisplay::fmt(self, &mut node_fmt).unwrap();

                write!(f, "{}", node_fmt.output)
            }
        }
    };
}

macro_rules! node {
    (pub struct $id:ident { $($field_id:ident: $field_type:ty ,)* }) => {
        pub struct $id {
            $($field_id: $field_type,)*
            pub position: Option<Box<$crate::ast::NodePosition>>,
        }
        node_display!($id);
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


pub struct NodePosition {
    pub start: usize,
    pub end: usize,
    pub range: PositionRange,
}
pub struct PositionRange {
    pub start: (usize, usize),
    pub end: (usize, usize),
}
