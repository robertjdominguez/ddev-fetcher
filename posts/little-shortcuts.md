---
title: "Little Shortcuts, Big Timesavers"
hook: "Snippets, macros, aliases, automation...all of these things involve a little work up front to make life easier down the line."
slug: little-shortcuts
created_at: 2023-02-10T03:08:36.428068+00:00
image: https://media.graphassets.com/sjOyf0fqTdiPPOL4ZyDp
---

# Little Shortcuts, Big Timesavers

### Productivity via code

More and more, I find myself creating solutions that remove friction from my day-to-day life. These solutions take a lot of forms, but - more often than not - they're little pieces of code that automate a specific task. In the past, I've reached for Node.js to write scripts...but I'm quickly realizing how powerful and dependency-free Bash scripts can be.

### In the wild

One task I start and finish my day with - every day - is reviewing PRs. As an engineer on the docs team, there's no shortage of updates or new documentation that we need to merge. We conveniently utilize preview builds of each PR to quickly examine the proposed changes, rendered as they would be in production. These previews all follow the same pattern:

```html
https://pr-<PR_NUMBER>---super_secret_preview_url.app/docs/latest/index/
```

In meetings, we'll often reference PRs by number and inelegantly search through GitHub to find preview links to these builds. I got tired of doing that. Since my terminal is *always* open, I decided to write a function where I can quickly reference the PR and automatically open it in Chrome.

### Writing a Bash function

You can declare a Bash function by writing `function` followed by the 'command' you want to run. Since this is related to PR preview builds, I called my function `pre` to keep it short and handy. At its core, the function takes this shape:

```bash
function pre {
  # actions / commands we'll automate via running pre
}
```

Anything in between the curly brackets is what the script will run; this follows normal Bash syntax, so - if we want - we can run multiple commands. However, we only need to do one thing: take in an argument and stick it in a URL that our browser will open. The complete function looks like this:

```bash
function pre {
  open https://pr-$1---super_secret_preview_url.app/docs/latest/index/
}
```
When passed a URL, the `open` command will open your machine's default browser and navigate to the URL. The `$1` is a placeholder for the first argument passed to the `pre` function. So, if we want to see the preview build for PR #1234, we can run `pre 1234` and our function will open `https://pr-1234---super_secret_preview_url.app/docs/latest/index/` in our browser 🚀

![pre_sample.gif](https://media.graphassets.com/ondbRxC1QWmUAOPjaUaW)

### Making a function reusable

This function is great...but what happens when we end our terminal's session? Well, that function is ephemeral. It's gone. However, depending on your shell, you can add functions and aliases to your configuration file. This is typically either `.bashrc` or `.zshrc`. The quickest way of modifying this is running `nano <filename>` from your root directory. Within this file, you can add aliases for shortcutting commands and any functions you want to reuse later 🔥

