---
title: "WILT: Week of 1/5/2024"
hook: "WILT — A dive and recap into what I learned this week. This is part of a weekly series that is summarized via daily reflections and compiled by ChatGPT 🚀"
slug: wilt-week-of-2024-01-05
created_at: 2024-01-07T17:01:22.246427+00:00
image: https://media.graphassets.com/mZuL9rwaQHOtlFux5Kb3
---

# WILT: Week of 1/5/2024

# WILT: January 4th

## Embracing Domain-Driven Design with Hasura v3

In my constant crusade to outsmart the mundane, I've been diving headfirst into the land of Hasura v3. It's like finding a toolkit under the Christmas tree - a set of tools designed to make life easier, but with an instruction manual that's somewhat... evasive. The key takeaway, though, is that Hasura v3 is less about piecing together a data jigsaw puzzle and more about creating a coherent story for different teams in an organization. Think of it like turning a cubist painting into a narrative; each team – User Experience, Fulfillment Services, Payment Processing – becomes the protagonist of their own chapter, armed with their specific data and operations.

### Bridging the Gap with TypeScript Connectors

In the often-bewildering world of APIs, I stumbled upon a gem: the TypeScript connector in Hasura v3. This nifty feature lets code poets like us script functions in our beloved TypeScript, influencing the tale within our GraphQL API.

Suppose you have a team that's as obsessed with hospitality as a five-star concierge. They want every user to receive a unique greeting. Old school? Maybe. But with TypeScript functions, you can craft a greeting algorithm that feels more like a handshake than a form letter. Imagine a function like this (simplified for sanity's sake):

```typescript
function determineGreeting(userPreference: string): string {
switch(userPreference) {
    case 'formal':
      return 'Good day to you, esteemed user.';
    case 'casual':
      return 'Hey there, friend-o!';
    default:
      return 'Hello, valued visitor!';
  }
}
```

This is the beauty of it all. A front-end developer can now include this personalized greeting directly in their query. The backend does the heavy lifting, and the front-end remains blissfully unburdened—an elegant symphony of separation of concerns.

### The Unanswered Queries of Function-Model Relationships

As per my daily debrief, my brain is itching with intrigue about the relationship dynamics between these TypeScript functions and the data models. There's a bit of a foggy area there. It's like trying to predict the weather – sometimes you've got clear skies, and sometimes a tornado drops in to say \"hi\". I’m pondering how deeply these functions can intertwine with the tables and data that describe the structure of each team's domain.

## Reflecting on Software Craftsmanship and TDD

It wasn't all GraphQL and team-based architectures. My grey matter got a good jog with the philosophy laid down in \"Clean Coder\" by Uncle Bob Martin. A shift away from pure code-smithing, the tome nudges us developers to ascend from mere keyboard warriors to knights of the round (office) table – professionals of the digital age.

### Test-Driven Development (TDD): The Red-Green Refactoring Mantra

Here comes an acronym that promises more than productivity: TDD. I'm not keen on trends that don't deliver, but TDD's tune was catchy enough to stop me in my tracks. It's like learning a new dance – you step on a few toes (the red), hit a groove (the green), and then jazz things up (refactor). As I set up a new TypeScript project and cuddled up with Jest for my hands-on TDD, I could taste the potential for cleaner, more robust code. Here's a morsel to chew on:

```typescript
describe('greeting function', () => {
  it('returns a formal greeting', () => {
    expect(determineGreeting('formal')).toBe('Good day to you, esteemed user.');
  });

  it('returns a casual greeting', () => {
    expect(determineGreeting('casual')).toBe('Hey there, friend-o!');
  });

  it('returns a default greeting', () => {
    expect(determineGreeting('')).toBe('Hello, valued visitor!');
  });
});
```

This is me turning software development into a feedback loop of quick learning and adapting, where tests drive the code structure and ultimately lead to more maintainable, happier code. A tad over-optimistic? Perhaps. But who says cynicism can't come with a side of code quality?

### Closing WILTs and Coding Voids

In the spirit of 'What I Learned Today', I took a micro-moment each day to log my digital discoveries and mental facepalms. I'm aiming for a mixtape of meaningful reflections and, let's be honest, some trite trivia – because consistency is the hobgoblin of little minds, and my mind is a veritable goblin den.

So, to anyone looking to spice up their procedure: embrace the chaos of learning, wield red-green refactoring like a lightsaber, and maybe, just maybe, accept that you may end up a bit more Jedi master than scruffy-looking nerf herder. But as always, take that with a grain of salt - optimism is great and all, but let's not sugarcoat the struggle that is software development.
