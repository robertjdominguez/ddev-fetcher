---
title: "Using React's Context API"
hook: "Have you ever seen a 'no context' account on Twitter or Instagram? Unfortunately, this post probably isn't as exciting as one of those, but there may be a few (unintentional) laughs involved. I just learned about React's context api and, man, is it great."
slug: context-api
created_at: 2021-10-29T16:36:23.678558+00:00
image: https://media.graphassets.com/OdordQHLTy2ImYa2cG4q
---

# Using React's Context API

### Thoughts on State Management

In the past, before using a framework like Gatsby or Next, I would always reach for [Redux](https://redux.js.org/) to handle state management. Back then, for a relative React newbie, it was _a lot_ to remember and to implement. And, in retrospect, almost totally unnecessary for the amount of prop-drilling I was doing anyway.

One of the great things that modern JavaScript architectures handle is the abstraction of data and what to do with it. Depending on your favorite flavor, this can be an alarmingly pleasant ([Svelte](https://svelte.dev/)) or a horribly painful process (more complex React applications).

If you're not careful, the inclusion of a state-management library can quickly become untenable and add unneeded complexity to your project. As such, React developed the Context API several years ago. The Context API makes for a relatively simple provider to be created and accessed from any component in your app; reusable/accessible state anywhere, without prop drilling!

### Context about Context

At the end of last week, I started on a new build at work. [This is a promotional site](https://bus-cent.vercel.app/) for a fundraising event at the end of the summer of 2022. The menu utilizes [Framer Motion](https://www.framer.com/motion/) to animate in and out based on a piece of state. Initially, this state lived in the menu component, but I wanted to trigger the menu being revealed by clicking on a button in the hero section of the landing page. My hero component isn't aware of the menu component, so how can these two share state? Enter context.

The context API uses a provider that works as a wrapper around your entire React application. My wrapper, along with a `useAppContext` hook, looks like this:

```jsx
// ./ctx.js
import { createContext, useContext, useState } from "react"

const AppContext = createContext()

export function AppWrapper({ children }) {
  // this state used to live in ./components/nav/Menu.js
  const [isNavVisible, setIsNavVisible] = useState(false)

  //   This object gets passed into the value prop on the provider below
  let state = {
    isNavVisible,
    setIsNavVisible,
  }

  // This provider gets wrapped around the entire app
  return <AppContext.Provider value={state}>{children}</AppContext.Provider>
}

export function useAppContext() {
  return useContext(AppContext)
}
```

I'm using Next, so my `_app.js` looks like this with the addition of the context provider:

```jsx
// ./pages/_app.js

import "../styles/globals.css"
import Layout from "../components/layout/Layout"
import { AppWrapper } from "../ctx"

function MyApp({ Component, pageProps }) {
  return (
    <AppWrapper>
      <Layout>
        <Component {...pageProps} />
      </Layout>
    </AppWrapper>
  )
}

export default MyApp
```

And now, with `useAppContext()` from the `ctx.js` component, I can access the state from any component:

```jsx
import Link from "next/link"
import styled from "styled-components"
import { motion } from "framer-motion"
import { landChildren, landVariants } from "../../styles/Lib"
import { useAppContext } from "../../ctx"

const Hero = () => {
  // Think of this as useState
  const { isNavVisible, setIsNavVisible } = useAppContext()
  return (
    <HeroHeader variants={landVariants} initial="hidden" animate="visible">
      <motion.h1 className="big-ass" variants={landChildren}>
        B.U.S. Centennial
      </motion.h1>
      <motion.div>
        <Link href="/#rsvp">
          <PrimaryBtn variants={landChildren}>RSVP</PrimaryBtn>
        </Link>
        <SecondaryBtn
          variants={landChildren}
          onClick={() => setIsNavVisible(true)}
        >
          Learn More
        </SecondaryBtn>
      </motion.div>
    </HeroHeader>
  )
}
```

And, viola! I can now control the menu's animation via the state object in my `ctx.js` file.

![bus.gif](https://media.graphcms.com/KmC2JzIDTihqv1lTHpRg)

