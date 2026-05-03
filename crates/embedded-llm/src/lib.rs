#![no_std]
#![allow(async_fn_in_trait)]

use embedded_io::ErrorType;
use futures_core::Stream;

/// Message type/role for conversation messages.
///
/// Used in tuples `(MessageType, &str)` to represent a message with its role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    System,
    User,
    Assistant,
    Tool,
}

pub trait LLMMut: ErrorType {
    fn start(&mut self, system: &str)
    -> Result<impl LLMInstance<Error = Self::Error>, Self::Error>;
}
pub trait LLMRef: ErrorType {
    fn start(&self, system: &str) -> Result<impl LLMInstance<Error = Self::Error>, Self::Error>;
}
pub trait LLMInstance: ErrorType {
    fn send<'a>(
        &mut self,
        messages: impl Iterator<Item = (MessageType, &'a str)>,
    ) -> Result<(MessageType, impl embedded_io::Read<Error = Self::Error>), Self::Error>;
}
pub trait AsyncLLMMut: ErrorType {
    async fn start(
        &mut self,
        system: &str,
    ) -> Result<impl AsyncLLMInstance<Error = Self::Error>, Self::Error>;
}
pub trait AsyncLLMRef: ErrorType {
    async fn start(
        &self,
        system: &str,
    ) -> Result<impl AsyncLLMInstance<Error = Self::Error>, Self::Error>;
}
pub trait AsyncLLMInstance: ErrorType {
    async fn send<'a>(
        &mut self,
        messages: impl Stream<Item = (MessageType, &'a str)>,
    ) -> Result<(MessageType, impl embedded_io_async::Read<Error = Self::Error>), Self::Error>;
}

/// Dyn-safe single-turn interface: start a session, send one user message, get a `String` back.
///
/// Enabled with the `alloc` feature. Unlike [`AsyncLLMInstance`], this trait is object-safe
/// and can be stored as `Box<dyn DynAsyncLLMInstance>`.
#[cfg(feature = "alloc")]
pub trait DynAsyncLLMInstance: Send {
    fn send_one<'a>(
        &'a mut self,
        user: &'a str,
    ) -> core::pin::Pin<
        alloc::boxed::Box<
            dyn core::future::Future<
                    Output = Result<
                        alloc::string::String,
                        alloc::boxed::Box<dyn core::error::Error + Send + Sync>,
                    >,
                > + Send
                + 'a,
        >,
    >;
}

/// Dyn-safe backend: object-safe version of [`AsyncLLMRef`].
///
/// Enabled with the `alloc` feature. Implementors can be stored as `Box<dyn DynAsyncLLMRef>`.
#[cfg(feature = "alloc")]
pub trait DynAsyncLLMRef: Send + Sync {
    fn start_boxed<'a>(
        &'a self,
        system: &'a str,
    ) -> core::pin::Pin<
        alloc::boxed::Box<
            dyn core::future::Future<
                    Output = Result<
                        alloc::boxed::Box<dyn DynAsyncLLMInstance>,
                        alloc::boxed::Box<dyn core::error::Error + Send + Sync>,
                    >,
                > + Send
                + 'a,
        >,
    >;
}

#[cfg(feature = "alloc")]
extern crate alloc;
