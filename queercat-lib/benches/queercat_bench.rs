use criterion::*;
use queercat_lib::*;

use std::io::{sink, Cursor, Seek, Write};
const DATASET_A: &'static str = include_str!("bench-data/a");
const DATASET_CYRILLIC: &'static str = include_str!("bench-data/cyrillic");
const DATASET_ESCAPE: &'static str = include_str!("bench-data/escape");
const DATASET_GLITCH: &'static str = include_str!("bench-data/glitch");
const DATASET_QUEERCAT_SRC: &'static str = include_str!("bench-data/queercat");
const DATASET_QUEERCAT_SRC_PP: &'static str = include_str!("bench-data/queercat-pp");

const TEST_FLAGS: [Flag<'_>; 5] = [transgender(), lesbian(), gay(), aroace(), rainbow()];

fn create_queercat_24bit<'a>(flag: &Flag<'a>) -> QueerCat<'a, Box<dyn Write>, Bits24> {
    let colorizer = Bits24::new(QueerCatFrequency::Original);
    QueerCat::new(colorizer, Box::new(black_box(sink())), flag.clone())
}

fn create_queercat_ansi<'a>(flag: &Flag<'a>) -> QueerCat<'a, Box<dyn Write>, Ansi> {
    let colorizer = Ansi::new(flag.ansi_colors.len() as u32, QueerCatFrequency::Original);
    QueerCat::new(colorizer, Box::new(black_box(sink())), flag.clone())
}

fn do_24bit(c: &mut Criterion, dataset_name: &'static str, dataset: &'static str) {
    let mut group = c.benchmark_group(dataset_name);
    let mut data = Cursor::new(dataset.as_bytes());
    group.throughput(Throughput::Bytes(dataset.len() as u64));

    for flag in TEST_FLAGS {
        let mut cat = create_queercat_24bit(&flag);
        group.bench_function(BenchmarkId::new(flag.name, "24 bit"), |b| {
            b.iter(|| {
                cat.cat(black_box(&mut data)).unwrap();
                data.rewind()
            })
        });
        data.rewind().unwrap()
    }
    group.finish()
}

fn do_ansi(c: &mut Criterion, dataset_name: &'static str, dataset: &'static str) {
    let mut group = c.benchmark_group(dataset_name);
    let mut data = Cursor::new(dataset.as_bytes());
    group.throughput(Throughput::Bytes(dataset.len() as u64));

    for flag in TEST_FLAGS {
        let mut cat = create_queercat_ansi(&flag);
        group.bench_function(BenchmarkId::new(flag.name, "ansi"), |b| {
            b.iter(|| {
                cat.cat(black_box(&mut data)).unwrap();
                data.rewind()
            })
        });
        data.rewind().unwrap()
    }
    group.finish()
}

fn bench(c: &mut Criterion) {
    for (dataset_name, dataset) in [
        ("glitch", DATASET_GLITCH),
        ("a", DATASET_A),
        ("cyrillic", DATASET_CYRILLIC),
        ("escape", DATASET_ESCAPE),
        ("queercat-src", DATASET_QUEERCAT_SRC),
        ("queercat-src-pp", DATASET_QUEERCAT_SRC_PP),
    ] {
        do_ansi(c, dataset_name, dataset);
        do_24bit(c, dataset_name, dataset);
    }
}

fn main() {
    let mut c = Criterion::default()
        .configure_from_args()
        .with_plots()
        .measurement_time(std::time::Duration::new(5, 0))
        .significance_level(0.01); // alpha = 0.01
    bench(&mut c);
    c.final_summary();
}
