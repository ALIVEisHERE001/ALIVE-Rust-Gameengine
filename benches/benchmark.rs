// Performance benchmarks for gameengine
#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks {
    use super::*;
    use test::Bencher;
    
    #[bench]
    fn bench_core_operation(b: &mut Bencher) {
        b.iter(|| {
            // Benchmark core operation
            perform_operation()
        });
    }
    
    #[bench]
    fn bench_with_large_dataset(b: &mut Bencher) {
        let data = vec![1; 10000];
        b.iter(|| {
            process_data(&data)
        });
    }
    
    #[bench]
    fn bench_memory_allocation(b: &mut Bencher) {
        b.iter(|| {
            let _v: Vec<i32> = Vec::with_capacity(1000);
        });
    }
}

fn perform_operation() -> i32 {
    42
}

fn process_data(data: &[i32]) -> i32 {
    data.iter().sum()
}
