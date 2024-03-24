---
title: "Combining Todoist and Reclaim for 10x productivity"
hook: "Yeah, that sounds like clickbait. However, you'll be amazed once you see what you can do by offloading as much low-level thinking and processing to technology."
slug: todo-or-not-todo
created_at: 2022-11-28T23:46:10.724007+00:00
image: https://media.graphassets.com/36qXbkzQS2qarrf2NUlQ
---

# Combining Todoist and Reclaim for 10x productivity

### Retrospective

Six months, nearly to the day, of grinding and working with some of the most passionate, innovative, and motivated people I've ever met has given me a lot of perspective. I've loved working daily with my team and the entire - quickly growing - company on a world-changing product. That may seem a bit cheesy, but it's true.

At the end of May, I left my school of nearly ten years to join Hasura as a software engineer. While I spend most of my time working with markdown files and trying to find ways of improving processes, this brief window has been an eye-opening experience and an opportunity to grow in a new role and industry. I miss the people on the Hilltop, but I can't imagine doing anything else now.

This new landscape triggered the need to understand new procedures, adjust to new ways of working, and find the best means of collaborating with a globally-distributed team. After working in the same building for nine and a half years, developing routines was easy. A highly sought-after flow state became second nature, and it was easy for me to finish my work on time and predictably. Why? I understood the landscape and the steps needed to get from A to B. I knew not only what my next day looked like but also my next week, months, and - in reality - the entire year. No surprises.

I thought I was busy any given day teaching three distinct courses while supporting faculty and students. I was wrong. Or, maybe, I just had things so dialed in it felt easy to achieve success each day and clock out, absent-mindedly heading down the hill to Lou's for a cold beer.

For the past six months, I've been looking at what I do daily, what patterns exist weekly, and how I tackle the work my team is tasked with achieving. I've finally found a process that can elevate me to that flow state and kick my work into high gear.

### How we work

Anyone in the tech industry shouldn't be surprised to hear we utilize the golden standard of SCRUM-based teamwork. Our sprints are usually two-week cycles where we divvy up the work and try to add value to our product by improving user-facing documentation. Adding value can take a lot of forms. Still, it's typically a series of tasks seeking to improve the quality of existing documentation, filling in the gaps if we find any, and creating infrastructure that removes friction for all parties involved. Punctuating all of this is a healthy density of PR reviews wherein we feel like teachers with their red pens...an oddly familiar feeling.

We organize our work on a SCRUM board (using Jira) and have the traditional set of columns:

- to do
- in progress
- review
- blocked
- done

At the end of each sprint, we perform a retro, see where we are, and try to increase our velocity in the fast-paced, high-stakes documentation world. 

While this makes sense at the team level, we're all individuals and choose to tackle our work differently. Until recently, I relied on my GCal and organized my tasks by day. I blocked off my afternoons for deep work, paused Slack notifications, and tried to churn out quality work. I was successful, but I never really felt I achieved that zen I was after.

### Offloading

Slack is a double-edged sword. I've learned to tame the slew of notifications (mainly by muting most channels: sorry, `#hobby-pets`) and to not keep it open constantly when trying to work. One thing I love is Slack statuses. It's nice to know what others are up to and if you can bug them with inane questions (like, have they ever seen [Matt Parker's video on RGB spreadsheets](https://www.youtube.com/watch?v=UBX2QQHlQ_I)).

Early on, I started noticing co-workers had an extra note in their status. Something to the effect of `<status> (via Reclaim.ai)`. I didn't think much of it but noted there must be some automation happening under the hood to set these statuses based on their calendars.

Eventually, my boundless laziness propelled my curiosity. I checked the service out after failing to find a parsimonious way of using Google Calendar's and Slack's APIs to automate my statuses as I changed tasks throughout the day. This was an easy win and didn't require upgrading to Reclaim's paid tier. Couple that with a "smart" calendar that can take in minimum and maximum chunks of time as arguments for a particular task and divide the task up into the empty slots of your calendar...we were getting somewhere. However, I still had the extra step of creating the task in Google after having it assigned to me in Jira and *then* going into Reclaim and creating the task there. I'm greedy. I wanted smoother capture, entry, and scheduling to get me to that flow state.

### Todoist, or not Todoist

My manger uses Todoist. My coworkers use Todoist. I, until recently, did not. I told myself, "I can do everything it does in Google, and it's right in front of me." Wrong.

Not only is Todoist the prototypical to-do app, but it's also the standard for a reason. Productivity nerds bow to the extensibility of its API, the gamification of our Sisyphean tasks, and - most importantly - the NLP that makes creating and organizing those tasks a breeze. This last point is the most critical. Why? We can use this feature when our Todoist is integrated with Reclaim to instantly create and organize a task and schedule it - broken down into smaller units - in our calendar.

### How it works

Todoist parses the input text when creating a new task and uses regex to look for particular values. A few need to be present for Reclaim to know a task needs to be scheduled:

- The estimated length of the entire task
- The `reclaim` tag
- The due date

Using a string like this, we can create an event in Todoist that Reclaim will automatically pick up and schedule on our calendar according to the rules we've set:

`Write a blog post on productivity by Wednesday [90m] @reclaim`

From here, a webhook is triggered, and Reclaim picks up the new task, parses the information, and adds it to the calendar. We told it the task is estimated to take 90 minutes; based on my rules, 30 minutes is the minimum length for a task. Ideally, Reclaim will find a 30-minute and a 60-minute window in my calendar and schedule time to work on this. All without me having to think about when I should do something or if I can fit it in. You can even double down on this strategy by organizing a particularly large task as a project or a parent task with several child tasks that Reclaim will automatically schedule.

All-in-all, it looks like this:

![todoist.gif](https://media.graphassets.com/qnZrJBaESVGVBPTEAJBR)

Find out more about Reclaim and Todoist by checking out these resources:
- [Reclaim.ai](https://reclaim.ai/)
- [Reclaim.ai <> Todoist integration](https://todoist.com/integrations/apps/reclaim)
- [Todoist](https://todoist.com/)
