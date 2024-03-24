---
title: "One Project to Rule Them All"
hook: "I've been re-reading The LOTR for the past few months and I am definitely in a Middle-Earth headspace...Recently, I completed my last large-scale project for the school I've worked for over the past decade: a single site for all of our student publications."
slug: lotr
created_at: 2022-05-20T22:15:57.840706+00:00
image: https://media.graphassets.com/lU2qSSpEQVi6eLfDkubg
---

# One Project to Rule Them All

## Scope
Over the past five years, I had the opportunity to build numerous services for The Altamont School. These projects increasingly grew in scope and complexity while simultaneously becoming simpler(🤞). From beginning with web scraping to identify patterns in student performance, to writing instant-runoff voting (IRV) algorithms for awards, to building several e-commerce sites, and, recently, a QR-fueled carpool application. It's been...a lot. To cap it all off, I took on "one" final build that was - in reality - *three* final builds.

Within the school, we have a series of student publications which primarily showcase writing and art. The scope of each of these publications varies: one is a school newspaper, another a scholarly journal, and finally a literary magazine.

In the past, I built a site using SvelteKit for our newspaper. I reached for Svelte for a few reasons. First, I was considering using it in my Upper School Web Design class; I wanted something that felt like semantic HTML but that also allowed students to utilize a JavaScript framework for building a real-world application. Second, I was just curious and wanted to give it a go.

I hoped to have other projects for my students so I could split them into teams. In my head, a team of 4-5 students would work through the entire design-development process with the student-editors and sponsors of each publication. The Web Design students would not only do the "hard" work of creating high-fidelity mock-ups and developing a functioning Jamstack site, but they'd also be totally immersed in an agency-like experience: they'd hold client meetings, do design reviews, user testing, the works!

What quickly became clear when working with my students this semester was that they would not have mastered the skills and learned the content to build a modern site before these new versions needed to be live. That was totally on me: I had high expectations and was a bit too ambitious for an introductory course. If I stayed at Altamont, I would undoubtedly have proposed a new *advanced* course for next year!

I was hesitant to take on a new project - let alone three - with only 10 weeks left in the school year. But, I saw an opportunity to solve some technical problems and really give our students an online presence to showcase their amazing work.

## Stack
I proposed a new, unified web app that acted as an umbrella for all three publications. Each would have its own dedicated, independent GraphCMS instance with access provided to the student-editors and sponsors. Each of these instances would use webhooks to trigger builds of a Next.js application hosted on Vercel. We would also configure our DNS records to create a subdomain under our school's site for a single, easy-to-navigate endpoint.

In theory, the structure looks like this:
![publications.png](https://media.graphassets.com/P1f5IJaRWy4CMbQ0pc03)

In practice, the basic Next.js directory structure looks like this:

```
├── pages
│   ├── acta-diurna
│   │   ├── index.js
│   │   ├── [slug].js
│   ├── beacon
│   │   ├── index.js
│   │   ├── [slug].js
│   ├── dragon
│   │   ├── index.js
│   │   ├── [slug].js
```

However, to make it iterative, I amended it to this by adding in a dynamic year route:

```
├── pages
│   ├── acta-diurna
│   │   ├── index.js
│   │   ├── [year]
│   │   |    ├── index.js
│   │   |    ├── [slug].js
│   ├── beacon
│   │   ├── index.js
│   │   ├── [year]
│   │   |    ├── index.js
│   │   |    ├── [slug].js
│   ├── dragon
│   │   ├── index.js
│   │   ├── [year]
│   │   |    ├── index.js
│   │   |    ├── [slug].js
```

This structure allows a user to visit each publication by following this template: `https://publications.altamontschool.org/dragon/2022` to visit a specific year. The content is loaded using `getStaticProps()` and the routes are generated using `getStaticPaths()`.

## Working with three clients simultaneously 

![giphy.gif](https://media.graphassets.com/SVYFUhe2R0vrxaYro6nA)

Working with a wide group of academics and creative professionals with many opinions was...a lot. There were lots of divergent ideas and differing desires for how visitors would interact with the content. Additionally, some wanted to showcase students' individual pieces whereas other wanted a "magazine" feel. To make it work, I ended up leaning pretty heavily on Framer Motion.

A critical piece of this plan working was allowing for all three publications to have their own unique identity reflective of their content. I wanted each to have its own personality that made it clear to a visitor that they were still within our little world, but also that they were in a place that felt like it was reflective of the publication's content.

While I implemented a design system that is reflective of the school's brand standards, I took liberty to incorporate some more playful UI aspects throughout different components on the literary site. Likewise, the newspaper's design is reflective of most modern news publications and the academic journal's appearance carries a bit of depth and gravity due to its heavier content.

## Lifecycle
Each year, a new set of student-editors take control of these projects. Depending on the publication, publishing happens at different times of the year. For example, the newspaper is a constant stream of articles as events happen. Alternatively, the literary and academic publications only publish once a year in May. Fortunately, with GraphCMS's easy-to-follow editing experience, non-technical users can take control as this project has officially been handed off.

## Parting thoughts
It feels really good to have pushed the last commit on this project. I hope it's used for years to come and allows the school to continue cataloguing and showcasing some amazing student work. As I said, it was my last project for The Altamont School. After nearly a decade on the hill, I'm transitioning to an engineering role with an exciting company (whose product I just happen to love 🔥). I'll share more about that in a couple of weeks, but it's with a heavy heart I leave such an amazing community. I'm thankful for my time working up here and look forward to seeing the great things our students and faculty accomplish!
