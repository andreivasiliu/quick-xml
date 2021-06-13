extern crate quick_xml;
extern crate criterion;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quick_xml::events::Event;
use quick_xml::Reader;

// To compare against a baseline:
// `cargo bench -- --save-baseline <name>`
// `cargo bench -- --baseline <name>`
//
// To get output similar to Rust's nightly libtest, usable with benchcmp:
// `cargo bench -- --output-format bencher`
//
// Criterion saves more details at: ./target/criterion/index.html

fn bench_quick_xml_normal(c: &mut Criterion) {
    let src: &[u8] = include_bytes!("../tests/sample_rss.xml");
    c.bench_function("quick_xml_normal", |b| b.iter(|| {
        let mut r = Reader::from_reader(src);
        r.check_end_names(false).check_comments(false);
        let mut count = black_box(0);
        let mut buf = Vec::new();
        loop {
            match r.read_event(&mut buf) {
                Ok(Event::Start(_)) | Ok(Event::Empty(_)) => count += 1,
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }
        assert_eq!(count, 1550);
    }));
}

fn bench_quick_xml_namespaced(c: &mut Criterion) {
    let src: &[u8] = include_bytes!("../tests/sample_rss.xml");
    c.bench_function("quick_xml_namespaced", |b| b.iter(|| {
        let mut r = Reader::from_reader(src);
        r.check_end_names(false).check_comments(false);
        let mut count = black_box(0);
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        loop {
            match r.read_namespaced_event(&mut buf, &mut ns_buf) {
                Ok((_, Event::Start(_))) | Ok((_, Event::Empty(_))) => count += 1,
                Ok((_, Event::Eof)) => break,
                _ => (),
            }
            buf.clear();
        }
        assert_eq!(count, 1550);
    }));
}

fn bench_quick_xml_escaped(c: &mut Criterion) {
    let src: &[u8] = include_bytes!("../tests/sample_rss.xml");
    c.bench_function("quick_xml_escaped", |b| b.iter(|| {        let mut buf = Vec::new();
        let mut r = Reader::from_reader(src);
        r.check_end_names(false).check_comments(false);
        let mut count = black_box(0);
        let mut nbtxt = black_box(0);
        loop {
            match r.read_event(&mut buf) {
                Ok(Event::Start(_)) | Ok(Event::Empty(_)) => count += 1,
                Ok(Event::Text(ref e)) => nbtxt += e.unescaped().unwrap().len(),
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }
        assert_eq!(count, 1550);

        // Windows has \r\n instead of \n
        #[cfg(windows)]
        assert_eq!(nbtxt, 67661);

        #[cfg(not(windows))]
        assert_eq!(nbtxt, 66277);
    }));
}

fn bench_quick_xml_normal_trimmed(c: &mut Criterion) {
    let src: &[u8] = include_bytes!("../tests/sample_rss.xml");
    c.bench_function("quick_xml_normal_trimmed", |b| b.iter(|| {
        let mut r = Reader::from_reader(src);
        r.check_end_names(false)
            .check_comments(false)
            .trim_text(true);
        let mut count = black_box(0);
        let mut buf = Vec::new();
        loop {
            match r.read_event(&mut buf) {
                Ok(Event::Start(_)) | Ok(Event::Empty(_)) => count += 1,
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }
        assert_eq!(count, 1550);
    }));
}

fn bench_quick_xml_namespaced_trimmed(c: &mut Criterion) {
    let src: &[u8] = include_bytes!("../tests/sample_rss.xml");
    c.bench_function("quick_xml_namespaced_trimmed", |b| b.iter(|| {
        let mut r = Reader::from_reader(src);
        r.check_end_names(false)
            .check_comments(false)
            .trim_text(true);
        let mut count = black_box(0);
        let mut buf = Vec::new();
        let mut ns_buf = Vec::new();
        loop {
            match r.read_namespaced_event(&mut buf, &mut ns_buf) {
                Ok((_, Event::Start(_))) | Ok((_, Event::Empty(_))) => count += 1,
                Ok((_, Event::Eof)) => break,
                _ => (),
            }
            buf.clear();
        }
        assert_eq!(count, 1550);
    }));
}

fn bench_quick_xml_escaped_trimmed(c: &mut Criterion) {
    let src: &[u8] = include_bytes!("../tests/sample_rss.xml");
    c.bench_function("quick_xml_escaped_trimmed", |b| b.iter(|| {
        let mut buf = Vec::new();
        let mut r = Reader::from_reader(src);
        r.check_end_names(false)
            .check_comments(false)
            .trim_text(true);
        let mut count = black_box(0);
        let mut nbtxt = black_box(0);
        loop {
            match r.read_event(&mut buf) {
                Ok(Event::Start(_)) | Ok(Event::Empty(_)) => count += 1,
                Ok(Event::Text(ref e)) => nbtxt += e.unescaped().unwrap().len(),
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }
        assert_eq!(count, 1550);

        // Windows has \r\n instead of \n
        #[cfg(windows)]
        assert_eq!(nbtxt, 50334);

        #[cfg(not(windows))]
        assert_eq!(nbtxt, 50261);
    }));
}

fn bench_quick_xml_one_text_event(c: &mut Criterion) {
    let src = "Hello world!".repeat(512 / 12).into_bytes();
    let mut buf = Vec::with_capacity(1024);
    c.bench_function("quick_xml_one_text_event", |b| b.iter(|| {
        let mut r = Reader::from_reader(src.as_ref());
        let mut nbtxt = black_box(0);
        r.check_end_names(false).check_comments(false);
        match r.read_event(&mut buf) {
            Ok(Event::Text(ref e)) => nbtxt += e.unescaped().unwrap().len(),
            something_else => panic!("Did not expect {:?}", something_else),
        };

        buf.clear();

        assert_eq!(nbtxt, 504);
    }));
}

fn bench_quick_xml_one_start_event_trimmed(c: &mut Criterion) {
    let src = format!(r#"<hello target="{}">"#, "world".repeat(512 / 5)).into_bytes();
    let mut buf = Vec::with_capacity(1024);
    c.bench_function("quick_xml_one_start_event_trimmed", |b| b.iter(|| {
        let mut r = Reader::from_reader(src.as_ref());
        let mut nbtxt = black_box(0);
        r.check_end_names(false)
            .check_comments(false)
            .trim_text(true);
        match r.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => nbtxt += e.unescaped().unwrap().len(),
            something_else => panic!("Did not expect {:?}", something_else),
        };

        buf.clear();

        assert_eq!(nbtxt, 525);
    }));
}

fn bench_quick_xml_one_comment_event_trimmed(c: &mut Criterion) {
    let src = format!(r#"<!-- hello "{}" -->"#, "world".repeat(512 / 5)).into_bytes();
    let mut buf = Vec::with_capacity(1024);
    c.bench_function("quick_xml_one_comment_event_trimmed", |b| b.iter(|| {
        let mut r = Reader::from_reader(src.as_ref());
        let mut nbtxt = black_box(0);
        r.check_end_names(false)
            .check_comments(false)
            .trim_text(true);
        match r.read_event(&mut buf) {
            Ok(Event::Comment(ref e)) => nbtxt += e.unescaped().unwrap().len(),
            something_else => panic!("Did not expect {:?}", something_else),
        };

        buf.clear();

        assert_eq!(nbtxt, 520);
    }));
}

fn bench_quick_xml_one_cdata_event_trimmed(c: &mut Criterion) {
    let src = format!(r#"<![CDATA[hello "{}"]]>"#, "world".repeat(512 / 5)).into_bytes();
    let mut buf = Vec::with_capacity(1024);
    c.bench_function("quick_xml_one_cdata_event_trimmed", |b| b.iter(|| {
        let mut r = Reader::from_reader(src.as_ref());
        let mut nbtxt = black_box(0);
        r.check_end_names(false)
            .check_comments(false)
            .trim_text(true);
        match r.read_event(&mut buf) {
            Ok(Event::CData(ref e)) => nbtxt += e.unescaped().unwrap().len(),
            something_else => panic!("Did not expect {:?}", something_else),
        };

        buf.clear();

        assert_eq!(nbtxt, 518);
    }));
}

criterion_group!(
    benches,
    bench_quick_xml_normal,
    bench_quick_xml_namespaced,
    bench_quick_xml_escaped,
    bench_quick_xml_normal_trimmed,
    bench_quick_xml_namespaced_trimmed,
    bench_quick_xml_escaped_trimmed,
    bench_quick_xml_one_text_event,
    bench_quick_xml_one_start_event_trimmed,
    bench_quick_xml_one_comment_event_trimmed,
    bench_quick_xml_one_cdata_event_trimmed,
);
criterion_main!(benches);
