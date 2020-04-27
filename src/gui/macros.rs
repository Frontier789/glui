
#[macro_export]
macro_rules! register_gui_element_struct_init {
    // base cases
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ .. $( $updated:tt )* ) => {
        $class {
            $( $field_in : $value_in ,)*
            .. $( $updated )*
        }
    };
    
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ ) => {
        $class {
            $( $field_in : $value_in ,)*
            ..Default::default()
        }
    };
    
    
    // take out children
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ children : $value_c:expr $(,)? ) => {
        register_gui_element_struct_init! {
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* } @
        }
    };
    
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ children : $value_c:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* } @
            $( $rest )*
        )
    };
    
    // take out child
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ child : $value_c:expr $(,)? ) => {
        register_gui_element_struct_init! {
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* } @
        }
    };
    
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ child : $value_c:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* } @
            $( $rest )*
        )
    };
    
    // handle |data|{} callbacks
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : |$data:ident| $body:expr $(,)? ) => {
        register_gui_element_struct_init! {
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $parser::make_callback1(move |$data : &mut $build_param| $body) , } @
        }
    };
    
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : |$data:ident| $body:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $parser::make_callback1(move |$data : &mut $build_param| $body) , } @
            $( $rest )*
        )
    };
    
    // handle |data,sender|{} callbacks
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : |$data:ident,$postbox:ident| $body:expr $(,)? ) => {
        register_gui_element_struct_init! {
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $parser::make_callback2(move |$data : &mut $build_param, $postbox: &mut $sender| $body) , } @
        }
    };
    
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : |$data:ident,$postbox:ident| $body:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $parser::make_callback2(move |$data : &mut $build_param, $postbox: &mut $sender| $body) , } @
            $( $rest )*
        )
    };
    
    // keep anything else
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : $value_c:expr $(,)? ) => {
        register_gui_element_struct_init! {
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $value_c , } @
        }
    };
    
    ( $build_param:ty, $sender:ty, $parser:ident, $class:ident { $( $field_in:ident : $value_in:expr ,)* } @ $field_c:ident : $value_c:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $build_param, $sender, $parser,
            $class { $( $field_in : $value_in ,)* $field_c : $value_c , } @
            $( $rest )*
        )
    };
}

#[macro_export]
macro_rules! register_gui_element_children {
    ( children : $f:expr, $( $x:tt )* ) => {
        $f
    };
    ( children : $f:expr ) => {
        $f
    };
    ( child : $f:expr, $( $x:tt )* ) => {
        {
            $f;
        }
    };
    ( $field_c:ident : $value_c:expr, $( $rest:tt )* ) => {
        register_gui_element_children! {
            $( $rest )*
        }
    };
    ( $( $field:ident : $value:expr),* $(,)? ) => {
    };
    ( .. $( $rest:tt )* ) => {
    }
}

#[macro_export]
macro_rules! register_gui_element {
    ($class:ident, $build_param:ty, $sender:ty, $parser:ident @ $( $x:tt )* ) => {
        {
            let tmp = register_gui_element_struct_init! { $build_param, $sender, $parser, $class {} @ $( $x )* };
            $parser::parse_push(tmp);
            
            register_gui_element_children! { $( $x )* }
            
            $parser::parse_pop();
        }
    };
}