pub const ESCAPE: char = '\x1b';

#[macro_export]
macro_rules! log {
    (
        $color:expr,
        $decoration:expr,
        $($params:tt)*
    ) => {
        if !crate::flag_set(opts!(), "silent") {
            let e_logger_msg = format!("{}", format!($($params)*));
            if (e_logger_msg.contains("\n")) {
                for e_logger_line in e_logger_msg.split("\n") {
                    let e_logger_line = e_logger_line.strip_prefix("\r").unwrap_or_else(||e_logger_line);
                    println!("[{}{} {} {}[0m]: {}", crate::logging::ESCAPE, $color, $decoration, crate::logging::ESCAPE, e_logger_line);
                }
            }
            else {
                println!("[{}{} {} {}[0m]: {}", crate::logging::ESCAPE, $color, $decoration, crate::logging::ESCAPE, e_logger_msg);
            }
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($params:tt)*) => {
        log!("[1;35m", "DEBUG", $($params)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($params:tt)*) => {
        if crate::flag_set(opts!(), "verbose") {
            log!("[1;36m", "INFO", $($params)*);
        }
    };
}

#[macro_export]
macro_rules! ok {
    ($($params:tt)*) => {
        if crate::flag_set(opts!(), "verbose") {
            log!("[1;32m", "OK", $($params)*);
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($params:tt)*) => {
        if !crate::flag_set(opts!(), "soft-silent") {
            log!("[1;33m", "WARN", $($params)*);
        }
    };
}

#[macro_export]
macro_rules! err {
    ($($params:tt)*) => {
        log!("[1;31m", "ERROR", $($params)*);
    };
}
