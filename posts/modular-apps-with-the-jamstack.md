---
title: "Modular Apps with the JAMstack"
hook: "Developing web apps using the JAMstack is pure nirvana. The modular nature and growing range of services (many that are open source) makes for blistering development speeds. In this post I discuss the creative opportunities the JAMstack brings to building new services."
slug: modular-apps-with-the-jamstack
created_at: 2021-10-29T16:34:40.060378+00:00
image: https://media.graphassets.com/j3GV4IkuT62yWTZzGTeF
---

# Modular Apps with the JAMstack

### I hate paperwork

Who doesn't? The mindless monotony of repeating the same task over, and over, and over, and over, and over...you get the picture. Though, the reality is that this is where most problems become evident -- at least for me.

Part of my job as the Director of Education Technology at the school where I work is to offer and lead PD sessions (🎉 awesome!) and track attendance and award certificates (💩 not awesome). We're talking upwards of twenty sessions each school year and around seventy faculty to keep up with. One of the least enjoyable parts of my week is when I've clearly procrastinated this work to the point of realizing, "I should just automate this" and then...don't.

Finally, I started to think about the problem itself and what different issues actually composed it. The workflow for a PD session looks something like this:

1. An email goes out letting faculty know about an upcoming session
2. People would respond and indicate they wanted to attend
3. In bulk I'd send out calendar invites after creating an event
4. I'd run the session, record it, and post it to our in-house Stream
5. ~~Days~~ ~~Weeks~~ Months later I'd send certificates

### I hate back-and-forth emails

This past year I discovered a lifechanging (seriously) calendar service called [Calendly](https://calendly.com). For faculty that needed tech help, I've been able to normalize that it's _really_ not an emergency and we can schedule time to figure out what's going on. It's allowed me so much more time to keep from context switching and focus on the deep work that needs to be done on a daily basis.

A few months ago, I realized I could leverage this service to create PD sessions in advance, send out the links in bulk, and let faculty sign up directly. It stopped the back-and-forth _and_ it generated an invite to a Teams event. This essentially knocks out steps 1-3 in the workflow above...🔥

But, I'm greedy. I wanted more. I wanted to automate the process end-to-end.

### I love building things

The great(est) thing about the JAMstack? The modularity. Like Legos, there's so many choices for solving different, specific problems that you can cook up just about whatever you'd like. You can build applications and services that utilize _specific_ tools to quickly and efficiently solve a specific problem and then move onto the next piece. This makes development **fast**. Like, "I can build a SaaS in a day" fast.

I knew that if I went JAMstack I could quickly and easily combine different open source pieces to create a powerful solution to my issues of laziness (and then some).

### The Stack

1. [Next.js](https://nextjs.org)
   - I remember hearing The Beatles for the first time. I (sort of) remember tasting my first chips and curry on a bender in Belfast. I definitely remember the first time I used Next.js and, if you can't tell, I'm hooked. The fact that routing is already handled, static generation is the default, and API routes are just a folder away makes this, hands down, the best React framework out there.
2. [Hasura](https://hasura.io)
   - Hasura is a realtime GraphQL engine that sits on top of a Postgres database. Essentially, it's the nerve center of this entire service: with it I can stitch together schema from our authentication service, a CMS for non-technical users, and it easily communicates with the front end via an accessible GraphQL API.
3. [GraphCMS](https://graphcms.com)
   - One thing I knew I needed was a CMS for non-technical users to create, publish, and update sessions. There are a _ton_ of great choices out there. One that gives you GraphQL out of the box is GraphCMS. Again, because of the remote schema stitching available in Hasura, I could easily combine these services into one unified GraphQL layer.
4. [Microsoft Graph](https://developer.microsoft.com/en-us/graph)
   - I knew Calendly was using this under the hood to create appointment windows and send invites. It also acts as the identity service so users don't need to create an account -- they just use their school credentials. I've wrestled with Microsoft's Graph API in the past but can thankfully say that this was the _most_ pleasant experience to date...probably because of the last piece of our stack 👇
5. [NextAuth](http://next-auth.js.org/)
   - I typically hate working with auth. However, NextAuth makes for one of the easiest, quickest, and most customizable authentication experiences I've ever dealt with. I was able to set up an application in our Azure portal and then create a Microsoft provider inside of the NextAuth API route -- within minutes the portion of a build that typically takes me a couple of days was done. Seriously 🤯

### The Service

Combining those pieces gives you this [site](http://pd-knight.vercel.app/). The dashboard and authenticated sections are only available to users in our tenant, but you can see how it works:

1. A session is created in GraphCMS
2. A build hook notifies the host who then rebuilds the static portion of the site with the new information
3. A second hook hits an api route to create an event using the Graph API and the response then updates an `id` field in GraphCMS
4. Once the rebuild is complete a user will see the session as an option and, if they're authenticated, can then register for the session
5. Once a session is complete teachers can access it on their dashboard and print a server-rendered certificate _or_ access the recording

Bottom line: instead of automating on the first three steps in my workflow I've now automated all of it. All I - or anyone running a session has to do - is create the entry in GraphCMS and everything else is taken care of once "publish" is clicked.

### Future Versions

This application very clearly solves a problem for teachers at **our** school. However, this issue isn't unique to us. Future iterations will feature access for teachers outside of our organization, async/on-demand sessions that can be consumed anywhere/anytime, and a blog/newsletter that allows us to share what we're learning with the entire education community. We're very excited about this new service and it wouldn't be possible without the JAMstack!

