#[macro_export]
macro_rules! match_downcast {
    ( $any:ident { _ : $type:ty => $arm:expr, $($rest:tt)* } ) => (
        if $any.is::<$type>() {
            $arm
        } else {
            match_downcast!( $any { $( $rest )* } )
        }
    );
    ( $any:ident { $type:ident ( $($pat:pat),* ) $(if $cond:expr)? => $arm:expr, $($rest:tt)* } ) => (
        if if $any.is::<$type>() {
            let any_ref = $any.downcast_ref::<PrintTimes>().unwrap();
            
            match *any_ref {
                $type ( $($pat),* ) $(if $cond)? => {
                    $arm
                    false
                },
                #[allow(unreachable_patterns)]
                _ => {
                    true
                }
            }
        } else {true} {
            match_downcast!( $any { $( $rest )* } )
        }
    );
    ( $any:ident { _ => $default:expr $(,)? } ) => (
        $default
    );
    ( $any:ident { $bind:ident : $type:ty => $arm:expr, $($rest:tt)* } ) => (
        if $any.is::<$type>() {
            let $bind = $any.downcast::<$type>().unwrap();
            $arm
        } else {
            match_downcast!( $any { $( $rest )* } )
        }
    );
}
