---
title: "WILT: Week of 2/1/2024"
hook: "WILT — A dive and recap into what I learned this week. This is part of a weekly series that is summarized via daily reflections and compiled by ChatGPT 🚀"
slug: wilt-week-of-2024-02-02
created_at: 2024-02-02T00:54:03.857037+00:00
image: https://media.graphassets.com/mZuL9rwaQHOtlFux5Kb3
---

# WILT: Week of 2/1/2024

# WILT: Reflections on Refactoring, Automation, and API Interactions

Another week has flown by, sprinkled with moments of clarity and the occasional facepalm—such is the life of a software engineer who's as cynical as he is self-deprecating. Let's dive into this week's WILT and break down some of the nerdy nuggets I encountered.

## Embracing TypeScript's Power Through Octokit

My journey of refactoring the cherry-picking utility for Hasura DDN beta docs has been a testament to TypeScript's prowess. There's something magical about TypeScript and its type system. For instance, deciphering and anticipating the shape of objects using Octokit’s types turned the once arcane task of API manipulation into a remarkably pleasant endeavor. It's like having a cheat sheet for every exam — I'm starting to wonder if this is a coder's version of 'easy mode'.

### The Pitfalls of Pipe Operators and Armchair Problem Solving

In my previous implementation, the pipe operator felt like a hammer in search of a nail – it made the code work but lacked elegance. The result was a confusing mess, like trying to decipher my own handwriting. I had this aha! moment when I realized I could throw an error to simplify the error handling process. I refined my code, and here's a taste of what it looks like now:

```typescript
try {
  const result = await someAsyncFunction();
  // Do something with the result
} catch (error) {
  // Gracefully handle the error
}
```

This approach is as neat as my desk on a good day, which is a rarity, but let’s not go there.

## Automating the Tedious with GitHub Actions

Ah yes, automation—the programmer’s panacea. We’re finally implementing continuous deployment (CD) for the Hasura DDN documentation site. It’s like creating a conveyor belt made of code: PRs are approved, merged into staging, and then a GitHub Action takes its cue, sending everything off to production like well-behaved soldiers. No more manual cherry-picking—a task I'm convinced was conceived by someone who hates joy.

Here's the essence of automation scripted in a GitHub Action:

```yaml
name: Automate Cherry-picking

on:
  push:
    branches:
      - main

jobs:
  cherry-pick:
    runs-on: ubuntu-latest
    steps:
    - name: Check out the repo
      uses: actions/checkout@v2
      with:
        ref: ${{ github.head_ref }}
    # More steps for checking labels and cherry-picking commits...
```

Crafting this felt as satisfying as poking bubble wrap, which for the uninitiated, is the coder's version of a stress ball.

## Parsing OpenAI's JSON Mode with a Side of Irony

Time spent experimenting with JSON mode in the OpenAI API confirmed my hypothesis: it's undeniably efficient for our docs-assertion-tester, yet I’m stuck parsing strings into JSON. I chuckle at the irony; didn't we create machines to save us from the mundane? I digress.

Here’s an example of parsing the API response, which, while comical in its triviality, is necessary:

```javascript
const response = await openAiApiCall();
const parsedResponse = JSON.parse(response);
```

The silver lining came to light when I realized giving the API a one-shot example in the prompt led to better results, even if the process felt as roundabout as explaining recursion to a philosophy major.

My weekly musings may seem like an odd mix of complaints and celebrations, yet it’s in this contrast where growth happens — a lesson I learn every week. With that, I'll wrap up this week's WILT. Stay tuned for the next series of revelations and perhaps some more self-deprecating humor—after all, it’s hard to take oneself too seriously when your day is spent conversing with machines.
