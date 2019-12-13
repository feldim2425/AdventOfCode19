# Advent of Code < 2019 >

[![Advent of Code](https://img.shields.io/badge/Day-13-blue?style=for-the-badge)](https://adventofcode.com/2019/) [![Rust](https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org) [![Join Trojaners Discord](https://img.shields.io/badge/Discord-Trojaner's%20Discord-blue?style=for-the-badge&logo=Discord)](https://discord.gg/qhHm8rM)

Not much to say here just go to the [AdventOfCode 2019 Website](https://adventofcode.com/2019/) and check out the puzzles. There is a new one each day that you have to solve with programming and get points.

I choose to use [Rust](https://www.rust-lang.org). Not because it's my favorite language (I have none), or that it is very difficult or anything challanging in general, but it's a language I want to learn for a long time, and finally found a reason to do so.


## Testing out my code
First run ``cargo update`` to install the dependencies.

After installing the dependencies you can simply run ``cargo run --bin dayXX`` (Replacing XX with a number from 1-25).
You can also just build the binary with ``cargo build --bin dayXX``, the built binary is stored in ``target/debug/dayXX``

The debug binaries are a bit slow sometimes. You can add ``--release`` to build/run the optimized version. If you build a release binary it will be stored in ``target/release/dayXX``.

On systems that support the bash syntax (Linux, MacOS, Linux System for Windows, git bash, ...) you can also run the ``run_all_days.sh`` shell script. The script will build all days (with the release target) and run them.

## Disclaimer
The solutions here are implemented me, they might not be the best solution possible.

If I get stuck on a puzzle I will look in other solutions, but I will still implement them myself after I understood the concept.

After I implemented my version I will also try and optimize it even with inspiration from other solution, but the same rule applies:
I will implement it myself when I can understand, how and why that optimization works, otherwise I won't use it.

I will also mark solutions / optimizations inspired by other ideas.

## Other solutions
+ Trojaner's [AdventofCode](https://github.com/TrojanerHD/AdventofCode) in JavaScript / NodeJS
+ LeMoonStar's [AdventOfCode2019Solutions](https://github.com/LeMoonStar/AdventOfCode2019Solutions) in GoLang
+ DCDragonCoder's [AdventOfCode2019](https://github.com/DragonCoder01/AdventOfCode2019) in C++
+ Hax's [advent-of-code](https://github.com/Schlauer-Hax/advent-of-code) in Java
+ derNiklaas' [Advent-of-Code-2019](https://github.com/derNiklaas/Advent-of-Code-2019) in Java
+ networkException's [AdventOfCode](https://github.com/dejakobniklas/AdventOfCode) in Java
+ derKaländer's [AdventOfCode](https://github.com/derkalaender/AdventOfCode) in Kotlin
+ Daan Breur's [AdventOfCode2019](https://github.com/daanbreur/AdventofCode2019) in Javascript / NodeJS
+ 1Turtle's (aka. Sammy) [AdventOfCode-for-Computercraft](https://github.com/1Turtle/AdventOfCode-for-Computercraft) in Lua / Minecraft ComputerCraft Mod
