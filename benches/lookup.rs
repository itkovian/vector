use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use indexmap::map::IndexMap;
use std::convert::TryFrom;
use std::{fs, io::Read, path::Path};
use vector::event::Lookup;

const FIXTURE_ROOT: &str = "tests/data/fixtures/lookup";

fn parse_artifact(path: impl AsRef<Path>) -> std::io::Result<String> {
    let mut test_file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut buf = Vec::new();
    test_file.read_to_end(&mut buf)?;
    let string = String::from_utf8(buf).unwrap();
    Ok(string)
}

// This test iterates over the `tests/data/fixtures/lookup` folder and ensures the lookup parsed,
// then turned into a string again is the same.
fn lookup_to_string(c: &mut Criterion) {
    vector::test_util::trace_init();
    let mut fixtures = IndexMap::new();

    std::fs::read_dir(FIXTURE_ROOT)
        .unwrap()
        .for_each(|fixture_file| match fixture_file {
            Ok(fixture_file) => {
                let path = fixture_file.path();
                tracing::trace!(?path, "Opening.");
                let buf = parse_artifact(&path).unwrap();
                fixtures.insert(path, buf);
            }
            _ => panic!("This test should never read Err'ing test fixtures."),
        });

    let mut group_from_elem = c.benchmark_group("from_string");
    for (_path, fixture) in fixtures.iter() {
        group_from_elem.throughput(Throughput::Bytes(fixture.clone().into_bytes().len() as u64));
        group_from_elem.bench_with_input(
            BenchmarkId::from_parameter(&fixture),
            &fixture.clone(),
            move |b, ref param| {
                let input = &(*param).clone();
                b.iter_with_setup(
                    || input.clone(),
                    |input| {
                        let lookup = Lookup::try_from(input).unwrap();
                        black_box(lookup)
                    },
                )
            },
        );
    }
    group_from_elem.finish();

    let mut group_to_string = c.benchmark_group("to_string");
    for (_path, fixture) in fixtures.iter() {
        group_to_string.throughput(Throughput::Bytes(fixture.clone().into_bytes().len() as u64));
        group_to_string.bench_with_input(
            BenchmarkId::from_parameter(&fixture),
            &fixture.clone(),
            move |b, ref param| {
                let input = &(*param).clone();
                b.iter_with_setup(
                    || Lookup::try_from(input.clone()).unwrap(),
                    |input| {
                        let string = input.to_string();
                        black_box(string)
                    },
                )
            },
        );
    }
    group_to_string.finish();

    let mut group_serialize = c.benchmark_group("serialize");
    for (_path, fixture) in fixtures.iter() {
        group_serialize.throughput(Throughput::Bytes(fixture.clone().into_bytes().len() as u64));
        group_serialize.bench_with_input(
            BenchmarkId::from_parameter(&fixture),
            &fixture.clone(),
            move |b, ref param| {
                let input = &(*param).clone();
                b.iter_with_setup(
                    || Lookup::try_from(input.clone()).unwrap(),
                    |input| {
                        let string = serde_json::to_string(&input);
                        black_box(string)
                    },
                )
            },
        );
    }
    group_serialize.finish();

    let mut group_deserialize = c.benchmark_group("deserialize");
    for (_path, fixture) in fixtures.iter() {
        group_deserialize.throughput(Throughput::Bytes(fixture.clone().into_bytes().len() as u64));
        group_deserialize.bench_with_input(
            BenchmarkId::from_parameter(&fixture),
            &fixture.clone(),
            move |b, ref param| {
                let input = &(*param).clone();
                b.iter_with_setup(
                    || serde_json::to_string(&Lookup::try_from(input.clone()).unwrap()).unwrap(),
                    |input| {
                        let lookup: Lookup = serde_json::from_str(&input).unwrap();
                        black_box(lookup)
                    },
                )
            },
        );
    }
    group_deserialize.finish();
}

criterion_group!(lookup, lookup_to_string);
criterion_main!(lookup);
