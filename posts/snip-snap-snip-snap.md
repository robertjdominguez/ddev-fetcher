---
title: "Shopping Shortcuts with Snipcart!"
hook: "I needed to create a 'simple' storefront for a project at work while also demonstrating for my students how JavaScript can be used to generate HTML and improve workflows. Snipcart was clutch!"
slug: snip-snap-snip-snap
created_at: 2022-03-18T14:18:13.834693+00:00
image: https://media.graphassets.com/4z31UvBZTSS6uAUKtGRw
---

# Shopping Shortcuts with Snipcart!

### Some background
At [school](https://www.altamontschool.org/), I was approached by a co-worker, who I facilitate PD with, about helping out with an interdisciplinary project. Her students, in a sixth-grade math course, were going to create a mini non-profit as a part of a comprehensive project that gave them an interdisciplinary experience with a core focus on the math needed to run a business and make "good" decisions.

We have an upper school leadership center where students create their own service projects and work to improve the community. Broadly, these middle schoolers selected different students' projects to support, generated t-shirt designs to sell in support of these projects, and created marketing and business strategies for getting their products to market 🤯  I don't think I could have done that in sixth grade!

Aside from wanting to help these students, I also knew I needed a real-world example for my web design students that showed the entire process of working with clients, to developing a mock-up and getting feedback, to delivering the final product. I *also* have been having a hard time helping them see the benefit of abstracting out the creation of elements based on **incoming** data sources...so I saw a way of killing two birds with one stone.

### What Snipcart does
As their CTA says, [Snipcart](https://snipcart.com/) is a way of adding a shopping cart to any site in minutes. And when I say any site, there's **no** need for frameworks. This is the [final product](https://altamont-school.github.io/miree-shirts/) and is just HTML, CSS, and JS. You can check out the repo [here](https://github.com/Altamont-School/miree-shirts/).

Snipcart *really* does make it incredibly simple to use their service. Throw on a few classes and data-attributes to your page's elements and...viola, their crawler checks your page to make sure the data is correct - for, you know, integrity - and you've got a functioning cart/store! Of course, you have to wire in a payment provider, but...it's a quick process!

### IRL
Aside from working with the students in the math classes, I wanted the opportunity to demonstrate the importance of abstracting out the generation of cards and project sections on the page. Why? Well, there's **numerous** attributes and pieces of data that need to be modified for each project. Why re-write that code over and over when I can just loop over a single data source and plug in my values where needed? Plus, when I rebuild this for next year's projects, why edit all these lines of HTML when I can target my efforts on updating a few properties in some JS objects? Win-win-win (thanks, Michael Scott).

What that all looks like is this: I have a data source, as an array of objects, called `students`.

```js
const students = [
  {
    name: `Riley Pierce`,
    slug: `riley-pierce`,
    picture: `./assets/r-pierce.webp`,
    projectName: `The Recursion Program`,
    bgImage: `./assets/recursion.webp`,
    shirtFront: `./assets/riley-front.webp`,
    shirtBack: `./assets/riley-back.webp`,
    shortBlurb: `Riley’s mission is to reshape our society, as well the homeless people stuck in poverty. He also runs a program to teach the youth about poverty, and how to help people.`,
    desc: `Our mission is to reshape the poverty numbers in our society. We are working to support children to volunteer to assist the effort of lowering the poverty percentage in our area. If you buy this shirt, you will be helping to educate children and help people who are less fortunate. When you buy this shirt, you help plant a seed in a young mind to help impoverished people.<br><br>To help you can buy a shirt. If you do so you are not just buying a shirt, you are helping other human beings. When you buy this shirt you not only donate the money for it but by wearing it you inspire people to do the same. One donation can make someone in the 28% of Birmingham that is impoverished live a little longer. You may not know what person will be getting the food bag from you, but you can still know it will help someone through the struggles life throws at them.<br><br>We need to fix this problem and we can't do it alone. We work together.`,
  },
  {
    name: `Karen Raymundo-Vega`,
    slug: `karen-raymundo-vega`,
    picture: `./assets/k-vega.webp`,
    projectName: `River Clean Up`,
    bgImage: `./assets/rivers.webp`,
    shirtFront: `./assets/karen-front.webp`,
    shirtBack: `./assets/karen-back.webp`,
    shortBlurb: `Karen’s mission is to make our future green. She will host cleanups and take before and after photos to put into a photography exhibition to bring awareness about our environment.`,
    desc: `Our mission is to make our future green by saving the environment. People pollute 14.5 trillion pounds of trash each year. This shirt will help our mission and save the earth. Karen Raymundo-Vega will do exhibitions, cleanups, and take before and after photos to show people how trash is affecting our Earth. It threatens species survival on this planet.<br/><br/> You can help by buying a shirt and donating to our cause. You can help spread awareness by wearing this shirt. People will see how bad this problem is and stop polluting. At the exhibitions you can see before and after photos. People will go to these exhibitions and see how bad this epidemic is, the epidemic of pollution. So please help spread awareness over this problem and help us solve it.<br/><br/> To hold a steady future for humanity and all life, no matter who you are, you can still help. So, help our cause and remember, our future is green.`,
  },
  {
    name: `Sarah Patrick Barze`,
    slug: `sarah-barze`,
    picture: `./assets/SP.webp`,
    projectName: `Paws, Tails, Talons, and Scales`,
    bgImage: `./assets/shelter.webp`,
    shirtFront: `./assets/sp-front.webp`,
    shirtBack: `./assets/sp-back.webp`,
    shortBlurb: `Sarah Patrick's mission is to help animals find loving homes where they are safe and protected. Our solution is asking for donations and fundraising by selling our shirts to help these animals.`,
    desc: `Our mission is rallying to help animals get adopted and put into good homes. We will do this by raising funds and donating money to shelters. We can prevent animals from being euthanized when it is not needed. With the money that we make, Sarah Patrick will provide the money for animals in shelters. Without your help we cannot achieve our goal of aiding these animals. We would appreciate it if you would devote yourself to this project.<br/><br/> By buying our shirt, you are giving to a great cause, along with owning a stylish T-shirt. This is important for us because it helps spread awareness. Please contribute to helping animals find good homes.`,
  },
];
```

Depending on the section of the page that needs content generated, I loop over it like this using `.map()`.

```js
students.map((student, i) => {
  const card = document.createElement("div");
  card.classList.add("shirt-card");
  card.innerHTML = `
            <div
              class="card-img"
              style="background-image: url('${student.bgImage}')"
            ></div>
            <div class="deets">
              <div class="frontmatter">
                <img src="${student.picture}" alt="${student.name}" />
                <div class="project-info">
                  <h3>${student.name}</h3>
                  <small>${student.projectName}</small>
                </div>
              </div>
              <p>
                ${student.shortBlurb}
                <a href="#${student.slug}" class="tertiary">Learn More &darr;</a>
              </p>
              <button
                class="snipcart-add-item card-btn"
                data-item-id="shirt-${student.slug}"
                data-item-price="28.00"
                data-item-url="https://altamont-school.github.io/miree-shirts/"
                data-item-description="A shirt supporting a project by Miree student ${student.name}."
                data-item-image="${student.shirtFront}"
                data-item-name="${student.projectName}"
                data-item-custom1-name="Size"
                data-item-custom1-options="YS|YM|YL|AS|AM|AL|AXL"
              >
                <img src="./assets/cart.svg" alt="Shopping Cart" /> Add to cart
              </button>
            </div>
  `;
  cards.appendChild(card);
});

```

I did run into **one** problem. As this content is being generated after the page loads, the crawler Snipcart uses couldn't find the right attributes on elements. So, I did have to hardcode some hidden elements at the bottom of the page like this:

```html
<!-- Hidden elements because we can't generate json -->
    <div style="display: none">
      <p
        class="snipcart-add-item"
        data-item-id="shirt-riley-pierce"
        data-item-price="28.00"
        data-item-url="https://altamont-school.github.io/miree-shirts/"
      ></p>
```

### Next time...

All in all, this was a fun project for numerous reasons! Most importantly, it felt good to help some excited students and some great non-profit projects. 

- If you're curious about our school, check out our [website](https://www.altamontschool.org/)
- If you'd like to see this site or support these students, [check it out](https://altamont-school.github.io/miree-shirts/)! No shipping and pick-up at the school, but feel free to donate 😉
- If you want to see the [source code](https://github.com/Altamont-School/miree-shirts/), have at it 🤙
