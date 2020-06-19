mod example;

#[async_trait::async_trait]
pub trait Test {
    async fn example(&self);
}