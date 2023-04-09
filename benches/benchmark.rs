use criterion::{black_box, criterion_group, criterion_main, Criterion};
use yahvm::vm;

fn run_vm(filename: &str) {
  let mut vm = vm::Vm::default();
  vm.load_bin(filename);
  vm.exec();
}

pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function(
    "simple vm",
    |b| b.iter(||
      run_vm(black_box("test/basic_test.bin"))
    )
  );

  c.bench_function(
    "jump",
    |b| b.iter(||
      run_vm(black_box("test/jump_test.bin"))
    )
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);