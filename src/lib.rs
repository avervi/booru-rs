mod error;

use async_trait::async_trait;

pub use error::*;

#[async_trait]
pub trait Booru {
    /// The type of the posts, this can vary booru to booru depending on implementation.
    type Post;

    /// Get the posts on the booru for the given tags on the provided page.
    async fn get_posts(
        &self,
        tags: &[&str],
        page: usize,
    ) -> Result<Vec<Self::Post>, Box<dyn std::error::Error>>;
}
