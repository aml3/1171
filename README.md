1171
====

A Rust 0.8 implementation based on [Ken Thompson's construction algorithm](http://en.wikipedia.org/wiki/Thompson's_construction_algorithm). It converts a regex into an NFA and then runs strings through the automaton.

This repo's title references this [xkcd comic](http://m.xkcd.com/1171).

Currently supports \*, +, ., ?, (), |, [], [^], and escaped (\\) characters.
