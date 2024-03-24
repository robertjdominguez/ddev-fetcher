---
title: "WILT: Week of 2/16/2024"
hook: "WILT — A dive and recap into what I learned this week. This is part of a weekly series that is summarized via daily reflections and compiled by ChatGPT 🚀"
slug: wilt-week-of-2024-02-16
created_at: 2024-02-16T19:07:36.262598+00:00
image: https://media.graphassets.com/mZuL9rwaQHOtlFux5Kb3
---

# WILT: Week of 2/16/2024

# WILT: Software Alchemy and CI/CD Incantations

In the realm of software engineering and in my quest to indoctrinate fresh minds into our dark arts, I stumbled upon some noteworthy incantations and amulets—or as the layperson might call them, development tools and practices. Here's what I've conjured up this week:

## The Magic of GitHub Actions

Ah, GitHub Actions... both the bane of my existence and my savior. They are the arcane scrolls for automation within my repositories, embodying scripts and spells that somehow bridge the gap from code to deployment. The real sorcery here is that they can turn mundane local scripts into automated, world-shifting processes with just a push of code — talk about modern-day alchemy!

For example, once upon a time (aka this week), I conjured this simple spell:

```yaml
name: Node.js CI

on:
  push:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v2
    - name: Use Node.js
      uses: actions/setup-node@v1
      with:
        node-version: '14'
    - name: Install dependencies
      run: npm install
    - name: Run tests
      run: npm run test
```

With this potion, I was able to automate testing for every commit pushed to the main branch. Simple, but powerful; the essence of every great spell.

## The Tortoise and the CI Pipeline

On the darker side, awaiting the feedback loop of GitHub Actions felt akin to watching water boil in a cauldron—except less exciting. Debugging is like sending a raven and waiting for it to return with news. It's slow, but—spoiler alert—the bird eventually comes back. Now, I've not yet fallen to the dark side, but I'm fairly certain a proper understanding of how to create and handle tarballs could have saved me from aging another decade this week.

Nevertheless, during my odyssey through CI/CD (Continuous Integration and Continuous Deployment/Delivery), a breakthrough materialized. The pipeline POC (Proof of Concept) for our e-learning platform, something that looked more like a tangled web of yarn than a well-oiled machine, finally achieved its true form. It's a system where:

1. Code commits trigger a dance of automated testing and builds.
2. Artifacts are created, spellbooks are updated, and tests are automated runes ensuring stability.
3. Deployment is not an arduous trek but a mere wave of a wand, carrying updates through mystical ether to servers afar.

## Strapi: The Necronomicon of Content

Strapi has become somewhat of a necromancer's grimoire, filled with commands to bend content and data to my will. One particular spell—`npm install strapi`—allows me to not only summon the CMS (Content Management System) but to manipulate its life force through tokens and endpoints that can vary by environment. Sorcery, I tell you!

Furthermore, the utopian dream of seamless content migration seemed one step closer with the promises of `strapi transfer`. Though yet to be proven outside of its theoretical confines, the prospect is enticing. The next chapter of my journey will be to harness this power, channeling it so that it may transport content between realms (or, you know, development and production environments) without the messiness of human intervention.

## Closing Scrolls

So, from the voodoo of pipelines to the almost effortless wonders of Strapi-based content management, it's been a week where the shadows of ignorance have retreated ever so slightly. And with Mardis Gras upon us, the revelry may be high but so is the satisfaction of a job well done. Or at least, a job... done.

Until next week, may your code compile and your tests pass on the first incantation.

- Rob Dominguez, software engineer, reluctant mage, and beacon of humility.
