use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hitze::compiler::compile;
use hitze::optimizer::optimize;
use hitze::parser::parse;

fn run_brainfuck(code: &str) -> i64 {
    let mut code = parse(code);
    optimize(&mut code);
    let program = compile(&code);
    let mut memory: Vec<u8> = vec![0; u16::max_value() as usize + 1];
    program.run(memory.as_mut_slice())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("long", |b| b.iter(|| run_brainfuck(black_box(">+>+>+>+>++<[>[<+++>-+>+>+>+>++<[>[<+++>-+>+>+>+>++<[>[<+++>-+>+>+>+>++<[>[<+++>-+++[->+++++<]>[-]<<<<<<]<<]>[-]<<<<<]<<]>[-]<<<<<]<<]>[-]<<<<<]<<]>"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
