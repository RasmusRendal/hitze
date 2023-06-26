//use criterion::{criterion_group, criterion_main, Criterion};
use hitze::compiler::compile;
use hitze::optimizer::optimize;
use hitze::parser::parse;
use iai::{black_box, main};

fn parse_optimize_compile_run(program: &str) -> i64 {
    let mut code = parse(program);
    optimize(&mut code);
    let program = compile(&code);
    let mut memory: Vec<u8> = vec![0; u16::max_value() as usize + 1];
    program.run(memory.as_mut_slice())
}

fn bench_long() -> i64 {
    parse_optimize_compile_run(black_box(include_str!("../examples/long.bf")))
}

fn bench_hello_world() -> i64 {
    parse_optimize_compile_run(black_box(include_str!("../examples/hello_world.bf")))
}

fn bench_hello_world_golf() -> i64 {
    parse_optimize_compile_run(black_box(include_str!("../examples/hello_world_golf.bf")))
}

fn bench_mandelbrot() -> i64 {
    parse_optimize_compile_run(black_box(include_str!("../examples/mandelbrot.bf")))
}

main!(
    bench_long,
    bench_hello_world,
    bench_hello_world_golf,
    bench_mandelbrot
);
