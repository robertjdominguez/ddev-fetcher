use crate::entities::Post;
use std::fs::File;
use std::io::{self, Write};

/*
* This function creates a new post based on inputs. We'll then take the output of this and create a
* new markdown file with frontmatter at the top.
*/
pub fn create_post(
    title: String,
    body: String,
    created_at: String,
    image: String,
    hook: String,
    slug: String,
) -> Post {
    Post {
        id: 0,
        title,
        body,
        created_at,
        image,
        hook,
        slug,
    }
}

/*
* This function can be used to create a markdown file with frontmatter for any Post object created
* using the create_post function.
*/
pub fn create_markdown_file(post: &Post) -> io::Result<()> {
    let filename = format!("posts/{}.md", post.slug);
    let mut new_file = File::create(filename)?;

    let post_template = format!(
        r#"---
title: "{title}"
hook: "{hook}"
slug: {slug}
created_at: {created_at}
image: {image}
---

# {title}

{body}
"#,
        title = post.title,
        hook = post.hook,
        slug = post.slug,
        created_at = post.created_at,
        image = post.image,
        body = post.body
    );

    write!(new_file, "{}", post_template)?;

    Ok(())
}

/*
* Need a little helper function that can take a string and escape any "" to '' to not break
* frontmatter formatting for markdown.
*/
pub fn escape_quotation_marks_in_string(raw_string: &str) -> String {
    return raw_string.replace("\"", "'");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_create_post() {
        let test_create_post_text = create_post(
            "A New Post".to_string(),
            "It was the best of times, it was the worst of times...\nNot really, dawg.".to_string(),
            "2024-01-01".to_string(),
            "https://image.com/image/image.png".to_string(),
            "This is a great post...click me!".to_string(),
            "magic-slug-bitch".to_string(),
        );
        assert_eq!(test_create_post_text.title, "A New Post");
        assert_eq!(
            test_create_post_text.body,
            "It was the best of times, it was the worst of times...\nNot really, dawg."
        );
    }

    #[test]
    pub fn test_create_markdown_file() {
        let sample_post = create_post(
            "My First Post".to_string(),
            "This is the post.".to_string(),
            "2024-02-25".to_string(),
            "https://example.com/image/image.png".to_string(),
            "Read this".to_string(),
            "my-first-post".to_string(),
        );

        let new_file = create_markdown_file(&sample_post);
        assert!(new_file.is_ok(), "Failed to create markdown file.");
    }

    #[test]
    pub fn test_quotation_replacement() {
        let raw = "I like to \"party\"";

        let new = escape_quotation_marks_in_string(raw);

        assert_eq!(new, "I like to 'party'");
    }
}
