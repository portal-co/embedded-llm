#![no_std]

use core::ops::Deref;

use embedded_io::ErrorType;
use futures_core::Stream;
pub trait LLMMut: ErrorType {
    fn start(&mut self, system: &str)
    -> Result<impl LLMInstance<Error = Self::Error>, Self::Error>;
}
pub trait LLMRef: ErrorType {
    fn start(&self, system: &str) -> Result<impl LLMInstance<Error = Self::Error>, Self::Error>;
}
pub trait LLMInstance: ErrorType {
    fn send(
        &mut self,
        user: impl Iterator<Item: Deref<Target = str>>,
    ) -> Result<impl embedded_io::Read<Error = Self::Error>, Self::Error>;
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
    async fn send(
        &mut self,
        user: impl Stream<Item: Deref<Target = str>>,
    ) -> Result<impl embedded_io_async::Read<Error = Self::Error>, Self::Error>;
}
