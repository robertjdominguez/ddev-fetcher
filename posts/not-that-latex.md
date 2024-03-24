---
title: "Lessons Learned: Building with LaTeX and Using Hooks"
hook: "That's a weird sounding title. In reality, this post is all about lessons learned via an abandoned project. In it, I talk a little bit about self-expression and using the useEffect hook for the first time."
slug: not-that-latex
created_at: 2021-10-29T16:33:15.189374+00:00
image: https://media.graphassets.com/b5AfOQ0TS2kQEHnEobds
---

# Lessons Learned: Building with LaTeX and Using Hooks

### My journey with React

I first started using React about two years ago. To date, I've utilized it in a few client projects that are currently in production, but it's not something I've ever considered myself to have mastered. The notion of "componentizing" the web resonates with me. As a CS teacher, one of the cornerstones of the discipline is abstraction; what is the use of components other than a beautiful, functional method of abstraction?

It's important to acknowledge that React - and, in this case, Gatsby - are complete overkill for the scope of a project like the one this post will cover. However, every build is an opportunity to play with a stack and learn more about it. When I got wind that this project was coming up at work, it didn't take long to settle on wanting to build it using this framework, even if it took a bit more time.

### Project brief

Over the summer, our school switched to a new comprehensive student information system (SIS). Logistically, this has been tremendous: teachers, students, and parents all now have one single endpoint to find information for classes, communicate with each other, and - as this post will cover - enter and view information about grades.

Due to the COVID pandemic, we also switched to a block schedule. Fewer transitions between classes have reduced the number of interactions between students in the hall and fewer academic courses at a time for students - and teachers - has meant more opportunity to dive deep and engage more with the material and each other.

One unexpected change due to compressing a year's worth of content into one semester was the change accompanying exams. Our new SIS allows teachers to utilize a total-points or weighted system for grades. In the past, we would have assigned a certain percentage to each quarter and factored in an exam to count for the remaining percentage of the semester. However, this year, we opted to not require end-of-term exams and chose to allow teachers to decide how they wanted to evaluate the end of the course.

The concern quickly became that, regardless of how their gradebooks were set up, the weight of any end-of-term assessment may end up being incredibly heavy. Teachers are permitted to allow an assessment to count up to 20% of the final grade of the course. What we needed was a way of helping them to visualize and calculate the _true_ value of their end-of-term exams.

### Constraints

Four scenarios were outlined: two for weighted systems and two for total-points systems. In reality, teachers simply need to plug in a few numbers to the appropriate formula and...they're done. There was, however, a desire to create an accessible method that facilitated the process of calculating the weights _for_ them 😉.

First go from our Math department was an Excel file. Teachers would simply plug in their numbers and then...viola: the numbers spit right out. Unfortunately, there was concern people would inadvertently adjust a formula and produce incorrect values. So, that was taken off the table.

The head of the Math department, a good friend of mine, came to me one Friday afternoon and said he wanted to create a Python script that would take in user input and produce the desired values. We both quickly agreed that sending people to a CLI wasn't the most accessible option. I told him, "You know, this would be a stupidly quick site to build. People would get a nice, friendly interface, input their numbers, get their final values, and be done."

He agreed but also displayed some reticence. "Yeah, but _I_ want to learn how."

After coming to an agreement that we would build it _together_, he sent me a PDF of all the necessary formulae...in LaTeX. That gave me an idea.

### Design choices

Earlier I referenced that the first incarnation of this program was a simple Excel file. Seeing as how the logic to solve this problem is basic algebra, the crudest and simplest of sites could have been designed to calculate a user's input...but where's the fun in that?

Since my friend sent me the necessary formulae in LaTeX, I wondered what the experience would be like for a user if the site was designed to look like an article from an academic journal. So, I quickly found a Goolge font that was a close match (Old Standard TT) and referenced a few online writing labs for guidance in creating a design system for type and general appearance.

That night I did a quick prototype in Figma and sent it his way. No response. I got impatient. Saturday morning I set to actually develop a working version of at least one formula just to prove the efficacy. Before long...the whole thing was built. While it is a simple site, it was quick to build _more_ because of the choices via landing on React and Gatsby to handle the "heavy lifting" of state and reusable components.

The one nagging issue I had: each time an exam calculation was complete, the necessary state and the message to the user was "one step" behind. Quick googling led me to understand the nature of hooks running asynchronously and, instead, needing a way to update the UI only _after_ the state had been updated. Enter, `useEffect()`.

### Using `useEffect()` for the first time

This is the uniform state of any exam calculation. The `input` state is set to an empty object so it can be expanded as needed, depending upon the complexity of the calculation and the number of inputs necessary to find its value.

```jsx
const [input, setInput] = useState({})
const [final, setFinal] = useState(null)
const [message, setMessage] = useState(null)
```

#### The problem

After all the inputs were entered by a user, a function called `calc()` would be invoked to determine either the value of the exam or of another category in the gradebook. Below is one example:

```jsx
const calc = () => {
  setFinal(
    Math.round(((input.m * input.p) / (100 * input.m + 100 * input.n)) * 100)
  )
  if (final <= 20) {
    setMessage(`This is good news! You don't have to change anything.`)
  } else {
    setMessage(
      `Because this value is greater than 20%, you need to reduce 
      the number of tests for which your exam will count.`
    )
  }
}
```

The `final` state would be updated via this function and, in theory, would now return to the correct message to the user via this ternary operator. This would be true _if_ these two functions were called synchronously. Unfortunately, they're not and the output below wasn't always truthful.

```jsx
<SolutionText style={final != null ? { opacity: 1 } : { opacity: 0 }}>
  <h3>iii. What do I do?</h3>
  <p>
    Based on the information you entered, your exam should count for{" "}
    <strong>{final}%</strong> of the final grade. {message}
  </p>
</SolutionText>
```

This is where `useEffect` comes into play. This hook runs after every update of the `final` and `message` states. When we update our state via the calculations above, we're causing a re-rendering of these components and, in turn, `useEffect()` fires off. Cool!

```jsx
useEffect(() => {
  if (final <= 20) {
    setMessage(`This is good news! You don't have to change anything.`)
  } else {
    setMessage(
      `Because this value is greater than 20%, you need to reduce 
      the number of tests for which your exam will count.`
    )
  }
}, [final, message])
```

No more "slacking" updates on the UI! Quick and easy.

### Reflection

More than anything, this was just a fun project to build. In the end, for a number of reasons, we opted not to use it; that's why it's full of lorem ipsum text and the last two formulae haven't been implemented. However, if you'd like to take a look, I'm still pretty proud of the design and UX. It's approachable, friendly, and is pretty damn close to any academic journal's formatting standards. You can check it out [here](https://exam-latex.netlify.app) or see the [source code on GitHub](https://github.com/robertjdominguez/examLatex).
