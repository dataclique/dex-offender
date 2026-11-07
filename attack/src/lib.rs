pub mod ethereum;
pub mod hints;
pub mod solana;
#[cfg(feature = "sui")]
pub mod sui;

/// Print this level's upstream hint inside an `Exploit::attack`
/// implementation. Nothing is shown unless the macro is called; levels
/// without a registered hint say so.
#[macro_export]
macro_rules! hint {
    () => {
        $crate::hints::print_for(file!())
    };
}
