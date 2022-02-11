
#[macro_export]
macro_rules! zig2rs {
    () => {};
    ($($code:tt)*) => {
        zig2rs_parser_core! {
            $($code)*
        }
    };
}

#[macro_export]
macro_rules! zig2rs_parser_core {
    () => {
        compile_error!("ended up with empty parser core input")
    };

    // Function declaration
    (
        // TODO: fix type parsing to allow more than 1 identifier
        // it takes a single identifier to prevent weird errors
        // it should in fact take entire *expressions* but for that
        // I'd definitely need a proc-macro-based compiler, as Rust
        // isn't capable of this magic lmao
        //
        // in general, most of the language should be implemented in
        // proc macros, as this is very not enough

        fn $fname:ident (
            $( $pname:ident : $ptype_raw:ident $(,)? )*
        ) $fret:ident {
            $( $body:tt )*
        }

        $($tail:tt)*
    ) => {
        unsafe fn $fname (
            $( $pname : zig2rs_parse_type!{ $ptype_raw } )*
        ) -> zig2rs_parse_type!{ $fret } {
            zig2rs_emit_scope! {
                $($body)*
            }
        }
    };
}

#[macro_export]
macro_rules! zig2rs_parse_type {
    (void) => { () };
    (usize) => { usize };
}

#[macro_export]
macro_rules! zig2rs_emit_scope {
    ($($body:tt)*) => {
        {
            zig2rs_emit_scope_impl! {
                $($body)*
                __break_line__
            }
        }
    };
}

#[macro_export]
macro_rules! zig2rs_emit_scope_impl {
    (
        __break_line__
        $($processed:tt)*
    ) => {
        $($processed)*
    };

    // Constant creating
    (
        const $vname:ident $(: $vtype:ident)? = $vvalue:expr;
        $($tail:tt)*
    ) => {
        zig2rs_emit_scope_impl! {
            $($tail)*
            let $vname $(:$vtype)? = ($vvalue).zigify();
        }
    };

    // Varaible creating
    (
        var $vname:ident $(: $vtype:ident)? = $vvalue:expr;
        $($tail:tt)*
    ) => {
        zig2rs_emit_scope_impl! {
            $($tail)*
            let mut $vname $(:$vtype)? = ($vvalue).zigify();
        }
    };

    // Function call creating
    (
        $fname:ident ( $($inner:tt)* );
        $($tail:tt)*
    ) => {
        zig2rs_emit_scope_impl! {
            $($tail)*
            $fname($($inner)*);
        }
    };

    // While loops
    (
        while ($condition:expr) : ($($continuation:tt)*) {
            $($body:tt)*
        }
        $($tail:tt)*
    ) => {
        zig2rs_emit_scope_impl! {
            $($tail)*
            while $condition {
                zig2rs_emit_scope! { $($body)* }
                $($continuation)*;
            }
        }
    };
}
