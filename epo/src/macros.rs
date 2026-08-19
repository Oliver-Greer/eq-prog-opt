#[macro_export]
macro_rules! primitive {
    ($func:path, $( $arg:ident = $index:tt ),+ $(,)?) => {
        Primitive {
            f: |args| {
                $func($(args[$index].into()),+).into()
            },
        }
    };
    (1, $func:path $(,)?) => {
        primitive!($func, a = 0)
    };
    (2, $func:path $(,)?) => {
        primitive!($func, a = 0, b = 1)
    };
    (3, $func:path $(,)?) => {
        primitive!($func, a = 0, b = 1, c = 2)
    };
    (4, $func:path $(,)?) => {
        primitive!($func, a = 0, b = 1, c = 2, d = 3)
    };
}

#[macro_export]
macro_rules! condition {
    ($func:path, $( $arg:ident = $index:tt ),+ $(,)?) => {
        Cond {
            f: |args| {
                $func($(args[$index].into()),+).into()
            },
        }
    };
    (1, $func:path $(,)?) => {
        condition!($func, a = 0)
    };
    (2, $func:path $(,)?) => {
        condition!($func, a = 0, b = 1)
    };
    (3, $func:path $(,)?) => {
        condition!($func, a = 0, b = 1, c = 2)
    };
    (4, $func:path $(,)?) => {
        condition!($func, a = 0, b = 1, c = 2, d = 3)
    };
}