use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::fs;
use growing_array::{process_growing_array, process_growing_array_heap};

fn bench_with_input(c: &mut Criterion, filename: &str) {
    let input = fs::read_to_string(filename).expect("Failed to read input file");

    let mut group = c.benchmark_group("Processing Arrays");

    group.bench_with_input(BenchmarkId::new("process_growing_array", filename), &input, |b, i| {
        b.iter(|| process_growing_array(i));
    });

    group.bench_with_input(BenchmarkId::new("process_growing_array_heap", filename), &input, |b, i| {
        b.iter(|| process_growing_array_heap(i));
    });

    group.finish();
}

fn benchmarks(c: &mut Criterion) {
    let path = "tests/inout/";

    // Collect all *.in files and sort them numerically
    let mut files: Vec<String> = fs::read_dir(path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if file_name.ends_with(".in") {
                Some(file_name)
            } else {
                None
            }
        })
        .collect();

    files.sort_by_key(|f| f.trim_end_matches(".in").parse::<usize>().unwrap_or(0));

    for file in files {
        let filename = format!("{}/{}", path, file);
        bench_with_input(c, &filename);
    }
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);

