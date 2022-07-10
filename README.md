# Klotski Solver

![License | MIT](https://img.shields.io/badge/License-MIT-orange)

## Description

Simple program made in Rust to solve Klotski.

[Klotski](https://en.wikipedia.org/wiki/Klotski) is a sliding puzzle game quite popular even though few know its name. In this game, the player has several blocks and has to move the biggest of them to the bottom center of the board. It has other names such as:

- _Huarong Dao_ or _華容道_ in Chinese
- _Daughter in the box_, _hakoiri musume_ or _箱入り娘_ in Japanese
- _L'âne rouge_ in French
- _Khun Chang Khun Phaen_ in Thai

I made this project in Rust to learn how to program with it, but also because I wasn't able to solve Klotski manually :)

The code is really fast when built with `--release` mode, but also thanks to the use of a simple A* search algorithm

## Installation and Program Execution

1. Clone this repository on your computer
2. Install `rustup`
3. Run `cargo run` or `cargo run --release`. After a short time, the solution will appear. You may need to change the `main()` and `solve_klotski()` functions in order to show all the steps between the initial state and the final solution.
