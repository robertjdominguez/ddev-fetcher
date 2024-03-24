---
title: "Building a Chatbot with AI"
hook: "My curiosity recently got the better of me. I built an open-sourced chatbot for use with Docusaurus. Though, it can easily be re-worked to take any directory and vectorize the data within it for simple, intuitive natural-language queries and conversations."
slug: chat-bot
created_at: 2023-10-13T15:36:20.48229+00:00
image: https://media.graphassets.com/8yKCMYG7SLOOCbGgfZUQ
---

# Building a Chatbot with AI

## Overview

### Disclaimer

A few weeks ago, curious after some work around [vectorizing PostgreSQL data](https://github.com/hasura/vectorize-postgresql-data-for-weaviate), I wondered how hard it would be to turn a directory of `.mdx` files into relational data, vectorize it, and then run queries against it using an LLM. Turns out, it’s very easy. 

At least, it’s very easy from a conceptual and “will it work?” point of view. What I share below is just an exploration of my curiosity and not what we’re (Hasura) currently using or plan to use. I learned some cool things that are new to me along the way, and want to share them for those that are also curious. 

[This repo](https://github.com/robertjdominguez/docs-bot) is open-sourced on GitHub and available for anyone that would like to build on it or improve it 🤙

You can also follow the instruction to quickly and easily deploy it anywhere you can deploy using Compose 🚀

### How’s it work?

Our `docker-compose.yml` has four services; of these four services, three will run continuously:

- A `postgres` service that runs a relational database with persistence.
- A `weaviate` service that stores our vectorized data.
- A `vectorizer` service that will crawl the directory » add the data to our PostgreSQL database » hit our server (👇) and vectorize the data into our Weaviate instance » then die.
- A `server` service — using Express — that acts as the API for the entire bot and is the only service accessible outside the docker network. Clients can hit this and execute LLM queries on the vectorized data.

## Lessons learned

### Building and deploying with Docker Compose

Docker Compose is a tool that allows you to define and manage multi-container Docker applications. It simplifies the process of running multiple containers, coordinating their configurations, and connecting them together. With Docker Compose, we can easily specify the services, networks, and volumes required for our application, enabling efficient development and deployment of complex, interconnected systems.

Essentially, from one command, you get an entire coordinated suite of services running in harmony: `docker compose up -d` and then you can easily check out what’s happening at any point by tailing the logs with something like: `docker logs -f --tail 50 <SERVICE_NAME>`

For shits and giggles, I deployed this to Digital Ocean just to see how difficult it would be. The hardest part? Waiting for the droplet to be provisioned! After it was up, I simply SSHed into the droplet, clone the repository, created a `.env`, ran `docker compose up -d`, and was able to hit my server from a client on my local machine 🎉

### Crawling docs data

As Docusaurus is a popular framework for documentation, I figured it was safe to make a few bets around the structure files:

- **Markdown is king**: if you wish, you could easily change the `.mdx` filtering to `.md` if you’re not into JSX 🤙
- **Frontmatter**: this is present at the top of each file and includes keywords, which are useful in our `nearText` query we perform to determine *****which***** files should be used to help answer a user’s question
- **File paths**: the file tree reflects our routing — thus, we’ve organized things in a relevant and contextual way. I felt this was important, so the path is saved as a field in the relational DB, allowing us to use it in `nearText` searches, too.

### Vectorizing relational data

Vectorizing relational data is crucial for AI/ML applications as it enables efficient and effective analysis of complex, interconnected systems. By converting the relational data — in this case, our docs pages that we piped into PostgreSQL — into vector representations, we can leverage advanced techniques like natural language processing and machine learning algorithms to process and understand the data more efficiently.

Vectorization allows us to perform tasks, like similarity matching, that empower AI/ML models to extract meaningful patterns and insights from relational data, enabling us to build a more intelligent and accurate bot.

### Performing LLM queries

Before we can make an LLM query (like what you would expect in a GUI for ChatGPT), we need to narrow the context for our bot. The more accurate and precise we can be in the information that we tell it to reference, the better the answers will be. We achieve this by performing a `nearText` query first to limit which of our docs pages we want to reference.

Performing a `nearText` query before performing the LLM query is essential to ensure the relevance and accuracy of the results. The `nearText` query helps identify and retrieve relevant documents based on their similarity to the user's query. 

By narrowing down the search space with `nearText`, we can then use the LLM query to extract more meaningful insights and patterns from the selected pages. This two-step process enhances the effectiveness of the bot in providing accurate and valuable information to the users.

### Streaming results

Nobody wants to wait for results. It’s more natural and easier exciting to engage with the bot when it feels conversational — streaming helps this. For those unsure of this term, “streaming” simply refers to the ability of the bot to return a response character by character instead of waiting for the entire body to be complete. For longer responses, this can take a while — thus, users are assumed to have a higher level of engagement when responses are streamed instead of sent upon completion.

Streaming is incredibly easy to implement from the `openai` package in Node.js. However, there’s a bit of complexity from the server-side code: we have to create a WebSocket instead of a traditional `GET` or `POST` route. This repo uses `express-ws` to achieve this.

## Nice to haves

I only spent a few hours over two days working on this, so the client is…lacking. In reality, it can handle streaming conversations and store context — in state — for having continual conversations that don’t have to include explicit question-asking over and over. Past that, I haven’t spent much time with any other element of it. Most of what I say below is related to how a client could be improved 🤷‍♂️

### Storing conversations and user information

In order to enhance the functionality and personalization of the chatbot, it would be valuable to store user information from each conversation, including the body of the conversation. By capturing and storing this data, we can gain insights into user preferences, behavior patterns, and specific topics of interest. 

This information can then be used to improve the overall user experience of the product Additionally, having access to the conversation history allows for continuity in future interactions, enabling the chatbot to provide more contextually relevant and personalized responses.

### Suggested topics / questions

In addition to the functionality and personalization enhancements mentioned above, I believe it would be valuable to implement a feature that suggests topics or questions based on previous questions a user has asked. This would not only make the chatbot more user-friendly, but it would also encourage further engagement and exploration of relevant information.

By analyzing the conversation history and understanding the context of the user's queries, the chatbot could intelligently generate suggestions for related topics or questions that the user might find interesting or useful. This proactive approach to guiding the conversation would provide a more seamless and intuitive user experience, ultimately enhancing the overall value and effectiveness of the chatbot.

By offering suggested topics or questions based on the user's previous questions, the chatbot can not only provide immediate assistance but also foster curiosity and exploration, empowering users to discover new information and engage in meaningful conversations.

### Better rendering in a client

In the responses generated by the chatbot, I would like to render code snippets, terms, hyperlinks, and other elements in a visually appealing and user-friendly manner. Code snippets should be formatted with syntax highlighting to make them easy to read and distinguish from regular text. Terms or concepts mentioned in the responses can be emphasized or highlighted to draw attention to them. Hyperlinks should be clickable and displayed as anchor text, allowing users to quickly access related resources or additional information. Additionally, I would like to ensure that the formatting and rendering of these elements are consistent across different client interfaces to provide a seamless and cohesive user experience.
