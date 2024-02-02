use std::{fs::read_dir, time::Duration};

use aarty::{convert_image_to_ascii, Config, Sympols, COLORS, REVERSE};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn convert_to_ascii_with_colors(c: &mut Criterion) {
    let mut images = Vec::new();

    for entry in read_dir("samples").unwrap() {
        let path = entry.unwrap().path();
        images.push((path.clone(), image::open(path).unwrap()));
    }

    macro_rules! group {
        ($name: expr, $config: expr) => {{
            let mut group = c.benchmark_group($name);
            for (path, image) in &images {
                group.bench_with_input(
                    BenchmarkId::from_parameter(path.to_str().unwrap()),
                    image,
                    |b, image| {
                        let mut null = std::io::sink();
                        b.iter(|| convert_image_to_ascii($config, image, &mut null).unwrap());
                    },
                );
            }
            group.finish();
        }};
    }

    group!(
        "convert with no colors",
        &Config::new(vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into())
    );

    group!(
        "convert with foeground colors",
        &Config::new(vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into())
            .with_flags(COLORS)
    );

    group!(
        "convert with foeground colors reversed",
        &Config::new(vec![' ', '.', ',', '-', '~', '!', '*', '%', '$', '@', '#'].into())
            .with_flags(COLORS | REVERSE)
    );

    group!(
        "convert with foeground colors reversed and empty set",
        &Config::new(Sympols::empty()).with_flags(COLORS | REVERSE)
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).with_plots().warm_up_time(Duration::from_millis(200));
    targets = convert_to_ascii_with_colors
}
criterion_main!(benches);
