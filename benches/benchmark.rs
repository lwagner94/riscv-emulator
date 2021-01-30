extern crate riscv_emu;

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};


use riscv_emu::instruction::WrappedInstruction;
use std::fs::File;
use std::io::Read;
use riscv_emu::util::{read_u32_from_byteslice, read_u32_from_byteslice_fast};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn instruction_decode(c: &mut Criterion) {
    let length = 64;

    let mut buffer = vec![0u8; length];
    let mut file = File::open("benches/files/main.bin").unwrap();
    file.read_exact(buffer.as_mut_slice()).unwrap();

    let mut group = c.benchmark_group("instruction_decode");

    group.throughput(Throughput::Bytes(buffer.len() as u64));
    group.bench_function("64 bytes", |b| b.iter(|| {
        for i in (0..length).step_by(4) {
            let code = read_u32_from_byteslice(&buffer[i..i + 4]);
            WrappedInstruction::new(code);
        }
    }
    ));
}

fn read_byteslice(c: &mut Criterion) {
    let mut buffer = vec![1,2,3,4];
    let mut group = c.benchmark_group("read_u32_from_byteslice");

    group.throughput(Throughput::Bytes(4 as u64));
    group.bench_function("slow", |b| b.iter(|| {
        let code = read_u32_from_byteslice(&buffer[0..0 + 4]);
        black_box(code);
    }));
    group.bench_function("fast", |b| b.iter(|| {
        let code = read_u32_from_byteslice_fast(&buffer[0..0 + 4]);
        black_box(code);
    }));
}

criterion_group!(benches, instruction_decode, read_byteslice);
criterion_main!(benches);
