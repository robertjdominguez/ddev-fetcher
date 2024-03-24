---
title: "Wordle, Wordle, Wordle!"
hook: "Obsessed with the game? I've honestly never played...but I got really curious at the algorithm behind the word choice each day! Check out this post to see into the code and how the 'random' word is chosen."
slug: wordle
created_at: 2022-02-08T21:26:08.870514+00:00
image: https://media.graphassets.com/AWFzOXCKTWiwsOl0StSE
---

# Wordle, Wordle, Wordle!

### Overview
Wordle has been sweeping the world with its simplicity, cleverness, and great design. While just about everyone I know has been playing it, I only recently learned about it as it seems to have reached a critical mass. When I took a look at it, I thought, "This would be a great lab for my CS students." And, I hope that to still be true! However, my interest really piqued when I learned that it is a completely client-side application. Inherently, this means all the game logic - including the solutions for all days' games - lives on whatever device is accessing it (your phone, computer, etc.). I also know I'm not the first person to do this, but independently figuring out the mechanics was a lot of fun.

### Isn't it just a list?
I mean...okay, yes. However, given the "limited" variety of structural data types in JavaScript, what else would it be?! **The TL;DR version of this is simple: a randomly sorted list lives in the client-side code. Each day, the list just iterates to the next item in the array.** The beauty is in the simplicity by *how* Wordle achieves this to ensure that every player around the world has the same game on the same day.

#### Simple Solutions
If you don't care about the way by which Wordle determines which word is in play for a particular date, but - like my students, want to feel like a hacker - then crack open your developer tools on your favorite browser. Once there, head to the JavaScript console and enter the following:

```js
console.log(JSON.parse(localStorage.gameState));
```
Wordle stores all pertinent game state in local storage. From there, it's just a matter of finding the `solution` property and reading its value.

#### Finding the List
While I was happy to see this, my curiosity had gripped hold of me. I wanted to see more of the innerworkings and determine *how* Wordle knew which word to play for the solution. Since I knew what the solution for the day was (ELDER, by the way), I opened up the sources tab in dev tools and found the main JavaScript file. Once in there, I searched for the solution from that day and found it in an array called `La`. This array contains every solution for ~ 7 years of games. I didn't have knowledge of previous day's solutions, so I'm ashamed to admit how long it took me to realize the list is just iterating one index every day. However, that quickly became clear after walking back through the nested functions to see how that index is selected.

### Understanding the Algorithm
1. I first found the data being written to local storage in an object called `ja`. This object has a property of solution that is set equal to `e.solution`
```js
ja.solution = e.solution;
```
2. Following that trail, you next arrive at `e.solution`'s value being set equal to `Da(e.today)`
```js
e.solution = Da(e.today);
```
3. When you find the `Da()` function, it looks like the following (with comments added):
```js
function Da(e) {
        // We'll follow the function Ga() next
        var a, s = Ga(e);
        return a = s % La.length, La[a]
    }
```
This is the biggest clue we've gotten so far and confirms suspicions of iterating over the `La` array. Knowing that bracket notation dictates `a` should be the index of an item in the array, we know the value of `a` is very important in our quest to understand Wordle's logic.

4. Since we need to follow `a`, we know its value is determined by taking `s` and dividing it by the length of the `La` array. So, what's `s`? Well, from the previous line we can see `s = Ga(e)`, which means we need to follow `Ga()` next. When we find `Ga()`, we get this:
```js
 function Ga(e) {
        return Na(Ha, e)
    }
```
`Ga()` is accepting an argument of `e` and returning the returned value of `Na()`. So...let's keep drilling and see what `Na()` has for us.

5. `Na()` is where the money is. It looks like:
```js
 function Na(e, a) {
        var s = new Date(e),
        t = new Date(a).setHours(0, 0, 0, 0) - s.setHours(0, 0, 0, 0);
        return Math.round(t / 864e5)
    }
```
`e` is the current date and `t` is the difference between `a` (the original date of the game) and the current date. The function returns the difference between the two dates and divides by the number of milliseconds in a day (`864e5`) and that number is the most important bit of logic: it's what becomes the index of the current solution in `La`.

### Summary
We can use that same logic to extrapolate and map the words in the `La` array to the solutions for a particular date. You can [see an implementation of that here](http://wordle.dominguezdev.com).
