// pub fn set_panic_hook() {
//     // When the `console_error_panic_hook` feature is enabled, we can call the
//     // `set_panic_hook` function at least once during initialization, and then
//     // we will get better error messages if our code ever panics.
//     //
//     // For more details see
//     // https://github.com/rustwasm/console_error_panic_hook#readme
//     #[cfg(feature = "console_error_panic_hook")]
//     console_error_panic_hook::set_once();
// }
// // #[inline]
// pub fn unwrap_option_abort<T>(o: Option<T>) -> T {
//     use std::process;
//     match o {
//         Some(t) => t,
//         None => process::abort(),
//     }
// }
#[inline]
pub fn unwrap_result_abort<T, U>(o: Result<T, U>) -> T {
    use std::process;
    match o {
        Ok(t) => t,
        Err(_) => process::abort(),
    }
}
