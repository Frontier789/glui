
macro_rules! register_gui_element_struct_init {
    // base case
    ( $build_param:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ ) => {
        $class {
            $( $field_in : $value_in ,)*
            ..Default::default()
        }
    };
    
    // take out children
    ( $build_param:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ children : $value_c:expr $(,)? ) => {
        register_gui_element_struct_init! {
            $build_param, $parser,
            $class { $( $field_in : $value_in ,)* } @
        }
    };
    
    ( $build_param:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ children : $value_c:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $build_param, $parser,
            $class { $( $field_in : $value_in ,)* } @
            $( $rest )*
        )
    };
    
    // take out child
    ( $build_param:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ child : $value_c:expr $(,)? ) => {
        register_gui_element_struct_init! {
            $build_param, $parser,
            $class { $( $field_in : $value_in ,)* } @
        }
    };
    
    ( $build_param:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ child : $value_c:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $build_param, $parser,
            $class { $( $field_in : $value_in ,)* } @
            $( $rest )*
        )
    };
    
    // handle callbacks
    ( $build_param:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : |$param:ident| $body:block $(,)? ) => {
        register_gui_element_struct_init! {
            $build_param, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $parser::make_callback(move |$param : &mut $build_param| $body) , } @
        }
    };
    
    ( $build_param:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : |$param:ident| $body:block , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $build_param, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $parser::make_callback(move |$param : &mut $build_param| $body) , } @
            $( $rest )*
        )
    };
    
    // keep anything else
    ( $build_param:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : $value_c:expr $(,)? ) => {
        register_gui_element_struct_init! {
            $build_param, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $value_c , } @
        }
    };
    
    ( $build_param:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : $value_c:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $build_param, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $value_c , } @
            $( $rest )*
        )
    };
}

macro_rules! register_gui_element_children {
    ( children : $f:expr, $( $x:tt),* $(,)? ) => {
        $f
    };
    ( child : $f:expr, $( $x:tt),* $(,)? ) => {
        {
            $f;
        }
    };
    ( $field_c:ident : $value_c:expr, $( $field:ident : $value:expr),* $(,)? ) => {
        register_gui_element_children! {
            $( $field : $value, )*
        }
    };
    ( $( $field:ident : $value:expr),* $(,)? ) => {
    }
}

macro_rules! register_gui_element {
    ($class:ident, $build_param:ty, $parser:ident @ $( $x:tt )* ) => {
        {
            let tmp = register_gui_element_struct_init! { $build_param, $parser, $class {} @ $( $x )* };
            $parser::parse_push(tmp);
            
            register_gui_element_children! { $( $x )* }
            
            $parser::parse_pop();
        }
    };
}