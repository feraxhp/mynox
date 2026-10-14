#[macro_export]
macro_rules! empty_iter { () => {{ [""; 0] }}; }

#[macro_export]
macro_rules! error {
    ($etype:expr, $message:expr, $detail:expr) => {{
        $crate::Error::new($etype, $message, $detail, [""; 0], [""; 0])
    }};
    (
        $etype:expr, 
        $message:expr, 
        $detail:expr, 
            $( $content:expr ),* $(,)?
    ) => {{
        $crate::Error::new($etype, $message, $detail, [
            $( $content, )*
        ], [""; 0])
    }};
    (
        $etype:expr, 
        $message:expr, 
        $detail:expr,  =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::new($etype, $message, $detail, [""; 0], [
            $( $notes, )*
        ])
    }};
    (
        $etype:expr, 
        $message:expr, 
        $detail:expr =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::new($etype, $message, $detail, [""; 0], [
            $( $notes, )*
        ])
    }};
    (
        $etype:expr, 
        $message:expr, 
        $detail:expr, 
            $( $content:expr ),* $(,)? =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::new($etype, $message, $detail, [
            $( $content, )*
        ], [
            $( $notes, )*
        ])
    }};
    (source: $source:expr, $etype:expr, $message:expr, $detail:expr) => {{
        $crate::Error::from($source, $etype, $message, $detail, [""; 0], [""; 0])
    }};
    (
        source: $source:expr, 
        $etype:expr, 
        $message:expr, 
        $detail:expr, 
            $( $content:expr ),* $(,)?
    ) => {{
        $crate::Error::from($source, $etype, $message, $detail, [
            $( $content, )*
        ], [""; 0])
    }};
    (
        source: $source:expr, 
        $etype:expr, 
        $message:expr, 
        $detail:expr,  =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::from($source, $etype, $message, $detail, [""; 0], [
            $( $notes, )*
        ])
    }};
    (
        source: $source:expr, 
        $etype:expr, 
        $message:expr, 
        $detail:expr =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::from($source, $etype, $message, $detail, [""; 0], [
            $( $notes, )*
        ])
    }};
    (
        source: $source:expr, 
        $etype:expr, 
        $message:expr, 
        $detail:expr, 
            $( $content:expr ),* $(,)? =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::from($source, $etype, $message, $detail, [
            $( $content, )*
        ], [
            $( $notes, )*
        ])
    }};
}
