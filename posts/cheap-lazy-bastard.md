---
title: "I pay my assistant $1.35 a month"
hook: "Clickbait? Maybe. Maybe not. All you have to do is click here to find out if I'm really that cheap and lack that much empathy. Don't you want to find out... 👋"
slug: cheap-lazy-bastard
created_at: 2023-07-05T21:14:27.138017+00:00
image: https://media.graphassets.com/BpuQY5mTJOYwgElxJqlA
---

# I pay my assistant $1.35 a month

### Yeah, it's ChatGPT
I'm not that much of a bastard. I think you'd be hard-pressed to find anyone bragging about paying a human $1.35 / month. Actually, as I typed that, I realized I'm probably wrong. People can suck. Read on to see why machines are good.

### Motivation
I've written and presented a fair amount about productivity and time management. However, a few weeks ago, one of our DevRels asked, "In a word, what's your brand?" I thought slowly and methodically — one might event say lethargically. However, without hesitation, I answered, "Lazy." Is it that I want to do the least work possible? Is it that I want to load up my teammates with countless tickets while I go for a run? Is it that I just really want to finish Succession without this job getting in the way? Of course (mostly) not — it's because there's effective uses of time and there's tasks that machines can do better and quicker to help us be more productive. And, with the advent of Generative AI, the scope of these machine-enabled tasks is quickly growing.

In my eyes, discipline is foundational to success. Self-discipline, as an engineer on a docs team, comes in the form of starting every day with the same set of tasks: 
- check for activity in PRs that I've either authored or been assigned
- if I'm feeling altruistic or curious, maybe I'll check in on my teammates' PRs, too
- check over Slack threads for any new activity
- figure out how to prioritize and execute for the day

All in all, this usually takes ~ 30 minutes each morning. Working in a 24-hour-globally-distributed world is kind of like a house party back in college: someone is always doing something. When you wake up, it's nice to have context and get a sense of what's been going on.

After working on a new GenAI course for our [Learn site](https://hasura.io/learn), I realized how incredibly easy it is to work with OpenAI's API. And, astonishingly to me, how cost-effective it can be.

### Implementation
My morning routine is pretty simple: get up at 5:00 am, scan through emails, check PRs, check Slack, shower, actually get in front of the computer and start working by 6:00 am. Squinting at my phone with one eye open and trying to parse what, often times, is a lot of activity in a set of PRs can be difficult. It can be hard to glean context and get a feel for what needs to be picked up from the conversations via comments. As such, this usually saw me repeating my efforts again after being slightly more awake. In general, the first pass was almost wasted effort.

As I was working on the tutorial for the Learn site, I started to wonder: "What if I could pass a string of comments to OpenAI and let it parse the conversations and tell me what was going on?" Then, I got greedy: "What if I could also tell it to generate a list of action items that I can add to Todoist?" I knew what my 4th of July holiday would be spent doing.

### Infra
The Node.js script is astonishingly simple: it uses the GitHub API to determine what PRs that I'm involved in (one way or another) have had any activity in the past 24 hours. If there is activity, it aggregates the recent comments into a single string and then passes that, along with a prompt, to ChatGPT. A separate module parses the results and uses the Todoist API to generate tasks related to the summary for each PR. Finally, I send a nice, concise report - with clickable links - to myself via `nodemailer`. Now, I wake up to a single email that's already a succinct summary, coupled with a set of tasks to get the day started (and ended) quicker 🔥

Additionally, I run it all on a cron job via [Render](https://render.com) for < $1.00 / month. Despite the plethora of PRs I'm involved in as an engineer working on documentation, each run of the script is only ~ $0.02 / day.

### "I want to be lazy, too"
Of course you do! Check out [the repo](https://github.com/robertjdominguez/lazy-dev) and feel free to contribute. I started building the Slack module to check for - and summarize - recent threads, but haven't gotten around to completing it yet (because I'm lazy?).
