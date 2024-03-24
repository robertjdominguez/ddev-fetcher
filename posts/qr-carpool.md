---
title: "QR Carpool"
hook: "QR codes have surged in popularity since the start of the pandemic. As such, they've even made their way into our carpool system at work; gone are the days of shouting over radios...now, there's an 'app' for that."
slug: qr-carpool
created_at: 2021-10-29T16:36:57.459475+00:00
image: https://media.graphassets.com/GQYf9cD0SxiHPTwsdzdI
---

# QR Carpool

### "Negative Ghost Rider, the pattern is full."

Every time I get a two-way radio in my hands, my mind immediately goes back to the war movies I watched as a kid (there were _tons_). And, much like Ewan McGregor <a href="https://www.imdb.com/title/tt0120915/trivia/" target="_blank">making lightsaber noises</a> that had to be edited out in post-production, I can't help but make that 'ka-shhh' noise in the back of my throat at the end of each transmission. That alone is reason enough to build what you'll see below.

Returning to those war movies, while I'm sure afternoon carpool is vastly different from combat, there have to be some similarities in making sure an orchestrated, well-timed sequence of events and movements of personnel goes off without a hitch.

This is my most recent build at work. Often, I don't get to share the details of the projects I work on as they're meant for internal use. (This one is, too.) However, I presented it to a group of other educators and technology professionals within a Microsoft PLC using some dummy data and was required to record a video; you can find that below if you're interested.

This app utilizes QR codes, placed on transparent stickers, to be used for afternoon carpool at our school. The QR codes are encoded with a unique number for each family; when a faculty member scans the QR code, all the students from that family are added into a live-updating queue.

### The Stack

It should come as no surprise to anyone who's read any of my recent posts, but I've been big on utilizing Nextjs and Hasura. Without them, this app would have been a _beast_ to build. Under the hood, it's a pretty simple marriage of these two pieces of tech to facilitate creating and updating a series of records in a database.

The schema for the database, aside from the auth components, essentially looks like this:

- **families** all receiving their own uuid, determined by their physical address
- **students** which have a foreign key linking them to a family
- **rides** which have a field of `studentId` and a `pickedUp` boolean

One of the most exciting pieces of this build was a chance to work with <a href="https://swr.vercel.app/" target="_blank">SWR</a> and the `useSWR` hook from Vercel. In my first Nextjs project, I tried using this but had a lot of trouble controlling my data the way I wanted to. Now that I feel more confident with using it in conjunction with GraphQL, its power is clear.

SWR allows refetching of data on regular intervals. For the queues, they're powered by the `useSWR` hook and update regularly -- that way, whenever a new student is added to the queue, it 'live' updates for the users watching the rooms.

As this was built for use at a Microsoft school, our Azure AD acts as an identity provider. Faculty and staff don't need to remember yet another password or username; they simply use their existing accounts.

There were some other fun pieces for this build. Did you know there's a <a href="https://developer.mozilla.org/en-US/docs/Web/API/Vibration_API" target="_blank">browser API for vibrations</a> and haptic feedback? I didn't until this project! It's easy to use and tons of fun.

I also did some things with <a href="https://developer.mozilla.org/en-US/docs/Web/CSS/@keyframes" target="_blank">keyframe animations in CSS</a> that I hadn't used before. There's emojis (like a waving hand: 👋) for user-actions related to students and their rides. I animated these to wave or grow depending upon the action button. It's cute. Teachers like cute.

### Logistics

Early on, I was convinced to use a mail merge for creating QR codes and placing them on a template for <a href="https://www.avery.com/blank/labels/94107/gc-film?qty=1&gclid=Cj0KCQjwwY-LBhD6ARIsACvT72PUY4EqWWfihS1vlF_LAIfzgE66rFxf9XU4Je6EMT0j4XQe42rqQcEaAkmzEALw_wcB&gclsrc=aw.ds" target="_blank">labels</a>.

I hate mail merge.

More accurately, I hate the steps involved to clean up and shape data to _be_ mail merged.

And, to reiterate, I hate mail merge.

Long story short: after I documented the steps for what would have to be done next year for the new batch of data, every fifth step was, "have a 🍺." It became clear I should abstract away these steps and automate the process.

With api routes in Nextjs, it will be a breeze next year:

1. From an admin view, drag/drop a csv with the correct headers/columns containing all students for the _upcoming_ school year.
2. That csv gets sent to an api route wherein the data is parsed. New families will be added to the database; new children added to the database and linked to the correct family; families that no longer have students will be removed.
3. A button on the admin page can be pressed to trigger a request to another api route that will handle generating QR codes and then formatting a Word document for printing with the labels. The user will be able to select a bulk print or choose specific students/families.

### A Demo

For now at least, this is here to stay. Though, half way through, all I could think about was using RFID tags instead of QR codes...talk about automating things! But, for the sake of job security, I'll hold off on that idea until version 2.0. For now, you can see the demo I presented to the Microsoft group.

<iframe width="100%" height="400px" style="margin-top: 2vh" src="https://www.youtube.com/embed/-ISOqt7fnes" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

