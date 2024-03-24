---
title: "WILT: Week of 1/12/2024"
hook: "WILT — A dive and recap into what I learned this week. This is part of a weekly series that is summarized via daily reflections and compiled by ChatGPT 🚀"
slug: wilt-week-of-2024-01-12
created_at: 2024-01-12T19:53:12.545294+00:00
image: https://media.graphassets.com/mZuL9rwaQHOtlFux5Kb3
---

# WILT: Week of 1/12/2024

# WILT Week of [Insert Date]

## The Mock(ery) of Perfection with Jest

Testing with Jest has been like a nerdy kind of delight for me. Getting those sweet, sweet greens on the coverage report is the closest I've come to feeling like I've won a coding Emmy. But let's not sugarcoat it; I know that 100% coverage is about as reassuring as a trust me from that guy selling genuine Rolexes from his trunk. It gives you this inflated sense of accomplishment, but deep down, you know it might just be fluff.

For instance, in testing the filesystem interactions while working on the content migration utility:

```javascript
const fs = require('fs');
jest.mock('fs');

test('writes a file', () => {
  const writeFileMock = jest.spyOn(fs, 'writeFile').mockImplementation((path, data, cb) => cb(null));

  // imagine someFunction uses fs.writeFile internally
  someFunction('path/to/file', 'data', () => {
    expect(writeFileMock).toHaveBeenCalledWith('path/to/file', 'data', expect.any(Function));
  });
});
```

In only two days, using test-driven development, the core functionality was alive and kicking—proving once again, with a sprinkle of cynicism, that TDD might just be slightly better than adding prints and hoping for the best.

## Architecting Content Migration: Hasura's Supergraph Journey

Embarking on a content migration feels kind of like organizing a closet. You start out thinking, This is a mess, but as you go, you find that one sock you thought the dryer ate, and things start looking up.

Utilizing the concept of a data supergraph, which we're pushing with Hasura V3, is like finding a Swiss Army knife in that same closet. It's making me consider how our single responsibility principle (SRP) can transcend code and reflect in our very team structure—each subgraph, a dedicated team. It's separating concerns like an adept chef does with egg yolks and whites—neat and efficient.

```graphql
type Query {
  # Each team focuses on their domain, leading to a 
  # clear cut separation of concerns in the supergraph
  books: [Book]
  authors: [Author]
}
```

If codebases were like kitchen duties, setting up microservices like this ensures one team isn’t stuck doing all the dishes.

## AI Meetups and Token Budget Overruns—A Day in the Life

I dove into an AI meetup that was the conversational equivalent of a rollercoaster—it went high-level, low-level, and all around the AI stack. I can't wait for the engineering sub-group so I can geek out even more.

On another note, while integrating the OpenAI API, I got a little too carried away with the size of the files I was processing. And just like that relative who fills their plate too high at the buffet, I overran the token budget. A lesson in restraint: maybe don't try to process War and Peace in one go.

```python
# Overstepping the token limit - not fun
try:
    response = openai.Completion.create(
        engine=davinci,
        prompt=Translate the following English text to French: '{}',
        max_tokens=100,  # Should've paid attention to this
    )
except openai.error.OpenAIError as e:
    print(fRan out of tokens: {str(e)})
```

## Open Source GitHub Action: Docs Assertion Tester

Ending the week on a high note: I've unleashed an open source GitHub action onto the unsuspecting masses. The 'docs-assertion-tester'—a testament to my obsession with TDD and short feedback loops. Now any doc-writer can get their prose unit tested by the merciless bots of GitHub CI.

Here, let me paint you a terminal-colored picture: you push some documentation, and on the other end, assertions are pitted against your changes. In moments, you get a verdict—passed or smacked down for further revision. It's like instant messaging with the judgmental grammarian in your head.

```yaml
# Sample GitHub Action Workflow
name: Documentation Test
on: pull_request

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - name: Check Documentation Assertions
      uses: hasura/docs-assertion-tester@v1
      with:
        github-token: ${{ secrets.GITHUB_TOKEN }}
        openai-api-key: ${{ secrets.OPENAI_API_KEY }}
```

Vercel NCC turned the whole operation into a slick one-file show, priceless for when you’ve got vendetta against `node_modules`. But I’ll stand by my opinion, GitHub Actions are as fun to test as stepping on a Lego—barefoot.

In essence, it’s been another week where sarcasm and software engineering weave a rich tapestry of lessons learned, punctuated by the realities that await outside the glowing embrace of my IDE. Until next week, keep your mocks close, your tokens closer, and never trust a coverage report at face value.
