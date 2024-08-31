pub const HELP_MSG: &str = "Use flag --help to see the documentation.";

/// Check if length of arguments matches
/// the minimal required number of arguments and
/// return with help command printed if it does not.
///
/// # Example usage
/// ```Rust
/// // This is the same
/// arg_len!(args.len(), 2, macros::HELP_MSG);
/// // to
/// if args.len() < 2 {
///     println!("{}", macros::HELP_MSG);
///     return;
/// }
/// ```
#[macro_export]
macro_rules! arg_len {
    ($arg_len:expr, $min:expr, $msg:expr) => {
        if $arg_len < $min {
            println!("{}", $msg);
            return;
        }
    };
}
