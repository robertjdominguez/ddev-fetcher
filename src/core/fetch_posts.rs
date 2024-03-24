use dotenv::dotenv;
use reqwest::{Client, Error};
use serde_json::{json, Value};
use std::env;

/*
* We'll start with a function that can make a fetch for all the posts I have on the CMS.
*/
pub async fn fetch_posts() -> Result<Value, Error> {
    dotenv().ok();
    let url = env::var("CMS_URL").expect("CMS_URL must be set");

    let graphql_query = r#"
        
query AllPostsQuery {
  posts {
    title
    slug
    publishedAt
    image {
      url
    }
    hook
    body
  }
}
"#;

    let client = Client::new();

    let request_body = json!({ "query": graphql_query });

    let response = client.post(url).json(&request_body).send().await?;

    let json_response: Value = response.json().await?;

    Ok(json_response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    pub async fn test_fetch_posts_real() {
        let result = fetch_posts().await;

        assert!(result.is_ok());
    }
}
