---
title: "Runbooks Can Save Your Life"
hook: "That's a bold statement, but I've learned to harness the power of writing processes and procedures down into a reproducible and replicable set of steps."
slug: runbooks
created_at: 2023-05-30T20:36:18.262747+00:00
image: https://media.graphassets.com/HMHXOo2IS7KOSeoqGdco
---

# Runbooks Can Save Your Life

### Overview

It's hard to believe, but today marks my one-year anniversary at Hasura. Over the past year, I've had the opportunity to learn so much about software engineering. From CI/CD, to working with Git in a team environment, to seeing just how much work goes into a single version release — it’s been astounding and humbling. However, there’s one thing I didn’t expect: taking tons of notes and compiling them into runbooks. 

### What are runbooks?

For those who may not be familiar, runbooks are a critical tool for software engineers. They are essentially a collection of procedures and guidelines that are used to ensure that critical responsibilities are performed consistently and efficiently. While they may not be the most exciting aspect of software engineering, runbooks are an essential component of any successful engineering team.

So why are runbooks so important? For starters, they help to avoid downtime by providing a clear and concise set of instructions for how to handle various scenarios. Additionally, runbooks help to minimize errors by standardizing processes and ensuring that everyone on the team is following the same procedures. They also help to enhance collaboration by providing a shared set of guidelines that everyone can reference.

I've recently been reading the book "Extreme Ownership", and one of the key principles that’s stressed is the importance of keeping everything simple. In the military, standardizing procedures is a way to ensure consistent and predictable performance, which ultimately saves time. This same principle can be applied to creating effective runbooks. By keeping procedures clear and concise, and avoiding unnecessary complexity, you can ensure that everyone on the team is following the same guidelines and can quickly and efficiently perform critical responsibilities.

### How you can create runbooks

Of course, creating effective runbooks is easier said than done. It requires a lot of time and effort to identify critical responsibilities, document step-by-step procedures, and keep runbooks up-to-date. However, the benefits of having well-maintained runbooks are well worth the effort.

So, how can you create effective runbooks? It starts with identifying critical responsibilities. This means taking the time to think through all of the various tasks and procedures that are required to keep your systems running smoothly. Once you've identified these responsibilities, you can begin to document step-by-step procedures for each one. It's important to keep these procedures as clear and concise as possible, so that anyone on the team can easily follow them.

### My runbooks

The kinds of runbooks I keep 👇

**Deployments**

I have a runbook that include the steps to deploy code to each of our environments, including staging and production. This runbook also includes two common scenarios (a general release vs. cherry-picked), the steps to roll back a deployment if necessary, as well as troubleshooting steps for common deployment issues such as merge conflicts.

**Reviewing PRs**

I have a runbook that outlines the process for reviewing pull requests. This includes ensuring that the code meets our team's coding standards, that the changes are consistent with the intended functionality, and that the docs work as expected. The runbook also outlines the steps for providing feedback and for merging the code into the main branch once it has been approved.

**Writing Redirects**

I have a runbook that outlines the process for writing redirects. This includes first identifying the URLs that need to be redirected, and then determining the best way to redirect them (= || ~). The runbook also includes instructions for how to test the redirects to ensure that they are working as expected.

**Dealing with old PRs**

I have a runbook that outlines the process for migrating old PRs to new documentation formats such as MDX from RST. This includes identifying the PRs that need to be migrated, and determining the best approach for migrating them.

The runbook also includes instructions for creating new documentation files, updating links, and making any necessary changes to the codebase. Additionally, the runbook outlines the steps for testing the new documentation to ensure that it is working as expected.

Migrating old PRs to new documentation formats can be time-consuming, but it's an important part of keeping our documentation up-to-date and ensuring that it remains relevant over time and that community contributions are considered.

### Summary

Creating runbooks is only the first step. It's also important to keep them up-to-date, so that they remain accurate and relevant over time. This means incorporating feedback from the team and making changes as needed. Additionally, it's important to include troubleshooting steps in your runbooks, so that anyone on the team can quickly and easily diagnose and resolve issues.

Runbooks help to ensure that critical responsibilities are performed consistently and efficiently, and they provide a shared set of guidelines that everyone on the team can reference. While creating effective runbooks requires a lot of time and effort, the benefits are well worth it. So if you haven't already, I encourage you to start creating and maintaining runbooks for your critical responsibilities today.
