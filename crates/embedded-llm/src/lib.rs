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
