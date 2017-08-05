
macro_rules! node_kind {
    (pub enum $name:ident { $($key:ident ,)* }) => {
        #[derive(Debug)]
        pub enum $name {
            $($key ,)*
        }
    };
}

macro_rules! node_enum_impl {
    ( ( $(@$label:tt)* ) pub enum $id:ident $body:tt ) => {
        #[derive(Debug)]
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
    ($item:expr, $s:expr) => {
        assert_eq!(format!("{}", $item), $crate::std::string::String::from($s));
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
        node_display!($id);
    };
    (#$meta:tt pub struct $id:ident { $(pub $field_id:ident: $field_type:ty ,)* }) => {
        node!(@ensure_debug $meta pub struct $id {
            // TODO: This 'pub' should be in the declarations themselves.
            $(pub $field_id: $field_type,)*
            pub position: Option<Box<$crate::ast::NodePosition>>,
        });
        node_display!($id);
    };
}

macro_rules! display_dsl {
    (@opt_body $self:ident $f:ident $id:tt @ $($t:tt)*) => {
        $f.node($id)?;

        display_dsl!(@opt_body $self $f $id $($t)*);
    };
    (@opt_body $self:ident $f:ident $id:ident $tok:tt $($t:tt)*) => {
        display_dsl!(@dsl $self $f $tok);

        display_dsl!(@opt_body $self $f $id $($t)*);
    };
    (@opt_body $self:ident $f:ident $id:ident) => {};


    (@dsl $self:ident $f:ident @?$prop:ident [ $($body:tt)* ] $($t:tt)*) => {
        if let Some(ref v) = $self.$prop {
            display_dsl!(@opt_body $self $f v $($body)*);

        }

        display_dsl!(@dsl $self $f $($t)*);
    };
    (@dsl $self:ident $f:ident @?$prop:ident $($t:tt)*) => {
        if let Some(ref v) = $self.$prop {
            $f.node(v)?;
        }

        display_dsl!(@dsl $self $f $($t)*);
    };
    (@dsl $self:ident $f:ident @[$prop:ident] $($t:tt)*) => {
        for n in $self.$prop.iter() {
            $f.node(n)?;
        }
        display_dsl!(@dsl $self $f $($t)*);
    };
    (@dsl $self:ident $f:ident @[$prop:ident,] $($t:tt)*) => {
        // for (i, n) in $self.$prop.iter().enumerate() {
        //     if i != 0 {
        //         display_dsl!(@token $f ,);
        //     }

        //     $f.node(n)?;
        // }
        let ref n = $self.$prop;
        $f.comma_list(n)?;

        display_dsl!(@dsl $self $f $($t)*);
    };
    (@dsl $self:ident $f:ident @in $($t:tt)*) => {
        let mut f = $f.allow_in();
        display_dsl!(@dsl $self $f $($t)*);
    };
    (@dsl $self:ident $f:ident @ @ $($t:tt)*) => {
        display_dsl!(@dsl $self $f @);
        display_dsl!(@dsl $self $f $($t)*);
    };
    (@dsl $self:ident $f:ident @ $prop:ident $($t:tt)*) => {
        let ref n = $self.$prop;
        $f.node(n)?;

        display_dsl!(@dsl $self $f $($t)*);
    };
    (@dsl $self:ident $f:ident [ $($t:tt)* ] $($t2:tt)*) => {
        $f.punctuator(Punctuator::SquareL);
        display_dsl!(@dsl $self $f $($t)*);
        $f.punctuator(Punctuator::SquareR);
        display_dsl!(@dsl $self $f $($t2)*);
    };
    (@dsl $self:ident $f:ident { $($t:tt)* } $($t2:tt)*) => {
        $f.punctuator(Punctuator::CurlyL);
        display_dsl!(@dsl $self $f $($t)*);
        $f.punctuator(Punctuator::CurlyR);
        display_dsl!(@dsl $self $f $($t2)*);
    };
    (@dsl $self:ident $f:ident ( $($t:tt)* ) $($t2:tt)*) => {
        {
            let mut f = $f.wrap_parens();
            display_dsl!(@dsl $self f $($t)*);
        }
        display_dsl!(@dsl $self $f $($t2)*);
    };
    (@dsl $self:ident $f:ident $tok:tt $($t:tt)*) => {
        display_dsl!(@token $f $tok);
        display_dsl!(@dsl $self $f $($t)*);
    };
    (@dsl $self:ident $f:ident) => {};

    (@token $f:ident =) => {
        $f.punctuator(Punctuator::Eq);
    };
    (@token $f:ident ,) => {
        $f.punctuator(Punctuator::Comma);
    };
    (@token $f:ident @) => {
        $f.punctuator(Punctuator::At);
    };
    (@token $f:ident ;) => {
        $f.punctuator(Punctuator::Semicolon);
    };
    (@token $f:ident var) => {
        $f.keyword(Keyword::Var);
    };
    (@token $f:ident export) => {
        $f.keyword(Keyword::Export);
    };
    (@token $f:ident default) => {
        $f.keyword(Keyword::Default);
    };
    (@token $f:ident class) => {
        $f.keyword(Keyword::Class);
    };
    (@token $f:ident extends) => {
        $f.keyword(Keyword::Extends);
    };

    ($name:ident : $($t:tt)*) => {
        impl $crate::ast::display::NodeDisplay for $name {
            fn fmt(&self, f: &mut $crate::ast::display::NodeFormatter)
                -> $crate::ast::display::NodeDisplayResult
            {
                display_dsl!(@dsl self f $($t)*);

                Ok(())
            }
        }
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
