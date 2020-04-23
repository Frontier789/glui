#[macro_export]
macro_rules! match_downcast {
    ( $any:ident { _ : $type:ident $(if $cond:expr)? => $arm:expr, $($rest:tt)* } ) => (
        if $any.is::<$type>() $(&& if $cond:expr)? {
            $arm
        } else {
            match_downcast!( $any { $( $rest )* } )
        }
    );
    ( $any:ident { _ => $default:expr $(,)? } ) => (
        $default
    );
    ( $any:ident { $bind:ident : $type:ty => $arm:expr, $($rest:tt)* } ) => (
        if $any.is::<$type>() {
            let $bind = *$any.downcast::<$type>().unwrap();
            $arm
        } else {
            match_downcast!( $any { $( $rest )* } )
        }
    );
}
