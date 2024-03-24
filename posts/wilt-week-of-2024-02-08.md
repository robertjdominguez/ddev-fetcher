---
title: "WILT: Week of 2/8/2024"
hook: "WILT — A dive and recap into what I learned this week. This is part of a weekly series that is summarized via daily reflections and compiled by ChatGPT 🚀"
slug: wilt-week-of-2024-02-08
created_at: 2024-02-08T20:34:09.356493+00:00
image: https://media.graphassets.com/mZuL9rwaQHOtlFux5Kb3
---

# WILT: Week of 2/8/2024

# WILT: Reflecting on a Week in the Tech Trenches

Programming and tech don't just occupy my 9-5; these are the realms I frolic in, wade through, sometimes trudge, and occasionally face-plant on. But it's all in a day's work. This week, as always, was a testament to the continuous learning and adapting that we in the software field have signed up for—willingly or otherwise.

## Dipping Toes into Deno

Recently, I caught myself peeking at something shiny and new—Deno. For those out of the loop, Deno is akin to Node.js, created by the same person but with some modern twists. Specifically, it enforces secure defaults, supports TypeScript out of the box, and aims to provide a more robust standard library.

```javascript
// A simple Deno server might look like this:
import { serve } from https://deno.land/std@0.107.0/http/server.ts;
const server = serve({ port: 8000 });
console.log(http://localhost:8000/);
for await (const req of server) {
  req.respond({ body: Hello World
 });
}
```

Now that's clean! You'd think I'd jump at the chance to get more involved, and I did – for about five minutes. I landed in the comfy chair of familiarity with Node when building out a Discord interface for our docs bot. Learning curves can be fun, but when you're on the clock and already tangling with Discord's API, you ditch the learning for docking and delivery speed. Still, Deno's sitting there, looking at me with puppy eyes—I'll be back.

## If You Build It, Document It!

I ran headfirst into a wall of realization: internal documentation and testing are worth their weight in gold, or maybe even Bitcoin. Hammering away at code and pulling off a solo act is fine—until you need to bring others into your project. Then, your lack of comments and tests become the unwashed dishes of your development kitchen.

Picture this: you've crafted a nifty little function, maybe something that computes the number of hours until the next coffee break. Without documentation or tests, it's like a beautiful, enigmatic language that only you understand. Now, what's so useful about that?

```javascript
// Without comments, who knows what this does at a glance?
function computeCoffeeBreak(hoursNow, breakHour) {
  return ((breakHour - hoursNow + 24) % 24);
}
```

So, I preach: write for the next poor soul who'll inherit your code—write as if it were me, and trust me, I'd thank you solemnly with every line I don't have to decipher.

## DevOps Woes and Digital Ocean's CPU Overload

I sometimes play make-believe as a DevOps engineer—it's all roleplay until a server starts weeping CPU tears. Digital Ocean droplets have been my sandbox to construct and deconstruct digital habitats. However, I've watched this week as one of these virtual sandcastles buckled under the strain of building a Strapi instance—ripping through CPU like a hungry caterpillar through leaves.

Is it the droplet's fault, or is Strapi just a heavyweight? It's a reminder that not all clouds are cumulus fluff—some are laden with downpours. I'm not about to point fingers, but let's say that on a scale of instant coffee to watching paint dry, this was an entire Bob Ross painting series.

```bash
# Watching htop is like staring into the abyss, watching the CPU percentage climb
htop
```

One might suggest beefier hardware might be the cure, but therein lies the rub of cloud services—you pay for the convenience and simplicity, not always the brawn. Back to the drawing board, perhaps with a side of resource optimization?

There you have it—a snapshot of the productivity, puzzles, and pitfalls from a typical week in the life of a software engineer and educator. Until the next WILT, keep your humor dry, your comments clear, and your servers cool.
