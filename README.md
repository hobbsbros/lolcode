# LolC

A simple Lolcode to C compiler.  Written in Rust.

# Installation Instructions

`lolc` requires an up-to-date installation of Cargo.

`cargo install lolcode`

# What is Lolcode?

Lolcode is an *esolang*, or esoteric programmming language.  Its syntax is intentionally humorous and makes it difficult to use in production use.  Subsequently, Lolcode is used exclusively as a joke in the software development community.

# Why does this exist?

I developed this Lolcode compiler to help solidify an understanding of Pratt parsing.  The parser for Lolcode is based on the principles of Pratt parsing.

Credit to [Bob Nystrom at Stuff With Stuff](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/) for a really great article that helps explain Pratt parsers.  Nystrom uses Java in his implementation (and I use Rust here) but he explains every step very clearly.

Credit to [Austin Henley](https://austinhenley.com/blog/teenytinycompiler1.html) for a great guide on writing compilers.

Credit to [Revanth Pothukuchi (GitHub user Hacker007)](https://github.com/Hacker-007/envious) for explaining to me the parser in his language Envious's compiler, `envyc`.

# Why write it in Rust?

I decided to write this in Rust for a few reasons:
- Effective & safe type polymorphism
- Effective error handling
- Speed
- I know Rust much better than C++