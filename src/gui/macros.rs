
macro_rules! register_gui_element_struct_init {
    ( $class:ident { $( $field_in:ident : $value_in:expr ),* } @ ) => {
        $class {
            $( $field_in : $value_in ),*
        }
    };
    ( $class:ident { $( $field_in:ident : $value_in:expr ),* } @ children : $value_c:block $(, $field:ident : $value:expr )* $(,)? ) => {
        register_gui_element_struct_init! (
            $class { $( $field_in : $value_in ),* } @
            $( $field:ident : $value:expr ),*
        )
    };
    ( $class:ident { $( $field_in:ident : $value_in:expr ),* } @ $field_c:ident : $value_c:expr , $( $rest:tt )* ) => {
        register_gui_element_struct_init! (
            $class { $( $field_in : $value_in ),* $field_c : $value_c } @
            $( $rest )*
        )
    };
    ( $class:ident { $( $field_in:ident : $value_in:expr ),* } @ $field_c:ident : $value_c:expr ) => {
        register_gui_element_struct_init! (
            $class { $( $field_in : $value_in ),* $field_c : $value_c } @
        )
    };
}

macro_rules! register_gui_element_children {
    ( children : $f:expr, $( $x:tt),* $(,)? ) => {
        $f
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
    ($class:ident, $context:ident @ $( $x:tt )* ) => {
        {
            let tmp = register_gui_element_struct_init! { $class {} @ $( $x )* };
            if $context.parse_start(&tmp) {
                register_gui_element_children! { $( $x )* }
            }
            $context.parse_end(&tmp);
        }
    };
}