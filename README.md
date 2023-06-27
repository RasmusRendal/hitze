# Hitze
This is meant to be a JIT interpreter for Brainfuck. If you do not feel like crying today, I would suggest you read some other code instead.

## TODO
 - [ ] Add support for input
 - [ ] Add support for ! to append input
 - [ ] Integrate some sort of fuzzing
 - [ ] Profiling, adaptive optimization

## Benchmarks
I have benchmarked this interpreter against [bff4](http://mazonka.com/brainf/), [fucker](https://github.com/danthedaniel/BF-JIT/), and [rdebath's Brainfuck interpreter](https://github.com/rdebath/Brainfuck).

| Benchmark | bff4 | fucker | bf | hitze (no jit) | hitze (no optimizations) | hitze (no jit or optimizations) | hitze |
------------|------|--------|----|----------------|--------------------------|---------------------------------|--------
| mandelbrot.bf | 0:03.34 | 0:00.55 | 0:00.43 | 0:07.29 | 0:00.75 | 0:07.76 | 0:00.54 |
| long.bf | 0:01.39 | 0:00.56 | 0:00.00 | 0:02.50 | 0:02.97 | 0:14.46 | 0:00.18 |
