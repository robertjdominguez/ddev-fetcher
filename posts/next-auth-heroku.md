---
title: "Heroku's Rotating Credentials"
hook: "Recently I created a client site that utilizes NextAuth.js for authentication. I needed the user data to persist in a database, but the connection configuration can be a pain with Heroku's rotating credentials. Read this post to see how I solved it."
slug: next-auth-heroku
created_at: 2021-10-29T16:40:38.664875+00:00
image: https://media.graphassets.com/lJRjZ1xVQRmSyih7LPaW
---

# Heroku's Rotating Credentials

### Blanket Statement

Holy s\*\*\* -- Next.js is an _amazing_ developer experience. If you've been thinking about using it in a project, stop waiting. As Next.js states, it's opinionated but quick to pick up. If you feel competent about your React skills, then spend a weekend getting to know the "production grade" framework for React.

### Overview

I was recently approached by a client to do a last-minute rebuild of a _giant_ monolithic application. Broadly, the project requirements included a CMS for non-technical editors, e-commerce, different user roles, and authentication. I've been using different JAMstack frameworks for the past year to develop personal projects and less complicated client projects. I _thought_ I had more time before this project was going to be dropped in my lap; unfortunately, that wasn't the case! However, I wanted to challenge myself to migrate the front and back ends of this project from a server-rendered Python application to a version implementing Next.js for a better UX and a more enjoyable development experience for me.

### Problem

Working in a new stack means reaching for new libraries to solve common problems. I've always viewed auth as the biggest pain point and it's undoubtedly one of the elements of a project that almost always slows me down. In the short period I had to do some research and start to think about how I would rebuild this project, I'd stumbled across [NextAuth](https://next-auth.js.org/) -- an easy, flexible, secure, and relatively unopinionated library for handling authentication in a Next.js application.

Additionally, I had a postgres database already set up on Heroku to store information relative to the users and their activity within the application. The issue? NextAuth, when using a database, requires a connection string - or a config object with identical information - to said postgres (or w/e flavor) database.

```javascript
import NextAuth from "next-auth"
import Providers from "next-auth/providers"

export default NextAuth({
  providers: [
    Providers.Email({
      server: process.env.EMAIL_SERVER,
      from: process.env.EMAIL_FROM,
    }),
  ],
  // Optional SQL or MongoDB database to persist users
  database: process.env.DATABASE_URL,
})
```

The `database` property of the config object is what concerns us: this string _cannot_ be hardcoded if we want to utilize the Heroku postgres database with NextAuth.

In particular, the issue with Heroku and its postgres add-on is that the credentials to access this database change... _all the time_. They go through repeated efforts to make this as clear as possible to anyone accessing these credentials in the postgres configuration on the Heroku app dashboard. Below is the needed configuration file named `[...next-auth].js` within the `api/auth` directory of a Next.js application. This allows for an email provider, but can easily be augmented by other social providers.

### Solution

As NextAuth's documentation states, one of the two following configs is required for using a db:

```javascript
database: "mysql://nextauth:password@127.0.0.1:3306/database_name"
```

Or

```javascript
database: {
  type: 'mysql',
  host: '127.0.0.1',
  port: 3306,
  username: 'nextauth',
  password: 'password',
  database: 'database_name'
}
```

Again, as stated in their documentation, these two values are identical.

We're going to use the first option. We'll generate a string in the shape of the configuration above by utilizing Heroku's API. You'll need your API key and some information from your application's dashboard on Heroku. You can access your API key from your **account** settings. You'll also need the **name** of your postgres add-on; this can be found from the **application's** settings.

Once you've got those pieces, it's pretty easy to make the API call and generate the string as the value of the `database` property in the config file. I've used axios as the fetching library, but choose your favorite flavor. Additionally, I've stored the API key and the postgres db's name in environment variables.

```javascript
/* imports... */

// Connection to Heroku API
const herokuApi = axios.create({
  baseURL: `https://api.heroku.com`,
  headers: {
    Authorization: `Bearer ${process.env.HEROKU_API}`,
    Accept: "application/vnd.heroku+json; version=3",
  },
})

// Async function to get database uri
const getCredentials = async () => {
  const res = await herokuApi.get(
    `addons/${process.env.HEROKU_POSTGRES}/config`
  )
  const cnxnString = res.data[0].value + "?ssl=no-verify" // This SSL bit is necessary for Heroku and NextAuth to play nicely
  return cnxnString
}

/* actual config... */
```

So, what's happening here? Well, we're generating a string in the shape of the initial database config found above. Each time the NextAuth API route is hit in our application, we're actively making an async request to Heroku's API to get the current credentials for the postgres database. That way, if there are security updates, our dyno is restarted, or Heroku simply rolls things over, we can still make a connection to the database.

To use this string, we just await the response within the configuration:

```javascript
export default async (req, res) =>
  NextAuth(req, res, {
    providers: [
      Providers.Email({
        server: process.env.EMAIL_SERVER,
        from: process.env.EMAIL_FROM,
      }),
    ],
    database: await getCredentials(), // our connection string gets returned here!
  })
```

Putting it all together, our `/api/auth/[...nextauth].js` file looks like this:

```javascript
import NextAuth from "next-auth"
import Providers from "next-auth/providers"
const axios = require("axios")

const herokuApi = axios.create({
  baseURL: `https://api.heroku.com`,
  headers: {
    Authorization: `Bearer ${process.env.HEROKU_API}`,
    Accept: "application/vnd.heroku+json; version=3",
  },
})

// Async function to get database uri
const getCredentials = async () => {
  const res = await herokuApi.get(
    `addons/${process.env.HEROKU_POSTGRES}/config`
  )
  const cnxnString = res.data[0].value + "?ssl=no-verify"
  return cnxnString
}

export default async (req, res) =>
  NextAuth(req, res, {
    providers: [
      Providers.Email({
        server: process.env.EMAIL_SERVER,
        from: process.env.EMAIL_FROM,
      }),
    ],
    database: await getCredentials(),
  })
```

That's all there is to it! Now you can ensure your connection string always has the most up-to-date credentials to keep access uninterrupted.
