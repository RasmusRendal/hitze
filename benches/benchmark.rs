//use criterion::{criterion_group, criterion_main, Criterion};
use hitze::runner;
use iai::{black_box, main};

fn bench_long() {
    runner::run(
        black_box(include_str!("../examples/long.bf")),
        runner::DEFAULT_COMPILE_DEPTH,
    );
}

fn bench_hello_world() {
    runner::run(
        black_box(include_str!("../examples/hello_world.bf")),
        runner::DEFAULT_COMPILE_DEPTH,
    );
}

fn bench_hello_world_golf() {
    runner::run(
        black_box(include_str!("../examples/hello_world_golf.bf")),
        runner::DEFAULT_COMPILE_DEPTH,
    );
}

fn bench_mandelbrot() {
    runner::run(
        black_box(include_str!("../examples/mandelbrot.bf")),
        runner::DEFAULT_COMPILE_DEPTH,
    );
}

main!(
    bench_long,
    bench_hello_world,
    bench_hello_world_golf,
    bench_mandelbrot
);
