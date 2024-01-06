+++
layout = "post"
title = "The Lingual Odyssey"
author = "Nishant Joshi"
[extra]
    toc = true
+++

<br/>

---

> _The limit of my language mean the limit of my world._

In reality we often convay our thoughts via language. Each language has it's nuonces; each has it's own character set, each one has it's own grammar and each own is unique in how it is written/spoken. But at the end of the day, all these languages have a single purpose.

To provide us with a way to convay our thoughts, and to solve a very complex problem we called communication, in a very complex system we call society.

Computers aren't very different in that regard. Though, there might be a multitude of programming languages out there at the heart of it, they are all aimed to solve problems.

They could be:

- processualis (Procedural)
- कार्यात्मक (Functional)
- பொருள் நோக்கிய (Object Oriented)

But, at the end of the day, I am pretty sure you could do `1 + 1` in all of them. Or better yet, write a multi-user concurrent transaction handling system. Then, why do people make such a fuss around which language to learn and how JavaScript is "The next best thing since sliced bread".

But on a serious note, the real advantage of learning new languages, isn't just for solve these real world problems, but a different perspective to think, a different understanding of the moving parts involved and finally a innate appreciation towards the problems itself. "Everything you write will eventually turn to 0s and 1s", that's true, but the way you express it will not just run on your computers but also on the minds of the people who see it.

In my search to find this innate appreciation for problems, I set out on a journey, an advanture. [The advanture of code](https://adventofcode.com/). But, I didn't what is to be like all the general coding challenges out there. So, to spice things up, I planned on using different languages per problem. Don't worry, this isn't a write-up on how I solved the problems and don't work I won't give you any spoilers about them. This is my perspective how each of the languages that I used and the understanding I gained at the end of each day.

_<sub>PS. <br />Dear Internet,<br /> I would appreciate if you critique my implementations, but remember I am not claiminig that I have learnt all the languages, nor am I saying that all the implementations are perfect and can run so fast that they would travel back in time and alter reality. If that was the case, all my code would have magically become better.</sub>_
<br /><br />

The first thought on my mind as any programmer would have had was, "I want to code **Assembly**!", and so I did!

### Imperative Programming

How bad can it be to try and solve a complete problem in what I would call a purely Imperative Language. And believe me, it was hard, after finding a handful of insturtions that will help me solve the problem and 4 hours of head scratching and giving up a couple of times, I was finally able to solve the first part of the problem. But, guess what there was a second part to the problem and which required me to do string lookup, for frustrated soul couldn't handle implementing a hashmap for this, so I did the must imperative thing possible, and though ugly it worked.

```asm
check_numbers:
  call check_0
  call check_1
  call check_2
  call check_3
  call check_4
  call check_5
  call check_6
  call check_7
  call check_8
  call check_9
  ret
```

> _Rule #1: A good code is a code that works._

By the time, I had completed admiring the horrific beauty of my code, the next imperative language **COBOL**. I thought COBOL would be easy, and looking back at it, it probably was. The whole code, looked more or less like an essay, and of all the languages that I used, this had the most english in it. My guess is, it was used to document and give tasks to interns, before computers took their jobs.

> _Rule #2: A good code should be easier to read, regardless of the technical background._

While traveling the imperative path, there were a few languages that gave me a hard time, one of them was **Zig**, maybe it was just me, but working with a language where, any allocation that I need to do, needs to be done via a allocator object, and the worst of all, I wan't able to implement coupled recursion with 2 fallible functions. This was foreign to me, as no language has every asked me not to do recursion, except **Rust**, but that's a story for another day. The second language, that arrayed me out was **R**, I was more frustrated, coding in R then I was in any of the other languages, maybe its just me, but the way array constructions and concatenation worked, mangled my brain.

> _Rule #3: variable names are very important; we don't accept either `c` or `PerformArrayConcatenationWhichMightAlsoHelpInChangingArrayDimensions`, both are equally bad._

There were a lot of good programming languages along the way, **lua**, **ruby** with their simplicity and relatibility to perconcieved notion of programming languages, **Fortran**, **Go** with their inherently supported concurrency and robust design, the free flowing syntax and maturity of **Python**, **Julia**; and then there was **CoffeeScript**, me trying to curse this would be an insult to the curse words.

> _Rule #4: The freedom of telling the computer exactly what to do, has it's downsides. (it's like teaching a child, how to bake a cake). There are 3 outcomes (We get the cake, the house explodes, the child dies)_

As my imperative path reached its end, a new challenged was ready and waiting, it was finally time to cross the mountains of

### Object Oriented Programming

And well, I wasn't going to use Java, that was certain. So, why not start pure. But, I wasn't ready, the challenge sounded so grave, Me just like all my fellow introverted readers was faced with the challenge of a life time, **SmallTalk**. Or atleast so I thought, until in realized its just a programming language; a purly object oriented programming language. While researching on the language, I was baffled by the thought that there could be a programming language, that could just comprise of objects. While programming, I felt a but confounded and intrigued by the dataflow and at the same time excited about how it was different that what I was used to. Imagine a language where even the conditional statements work, as If you are passing messages between objects similar to how corrupt politicians pass money under the table to execute certain things on conditions.

> _Rule #5: Having functions doesn't mean everything has to be a functional._

As I was scaling the steep mountains of object oriented programming, I stepped upon something I used to call out to be, extremely ancient, weird and possibly something that I will never stumble upon, **PHP**! I always though PHP was this ancient relic from the past in which everything was difficult, gooey and a complete mess. But, turned out to be very different, I would say the first every PHP code that I have every written turned out to be extremely clear, consise and leveraging the complete power of OOP. It was definitely a exprerience worth remembering and it just gave me a proof that, "I should not judge a book, by their cover".

> _Rule #6: Let's try to package data, with all the constituent information contained within them._

I was planning to write some Java, but I knew the next best thing, and the final Object Oriented Obstacle **Scala**. And for all the people screaming that it's a functional programming language, no! the first word that I actually wrote when I started writing the code wasn't `fn`, `fun` or `def`. Guess what, it was `object`! Scala was an interesting language transaction, with my exprerience on Rust, Java, Python, in my workplace, I thought it would be a sinch, but I am not very proud of the code that actually came out. But, compared to a few other languages where I had to implement preliminary functionality, this was quiet a comfortable language.

> _Rule #7: Commented code, is worst than bad code. (It's similar to gossip, it poisons the mind of both the writer and the reader)._

Here we are folks, the final path in this great journey, and it's time to get wet in the river of

### Functional Programming

And what better way then to start pure, it was time to pull out the big guns, **Haskell**. I was planning to write my dream functional code, but dream are just dreams. I ended up writing dijkstra algorithm, with some of the most ugly looking functions that I have every writtne in any of the functional languages I know. It was more like functional vomit. But, nonetheless, that vomit functions.

> _Rule #8: Let's face it, "with more power comes more responsibility"._

But, to a larger extent these languages were very well built, and I was finally getting some help from my editor, otherwise in nearly all of the previous languages, either there wasn't a LSP or it was quite time consuming to set it up. In the challenge out of all the functional programming languages I used, I found **Ocaml** to be really great at doing what it is supposed to do.

- [x] Being Functional
- [x] Solve My Problems
- [x] Maximize Reusibility

And before you start saying every language, is reusable. I would like to say, "If you can read assembly, everything is open source.". Here, checkout this tweet, about how easy it was for me to build open my earlier solution to solve the part-2.

{{ x(name="lingual-odyssey") }}

> _Rule #9: Code should improves re-usability and facilitate easy refactoring for future code modifications._

After writing both, **Erlang** and OCaml, it felt like **Elixir** was their elicit baby. with function definitions like erlang has and operators similar to that of OCaml.

Though, there were many more languages that were part of the stretched out journey, these were some of the more interesting one's. And though, this is just a fraction of the learning that I found during this challenge, it was worth ever hardship and relentless duckduckgoing, that I did. But, the real hero of this journey isn't the forest of procedural programming, the mountains of object oriented programming or the river of functional programming, it is and always will be **Markdown**! Because, its the narrator who gets to tell the story not the warriers who can fight nanosecond wars, its the language that the blog is written in.

> _Rule #10: no code, is good code._

This was the journey, or atleast the glimpse of it, countless obstacles and numerous challenges and at the end of it all, the feeling of satisfactions and the fruit of determination was ever so sweet. The real challenge wasn't in solve all those problems, it was in following though with the chllenge relentlessly, without losing heart.

If you are atleast interested in looking over the code, scrutinising it, or just bitching about it. Feel free to check it out here:

- [NishantJoshi00/advent-of-code](https://github.com/NishantJoshi00/advent-of-code-2023)
  If you feel, like following me, here are some of my social handes:
- github.com/[@NishantJoshi](https://github.com/NishantJoshi00)
- x.com/[@nishantjosh](https://twitter.com/nishantjosh)
- linkedin.com/[@joshi-nishant](https://www.linkedin.com/in/joshi-nishant/)

If you are reached this far, it was valiant enough just to read though this whole mess. But remember what Kakashi said, "In society, those who don't have many ablities tend to complain more."
