use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder};

use crate::{Booru, BooruError};

/// A client for [Gelbooru](https://gelbooru.com).
pub struct Gelbooru {
    http_client: Client,
    auth: Option<(String, usize)>,
}

impl Gelbooru {
    /// Creates a new builder to construct the client instance.
    pub fn builder() -> GelbooruBuilder {
        GelbooruBuilder::default()
    }

    fn set_base_query(
        &self,
        mut request_builder: RequestBuilder,
        api_page: &'static str,
    ) -> RequestBuilder {
        request_builder =
            request_builder.query(&[("page", api_page), ("s", "post"), ("q", "index"), ("json", "1")]);

        if let Some((ref api_key, user_id)) = self.auth {
            request_builder = request_builder.query(&[("api_key", api_key.clone())]);
            request_builder = request_builder.query(&[("user_id", user_id)]);
        };

        request_builder
    }
}
#[async_trait]
impl Booru for Gelbooru {
    type Post = GelbooruPost;

    async fn get_posts(
        &self,
        tags: &[&str],
        page: usize,
        limit: usize,
    ) -> Result<Vec<Self::Post>, BooruError> {
        // The URL encoding gets a bit weird here, so we'll do this the hacky way
        let tags = tags.to_vec().join("+");
        let url = format!("https://gelbooru.com/index.php?tags={}", tags);

        let request_builder = self.http_client.get(&url);
        let request_builder = self
            .set_base_query(request_builder, "dapi")
            .query(&[("pid", page.to_string())])
            .query(&[("limit", limit.to_string())]);

        let response = request_builder.send().await?;
        Ok(response.json().await?)
    }
}

/// A builder used to create instances of a [`Gelbooru`](crate::gelbooru::Gelbooru).
#[derive(Debug, Clone, Default)]
pub struct GelbooruBuilder {
    http_client: Option<Client>,
    api_key: Option<String>,
    user_id: Option<usize>,
}

impl GelbooruBuilder {
    /// Sets the HTTP client that will be used to make requests to gelbooru,
    /// useful if you want to use a proxy.
    pub fn http_client(mut self, http_client: Client) -> Self {
        self.http_client = Some(http_client);
        self
    }

    /// Sets the authentication for gelbooru to use for any special queries.
    pub fn auth(mut self, api_key: impl ToString, user_id: usize) -> Self {
        self.api_key = Some(api_key.to_string());
        self.user_id = Some(user_id);
        self
    }

    // TODO: Make this return an error?
    pub fn build(self) -> Gelbooru {
        let auth = self
            .api_key
            .as_ref()
            .and_then(|api_key| self.user_id.map(|user_id| (api_key.clone(), user_id)));
        Gelbooru {
            http_client: self.http_client.unwrap_or_else(|| Client::new()),
            auth,
        }
    }
}

/// Representation of a Gelbooru post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GelbooruPost {
    pub source: String,
    pub directory: String,
    pub hash: String,
    pub height: i64,
    pub id: Option<i64>,
    pub image: String,
    pub change: i64,
    pub owner: String,
    pub parent_id: Option<serde_json::Value>,
    pub rating: GelbooruRating,
    pub sample: i64,
    pub sample_height: i64,
    pub sample_width: i64,
    pub score: i64,
    pub tags: String,
    pub width: i64,
    pub file_url: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GelbooruRating {
    #[serde(rename = "e")]
    Explicit,
    #[serde(rename = "q")]
    Questionable,
    #[serde(rename = "s")]
    Safe,
}

#[cfg(test)]
mod tests {
    use super::Gelbooru;
    use crate::Booru;

    #[tokio::test]
    async fn get_posts() {
        let gelbooru = Gelbooru::builder().build();
        let posts = gelbooru
            .get_posts(&["id:>=0", "id:<=10"], 0, 10)
            .await
            .unwrap();

        assert_eq!(posts.len(), 9);

        let first_id = posts.get(0).unwrap().id;
        assert_eq!(first_id, Some(10));
    }
}
