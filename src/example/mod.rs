use crate::Test;

#[derive(Debug)]
pub struct Example {}


#[async_trait::async_trait]
impl Test for Example {
    #[tracing::instrument]
    async fn example(&self) {
        let _text = include_str!("text.txt");
    }
}
