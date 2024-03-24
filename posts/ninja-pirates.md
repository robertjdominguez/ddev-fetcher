---
title: "Lessons Learned from The Martian"
hook: "The Martian should be celebrated as one of the greatest, nerdiest, most kick-ass pieces of literature ever written. For any teacher that likes to utilize a problem-based approach in class, look no further than Mark Watney's attempts to stay alive on the red planet."
slug: ninja-pirates
created_at: 2021-10-29T16:27:22.008969+00:00
image: https://media.graphassets.com/hcmDLfYVT4eKZSKcGRI4
---

# Lessons Learned from The Martian

### "I'm f-----."

Is there a better first line to a novel? If there is, I haven't found it yet. I didn't first learn about _The Martian_ until it was a major motion picture starring Matt Damon and - the United States - was having to shell out billions of dollars to save him...again (ever notice how Matt Damon is always getting himself into trouble...[It's a real issue](https://time.com/4162254/cost-of-rescuing-matt-damon/)). However, as soon as I learned it was a book as well, I purchased it.

Andy Weir is a tremendously skilled author. Reading his words, it quickly becomes clear that a significant amount of research went into creating an authentic scenario wherein an astronaut would have to "MacGyver" (or, in Watney's words, _"Science the s\*\*\*..."_) through different scenarios to survive and eventually make it back to Earth.

### The Premise

In the not-too-distant future, NASA has been sending manned missions to Mars. The crew of Ares III, with engineer/botanist Mark Watney, is six sols (Martian days) into a month-long mission. Spoiler alert: disaster strikes during a storm that causes the rest of the crew to think Watney is dead; they bug out and he comes to a few hours later realizing he's fu-- well, remeber the first line?

To not give anything away, let's just say the next 400+ sols are filled with near-death attempts at solving complex and wide-ranging problems. Weir's background in computer science clearly manifests itself in the logic-driven approach we see Watney use to stay alive.

### Lessons

Below, I've listed out a few examples of lessons that come directly from _The Martian_. These lessons would be most at home in algebraic based math or computer science classes.

#### Hexadecimals to the rescue

Early on Watney realizes he has the opportunity to communicate with Earth by retrieving the _Pathfinder_ probe and seeing if it can acquire a signal with Earth. There's no direct way of communicating, but he realizes that _Pathfinder_ has a camera that can rotate; he settles on using hexadecimal characters (0-9, A-F) on the perimeter for NASA to spell out messages by pointing the camera at them. Why hex? Easy: the English alphabet utilizes 26 letters, Watney would also need a question mark to indicate a query opposed to a statement. This would give him 27 characters in total. This doesn't leave a very big margin for pointing at these characters on the perimeter of _Pathfinder_ and he needs his messaging to be precise. Instead, if he were to use hexadecimal characters, he would only need 15 characters thereby giving him a great degree of arc for _Pathfinder_ to point for each character.

##### In the classroom

I think this scene opens up lots of opportunities, even at the youngest ages. In an elementary math lesson, students could work to calculate the amount of arc available for each character and - reaching towards higher levels of Bloom's Taxonomy - articulate the reasoning behind _why_ hexadecimals would be better than the traditional alphabet.

However, this scene really shines in a CS course. Hexadecimals can seem confusing, especially to younger students. In reality, encoding in this format is really just about completing two simple arithmetic operations: multiplication and addition.

Recently, I used this lesson with a middle school CS class. The primary objective of the course is to introduce students to algorithms: what they are, how they work, how to solve them, explain them, and create their own. Using this scene from the movie, students complete a lesson in three parts:

1. **Background** -- what is Hex?

   In this portion of the lesson, I wrote a short explanation that set the stage for students. I told them broadly about _The Martian_, explained Watney's dilemma with messaging, and told them his solution.

2. **Practice** -- translate the messages

   After giving students some background information, I began explaining hex and _why_ it made more sense than the traditional alphabet. Students followed along as I introduced ASCII and explained that computers really see text characters as input-values. To connect to previous lessons on encryption, we used the term "key".

   After walking students through _how_ to convert from hex to ASCII to a character, I gave them a series of messages that they had to translate; these acted as "practice problems" to help give them confidence through repetition.

3. **Explain** -- walk us through this algorithm

   The following week students received a nearly blank flowchart that began with `hex` as input and ended with `character` as the output. Within the flowchart, there were two bits of logic; students had never explicitly been told the algorithm for converting this information but had practiced it dozens of times. Their objective was to record a short screencast on Flipgrid wherein they explained the process of decoding a hexadecimal to an English character.

### Pedagogy

How _The Martian_ is written lends itself to being used in the classroom. Repeatedly, problems are presented from the perspective of the protagonist to the reader. The natural reaction is to think, "How is he going to solve this?!"

For any teacher who has worked with a problem-based approach, it's clear that students are more engaged from the beginning of a lesson if there's a hook -- here, the antecedent near-death experiences Watney endures are cause enough to bring students into the material and _want_ to come up with a solution. This leads to more opportunities for independent investigation that allows for creative, unique solutions -- even different from that in the book.

For any teacher looking to get students engaged with the material, check it out now!

