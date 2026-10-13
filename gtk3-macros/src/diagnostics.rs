//! Emit ordinary compiler errors without an unmaintained proc-macro runtime.
use proc_macro2::TokenStream;

pub(crate) trait ErrorOutput {
    fn from_error(error: syn::Error) -> Self;
}
impl ErrorOutput for TokenStream {
    fn from_error(error: syn::Error) -> Self { error.into_compile_error() }
}
impl ErrorOutput for proc_macro::TokenStream {
    fn from_error(error: syn::Error) -> Self { error.into_compile_error().into() }
}
impl<T> ErrorOutput for syn::Result<T> {
    fn from_error(error: syn::Error) -> Self { Err(error) }
}
pub(crate) fn output<T: ErrorOutput>(error: syn::Error) -> T { T::from_error(error) }

macro_rules! diagnostic_message {
    ($message:expr) => { $message.to_string() };
    ($format:literal, $($args:tt)*) => { format!($format, $($args)*) };
}
macro_rules! abort_call_site {
    ($($message:tt)*) => { return crate::diagnostics::output(syn::Error::new(proc_macro2::Span::call_site(), diagnostic_message!($($message)*))) };
}
macro_rules! abort_span {
    ($span:expr, $($message:tt)*) => { return crate::diagnostics::output(syn::Error::new($span, diagnostic_message!($($message)*))) };
}
