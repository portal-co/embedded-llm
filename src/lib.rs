#![no_std]

use core::ops::Deref;

use futures_core::Stream;
pub trait LLMMut{
    fn start(&mut self, system: &str) -> impl LLMInstance;
}
pub trait LLMRef{
    fn start(&self, system: &str) -> impl LLMInstance;
}
pub trait LLMInstance{
    fn send(&mut self, user: impl Iterator<Item: Deref<Target = str>>) -> impl embedded_io::Read;
}
pub trait AsyncLLMMut{
    async fn start(&mut self, system: &str) -> impl AsyncLLMInstance;
}
pub trait AsyncLLMRef{
    async fn start(&self, system: &str) -> impl AsyncLLMInstance;
}
pub trait AsyncLLMInstance{
    async fn send(&mut self, user: impl Stream<Item: Deref<Target = str>>) -> impl embedded_io_async::Read;
}