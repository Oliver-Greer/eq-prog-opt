#[macro_export]
macro_rules! primitive {
    (@impl $func:path, $( $arg:ident = $index:tt ),+ $(,)?) => {
        Primitive {
            f: |ctx, args| {
                $func(ctx, $(args[$index].into()),+).into()
            },
        }
    };
    (1, $func:path $(,)?) => {
        primitive!(@impl $func, a = 0)
    };
    (2, $func:path $(,)?) => {
        primitive!(@impl $func, a = 0, b = 1)
    };
    (3, $func:path $(,)?) => {
        primitive!(@impl $func, a = 0, b = 1, c = 2)
    };
    (4, $func:path $(,)?) => {
        primitive!(@impl $func, a = 0, b = 1, c = 2, d = 3)
    };
}

#[macro_export]
macro_rules! condition {
    (@impl $func:path, $( $arg:ident = $index:tt ),+ $(,)?) => {
        Cond {
            f: |ctx, args| {
                $func(ctx, $(args[$index].into()),+).into()
            },
        }
    };
    (1, $func:path $(,)?) => {
        condition!(@impl $func, a = 0)
    };
    (2, $func:path $(,)?) => {
        condition!(@impl $func, a = 0, b = 1)
    };
    (3, $func:path $(,)?) => {
        condition!(@impl $func, a = 0, b = 1, c = 2)
    };
    (4, $func:path $(,)?) => {
        condition!(@impl $func, a = 0, b = 1, c = 2, d = 3)
    };
}