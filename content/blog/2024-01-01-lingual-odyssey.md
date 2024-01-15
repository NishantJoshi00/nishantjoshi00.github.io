+++
layout = "post"
title = "Advent of Code: The Lingual Odyssey"
author = "Nishant Joshi"
[extra]
    toc = true
+++

<br/>

---

> _The limit of my language means the limit of my world._

<br/>

In reality, we often convey our thoughts through languages. Each language has its own nuances, character set, grammar, and unique expression. But at the end of the day, all these languages share a singular purpose: to provide a means of communicating our thoughts and navigating the complexities of a system we call society.

To provide us with a way to convey our thoughts, and to solve a very complex problem we call communication, in a very complex system we call society. Computers aren't very different in that regard. Though there might be a variety of programming languages out there, at the heart of it, they are all aimed at solving problems.

They could be:

- processualis (Procedural)
- कार्यात्मक (Functional)
- பொருள் நோக்கிய (Object Oriented)

<br/>

But, at the end of the day, I'm pretty sure you could do `1 + 1` in all of them. Or, better yet, write a multi-user concurrent transaction handling system. Then, why are people chasing new languages (like **Rust**) and talking about how JavaScript is "the next best thing since sliced bread"?

But on a serious note, the real advantage of learning new languages isn't just to solve these real-world problems, but to gain a different perspective to think, a different understanding of the moving parts involved, and finally, an innate appreciation towards the problems themselves. "Everything you write will eventually turn to 0s and 1s"; that's true, but the way you express it will not just run on your computers, but also on the minds of the people who see it.

I set out on a journey, an adventure: [The Adventure of Code](https://adventofcode.com/). But I didn't want it to be like all the general coding challenges out there. So, to spice things up, I planned on using different languages per problem, and to top that I set a self-imposed condition that I will only use the languages that I don't already know!

<br/>

> _Talk is cheap. Show me the code._ Here you go [github:advent-of-code-2023](https://github.com/NishantJoshi00/advent-of-code-2023)!

<br/>

The first thought on my mind as any programmer would have had was, "I want to code **Assembly**!", and so I did!

<br/>

## Imperative Programming

How bad can it be to try and solve a complete problem in what I would call a purely Imperative Language? And believe me, it was hard. After finding a handful of instructions that would help me solve the problem and 4 hours of head-scratching and giving up a couple of times, I was finally able to solve the first part of the problem. But guess what? There was a second part to the problem that required me to do a string lookup. My frustrated soul couldn't handle implementing a hashmap for this, so I did the most imperative thing possible, and though ugly, it worked.

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

<br/>

> _Rule #1: Good code is code that works._

<br/>

By the time I had completed admiring the horrific beauty of my code, the next imperative language was **COBOL**. I thought COBOL would be easy, and looking back, it probably was. The whole code looked more or less like an essay, and of all the languages that I used, this had the most English in it. My guess is, it was used for documentation or offloading work to interns.

<br/>

> _Rule #2: A good code should be easier to read, regardless of the technical background._

<br/>

While traveling the imperative path, there were a few languages that threw me for a loop (literally in Zig's case). One of them was **Zig**. Maybe it was just me, but working with a language where any allocation had to be hand-wrung from an allocator object felt like walking a memory tightrope. And the worst of it all? I couldn't even build a bridge of coupled recursion with two fallible functions – Zig wouldn't let me! This was completely unexpected, like facing a "no recursion allowed" sign after years of coding freedom. No other language had ever tried to clip my recursion wings – except maybe Rust, but that's a story for another day. The second language that really threw me off was **R**. I've never been more frustrated than when wrestling with R's array constructions and concatenation. Maybe it's just me, but the way everything fit together felt like trying to solve a Rubik's cube blindfolded – it was frustrating...

<br/>

> _Rule #3: variable names are very important; we don't accept either `c` or `PerformArrayConcatWithVaryingDims`, both are equally bad._

<br/>

There were a lot of good programming languages along the way: **Lua** and **ruby** with their simplicity and relatability to preconceived notions of programming languages; Fortran and Go with their inherently supported concurrency and robust design; and the free-flowing syntax and maturity of Python and Julia. And then there was CoffeeScript... me trying to curse it would be an insult to the curse words themselves.

<br/>

> _Rule #4: The freedom of telling the computer exactly what to do, has its downsides. (it's like teaching a child, how to bake a cake). There are 3 outcomes (We get the cake, the house explodes, the child dies)_

<br/>

As my imperative path reached its end, a new challenge awaited—it was finally time to cross the mountains of...

<br/>

## Object-Oriented Programming

Java was out of the question, that much was certain. I wanted to start pure. But was I truly ready? The challenge loomed before me, a daunting prospect for any introvert—**SmallTalk**. Or at least, so I thought until I realized it was just a programming language. A purely object-oriented programming language, to be precise.

During my research, the concept of a language composed entirely of objects boggled my mind. Then, as I delved into the programming itself, confusion mingled with intrigue as I grappled with the unorthodox dataflow (this sentence literally represents my thoughts from back then). Yet, I was somehow excited. This was unlike anything I'd encountered before. Imagine a language where even conditional statements operate through messages passed between objects like corrupt politicians slipping bribes under the table to get things done.

<br/>

> _Rule #5: Having functions doesn't mean everything has to be functional._

<br/>

As I scaled the steep mountains of object-oriented programming, I stumbled upon something I'd always dismissed as extremely ancient, weird, and unlikely to cross my path: **PHP**! I'd always envisioned PHP as a relic from the past, a realm of difficulty, gooiness, and utter chaos. But reality proved vastly different. The first PHP code I ever wrote turned out to be remarkably clear, concise, and fully embraced the power of OOP. It was an experience worth remembering, a testament to the age-old wisdom of "not judging a book by its cover."

It might also be the preconceived notion that PHP has a bad language design and will always stay that way. Moreover, it also felt that the language has evolved to be more robust and easy to adapt to. To put it simply, I didn't feel like I was learning a new language, but just using the built-up experience.

<br/>

> _Rule #6: Let's try to package data, with all the constituent information contained within them._

<br/>

I'd planned to write some Java, but I knew the next best thing—and the final Object-Oriented Obstacle—Scala. And for all the people out there screaming that it's a functional programming language, I have news for you: the first word I wrote when I started coding wasn't fn, fun, or def. It was object!

Scala presented an interesting language transition. With my experience in Rust, Java, and Python at work, I thought it would be a cinch. But I'm not exactly proud of the code I produced. Still, compared to a few other languages where I'd had to implement preliminary functionality, Scala felt quite comfortable.

<br/>

> _Rule #7: Awry commented code is worse than bad code. (It's similar to gossip, it poisons the mind of both the writer and the reader)._

<br/>

Here we are, folks—the final path in this grand journey. It's time to dive into the river of...

<br/>

## Functional Programming

And what better way to start than pure? It's time to pull out the big guns: **Haskell**. I'd envisioned writing my dream functional code, but dreams are just dreams. I ended up with a Dijkstra algorithm sporting some of the ugliest functions I've ever crafted in any functional language. It was more like functional vomit. But hey, that vomit functions.

<br/>

> _Rule #8: Let's face it, "with great power comes great responsibility"._

<br/>

But to a larger extent, these languages were very well-built, and I was finally getting some much-needed help from my editor. In nearly all of the previous languages, either there wasn't an LSP or it was quite time-consuming to set one up. In this challenge, out of all the functional programming languages I used, I found **Ocaml** to be truly exceptional at fulfilling its purpose.

- [x] Being Functional
- [x] Solve My Problems
- [x] Maximize Reusability

<br/>

And before you start saying every language is reusable, sure, "If you can read assembly, everything is open source," but who has time for that? I crave languages that embrace reusability from the start. OCaml delivered in a big way. Don't believe me? Witness the magic...

{{ x(name="lingual-odyssey") }}

<br/>

> _Rule #9: Code should improve re-usability and facilitate easy refactoring for future code modifications._

<br/>

After wrestling with both **Erlang** and OCaml, **Elixir** felt like their love child, inheriting function definitions from Erlang and operators from OCaml.

My journey stretched across countless languages, but these were some of the most captivating detours. While this is just a sliver of the learning I gleaned, it was worth every hurdle and relentless DuckDuckGo-ing session. In the end, the true hero of this odyssey wasn't the procedural forest, the object-oriented mountains, or the functional river, but Markdown! It's the narrator, unlike the nanosecond warriors – markdown's the language this very blog breathes in.

> _Rule #10: no code, is good code._

<br/>

This was the journey, or at least a glimpse of it, littered with countless obstacles and daunting challenges. Yet, at the end, the sweetness of satisfaction and the fruit of unwavering determination were beyond compare. The real challenge wasn't solving all those problems; but rather, being consistent and not giving up on the principles that I set for myself. (Well! I did solve the last problem in Python, but in my defense, finding eigenvectors for Laplacian matrix would've been hella difficult in **BrainFuck**.)

{{ count_code(owner="NishantJoshi00", repo="advent-of-code-2023") }}

<br/>

If you are interested in scrutinizing my code, or wish to talk smack about it. Feel free to check it out here:

- [NishantJoshi00/advent-of-code](https://github.com/NishantJoshi00/advent-of-code-2023)

If you feel like following me or sharing your own ideas around the advent of code, here are some of my social media handles:

- github.com/[@NishantJoshi](https://github.com/NishantJoshi00)
- x.com/[@nishantjosh](https://twitter.com/nishantjosh)

If you have reached this far, it was valiant enough just to read through this whole mess. But remember what Kakashi said, "In society, those who don't have many abilities tend to complain more."
