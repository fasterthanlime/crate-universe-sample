use std::error::Error;

use async_trait::async_trait;

pub fn get_stuff() -> String {
    "stuff!".into()
}

#[async_trait]
pub trait SomeAsyncTrait {
    async fn do_async_stuff(&self) -> Result<String, Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
