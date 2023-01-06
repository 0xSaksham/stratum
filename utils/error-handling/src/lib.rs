/// # Description
/// This macro handles errors inserting error handling logic for a given `Result<T, crate::error::Error<'a>>`
/// it is used by passing in a `Sender<crate::status::Status>` as the first parameter
/// and a `Result<T, crate::error::Error<'a>>` as the second parameter.
///
/// NOTE: There are 3 caveats to using this macro:
/// 1. can only be used within async functions since status needs to be send over async channel
/// 2. The macro must be used within a loop since it calls `continue` on error. If `unwraps/expects` are used within a function
///     without a lopo, you should make the function return a result and handle the result within a main loop
/// 3. The macromust be able to reference the user defined function `crate::status::handle_error(T, U);` where U is the output
///     of `e.into()`
///
/// # Example
/// ```
/// let (tx_status: Sender<Status>, rx_status) = async_channel::unbounded();
/// let variable = handle_result!(
///     tx_status,
///     type_.try_into()
/// );
/// ```
#[macro_export]
macro_rules! handle_result {
    ($sender:expr, $res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                // handle error
                crate::status::handle_error(&$sender, e.into()).await;
                continue;
            }
        }
    };
}
