---
title: "Stripe Webhooks and Next.js"
hook: "Webhooks are nothing new, but I'm just realizing their potential when paired with API routes in Next.js. For a SaaS I'm currently building, I needed a way of programmatically generating licenses for users after purchase. With Stripe webhooks, it was a snap.
"
slug: stripe-webhooks-and-nextjs
created_at: 2021-10-29T16:35:11.481875+00:00
image: https://media.graphassets.com/D6Y9JsKpQ8eboYq7iJbt
---

# Stripe Webhooks and Next.js

### Overview

For the past two weeks I've been working on an education SaaS for teaching computer science; specifically, for teaching web design and development. While there's a ton of great resources for self-learners, there's not a lot of well-developed content that's geared towards teachers. _All_ the service's content will be free, but there's a 'pro' option for those that want built-in formative assessments (think quizzes to help check for understanding) for students and data-tracking to help keep students on pace for learning outcomes.

In the past I've used [Next.js API routes](https://nextjs.org/docs/api-routes/introduction) to handle the logic of processing payments with Stripe's Payment Intent API. I would chain together promises and, after a payment cleared, send a mutation to the graphQL service (in my case, this has been Hasura) to insert or update a record, registration, etc. This worked well. However, I wasn't ever really happy with my front-end forms' designs and the labors of validating users' input into form fields. Stripe has a great option called [Checkout](https://stripe.com/payments/checkout) that allows a developer to quickly 'build conversion-optimized payment forms, hosted on Stripe.' With a new library called [next-stripe](https://github.com/ynnoj/next-stripe), you can generate these Checkout sessions from your front end. For jamstack builds, this is pretty sweet.

### Stripe

It should be no surprise that Stripe makes _everything_ easier for developers. In particular, two tools stand out for this process: webhooks and the Stripe CLI. Below is the basic architecture of the flow.

![stripe-diagram.png](https://media.graphcms.com/qzqZL5BdTZq4hRgG4dcT)

1. A user will purchase a license via Stripe Checkout.
2. That session - along with some extra metadata - will get passed to our API route via a webhook trigger set up on our Stripe dashboard.
3. Finally, our API route will process the request and send a mutation to our db.

All of this is happening while the user watches their 'processing' animation on the Stripe Checkout. When they're redirected - after a successful payment - to their dashboard, their licenses will be waiting for them to use!

#### The Stripe CLI

The Stripe CLI makes getting started with webhooks incredibly straightforward. Their [documentation](https://stripe.com/docs/webhooks/test) is great, the walkthroughs are easy to follow, and - if you're logged in - they even throw your own keys into the code snippets.

With the CLI we can listen to events happening with our Stripe Checkout sessions. When our webhook gets hit, we can have that request forwarded to our development server and test/debug before pushing to production.

When it's time for our webhook to be utilized in production, we can simply add the API route's path to our webhooks in the Stripe dashboard.

### API Routes

API routes are the severless component of this system. For one-off instances, where you'd like to house business logic _away_ from the client and do the work of an actual server, these routes are a godsend.

In this instance, we're simply waiting for a request to come through that contains the Checkout session data. Fortunately, while this data object is already **substantial**, we can add unique metadata if we need certain information to travel from our initial checkout flow to this point of the process. Particularly, I needed the user's email address in case they used a different one when completing the Checkout form.

From there the API route can simply send the mutation and generate the licenses attached to the user who made the purchase.

### More to Come

Purposefully there's not a ton of detail in this post. More than anything I'm trying to document my process through this build and remember patterns for solutions. I'm looking forward to posting more about this new service over the next several weeks and hope to make it available by the start of the [American] school year in August.

