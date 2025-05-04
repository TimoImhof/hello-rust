I want to learn something new and challenge myself. I'm completely new to Rust and want to learn it as a second side project next to maintaining [adapters](https://github.com/adapter-hub/adapters).

First I will complete a couple of tutorials to get comfortable with writing code in Rust. The longterm goal will be to build an inference server for LLM deployment from scratch. As I spend more time with adapters, I am becoming more and more interested in the full engineering cycle of building and also SERVING machine learning models.

Therefore I think that by learning Rust I can start building up another view on the problem machine learning from a systems programming and performance optimization standpoint, which hopefully will complement my Python/PyTorch standpoint.

This repository will document my journey from Rust beginner to building a complete LLM inference server.

If you should stumble upon this repo and have constructive feedback on anything I am more than happy to include it!
So I encourage you to open a new issue and explain to me what I am doing wrong! :D

## Structure

For now I use this repo as a Rust workspace (as explained [here](https://medium.com/@aleksej.gudkov/rust-workspace-example-a-guide-to-managing-multi-crate-projects-82d318409260)). The [Rust By Example guides](https://doc.rust-lang.org/rust-by-example/index.html) start with `rbe_xx`; chapters of the [book](https://doc.rust-lang.org/book/title-page.html) are marked by `b_xx`.