mod core;
mod entities;

use core::{fetch_posts, create_post, escape_quotation_marks_in_string, create_markdown_file};

#[tokio::main]
async fn main() {
    /* We'll generate files for each of the posts that's returned
     * from the fetch_posts() call using a match. A match allows us to have two options for what's
     * returned from the function: either Ok(a_variable), wherein we can then use the a_variable as
     * the encapsulation of what's returned or Err(err) in which we return an error.
     */
    match fetch_posts().await {
        Ok(posts_as_json) => {
            /* First, we'll create a Post instance for each post by iterating over posts
             * and using Some(posts) to filter out for None values. If a None value exists, shit happened.
             * Otherwise, we have postss that can be turned into an array and we can then loop over them.
             */
            if let Some(posts) = posts_as_json["data"]["posts"].as_array() {
                for post in posts {
                    /* We'll deserialize the JSON and get the different values we need. However,
                     * Rust forces us to handle errors up front: as an example, we're taking a key
                     * of "title" from the post object in JSON and turning it into a string slice
                     * (reference) using as_str() » this returns a &str that we then need to either
                     * unwrap (which means get the value) or return a default (in this case, an
                     * empty string?). Finally, we have to convert this into a String using
                     * to_string() so that it's what our new_post() function expects when generating
                     * a post.
                     */
                    let title = post["title"].as_str().unwrap_or_default().to_string();
                    let body = post["body"].as_str().unwrap_or_default().to_string();
                    let created_at = post["publishedAt"].as_str().unwrap_or_default().to_string();
                    let image_url = post["image"]["url"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string();
                    let hook = post["hook"].as_str().unwrap_or_default().to_string();
                    let slug = post["slug"].as_str().unwrap_or_default().to_string();
                    let new_post = create_post(
                        title,
                        body,
                        created_at,
                        image_url,
                        escape_quotation_marks_in_string(&hook),
                        slug,
                    );

                    // Then, we'll need to create a new markdown file using the new_post
                    let _new_file = create_markdown_file(&new_post);

                    println!("✅ Created new file for post titled: {}", &new_post.title);
                }
            } else {
                println!("💩");
            }
        }
        Err(err) => {
            eprintln!("Failed to create markdown file: {}", err);
        }
    }
}
