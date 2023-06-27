//use criterion::{criterion_group, criterion_main, Criterion};
use hitze::runner::run;
use iai::{black_box, main};

fn bench_long() {
    run(black_box(include_str!("../examples/long.bf")));
}

fn bench_hello_world() {
    run(black_box(include_str!("../examples/hello_world.bf")));
}

fn bench_hello_world_golf() {
    run(black_box(include_str!("../examples/hello_world_golf.bf")));
}

fn bench_mandelbrot() {
    run(black_box(include_str!("../examples/mandelbrot.bf")));
}

main!(
    bench_long,
    bench_hello_world,
    bench_hello_world_golf,
    bench_mandelbrot
);
