use blink_pairs::parser::{
    indent::indent_levels,
    languages::{Rust, C},
    parse_filetype,
    tokenize::tokenize,
    Matcher, State,
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

fn criterion_benches(c: &mut Criterion) {
    let c_text: &str = include_str!("./languages/c.c");
    let rust_text: &str = include_str!("./languages/rust.rs");

    let c_lines = c_text.lines().collect::<Box<[_]>>();
    let rust_lines = rust_text.lines().collect::<Box<[_]>>();

    c.bench_function("tokenize simd - c", |b| {
        b.iter(|| black_box(tokenize(black_box(c_text.as_bytes()), black_box(C::TOKENS))))
    });

    c.bench_function("tokenize simd - rust", |b| {
        b.iter(|| {
            black_box(tokenize(
                black_box(rust_text.as_bytes()),
                black_box(Rust::TOKENS),
            ))
        })
    });

    c.bench_function("parse simd - c", |b| {
        b.iter(|| parse_filetype("c", 4, black_box(&c_text), State::Normal))
    });

    c.bench_function("parse simd - rust", |b| {
        b.iter(|| parse_filetype("rust", 4, black_box(&rust_text), State::Normal))
    });

    c.bench_function("indent - c", |b| {
        b.iter(|| indent_levels(black_box(&c_lines), 4))
    });

    c.bench_function("indent - rust", |b| {
        b.iter(|| indent_levels(black_box(&rust_lines), 4))
    });
}

criterion_group!(benches, criterion_benches);
criterion_main!(benches);
