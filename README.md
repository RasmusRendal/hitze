# Hitze
This is meant to be a JIT interpreter for Brainfuck. If you do not feel like crying today, I would suggest you read some other code instead.

## TODO
 - [ ] Add support for input
 - [ ] Add support for ! to append input
 - [ ] Integrate some sort of fuzzing
 - [ ] Profiling, adaptive optimization

## Benchmarks
I have benchmarked this interpreter against [bff4](http://mazonka.com/brainf/), [fucker](https://github.com/danthedaniel/BF-JIT/), and [rdebath's Brainfuck interpreter](https://github.com/rdebath/Brainfuck).

| Benchmark | bff4 | fucker | bf | hitze |
------------|------|--------|----|--------
| mandelbrot.bf | 0:04.13 | 0:00.68 | 0:00.55 | 0:00.66 |
| long.bf | 0:01.67 | 0:00.69 | 0:00.01 | 0:00.22 |
