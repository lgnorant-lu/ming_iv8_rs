//! iv8-rs performance benchmarks
//!
//! Measures key performance metrics against iv8 0.1.2 baseline:
//! - JSContext create + eval + close: target ≤ 5ms (iv8: ~3.3ms)
//! - Simple eval throughput (1+1): target ≥ 500k ops/s (iv8: ~950k)
//! - Browser API call: target ≥ 200k ops/s (iv8: 340-570k)
//! - 8-thread speedup: target ≥ 3.5x (iv8: 4.71x)
//! - 100-round memory drift: target ≤ +5MB (iv8: +2MB)

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use iv8_core::{EmbeddedV8Kernel, EvalOpts, KernelConfig};
use std::time::Duration;

fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).expect("kernel")
}

fn eval_opts() -> EvalOpts {
    EvalOpts::default()
}

// ─── 1. JSContext create + eval + close ──────────────────────────────────────

fn bench_context_lifecycle(c: &mut Criterion) {
    c.bench_function("context_lifecycle", |b| {
        b.iter(|| {
            let mut kernel = make_kernel();
            let result = kernel.eval(black_box("1+1"), eval_opts()).expect("eval");
            kernel.dispose();
            black_box(result);
        });
    });
}

// ─── 2. Simple eval throughput ───────────────────────────────────────────────

fn bench_eval_simple(c: &mut Criterion) {
    let mut kernel = make_kernel();

    c.bench_function("eval_simple_1plus1", |b| {
        b.iter(|| {
            let result = kernel.eval(black_box("1+1"), eval_opts()).expect("eval");
            black_box(result);
        });
    });
}

// ─── 3. Browser API calls ────────────────────────────────────────────────────

fn bench_browser_api(c: &mut Criterion) {
    let mut kernel = make_kernel();

    let mut group = c.benchmark_group("browser_api");

    group.bench_function("navigator_userAgent", |b| {
        b.iter(|| {
            let result = kernel
                .eval(black_box("navigator.userAgent"), eval_opts())
                .expect("eval");
            black_box(result);
        });
    });

    group.bench_function("screen_width", |b| {
        b.iter(|| {
            let result = kernel
                .eval(black_box("screen.width"), eval_opts())
                .expect("eval");
            black_box(result);
        });
    });

    group.bench_function("date_now", |b| {
        b.iter(|| {
            let result = kernel
                .eval(black_box("Date.now()"), eval_opts())
                .expect("eval");
            black_box(result);
        });
    });

    group.bench_function("performance_now", |b| {
        b.iter(|| {
            let result = kernel
                .eval(black_box("performance.now()"), eval_opts())
                .expect("eval");
            black_box(result);
        });
    });

    group.bench_function("math_random", |b| {
        b.iter(|| {
            let result = kernel
                .eval(black_box("Math.random()"), eval_opts())
                .expect("eval");
            black_box(result);
        });
    });

    group.bench_function("crypto_random_uuid", |b| {
        b.iter(|| {
            let result = kernel
                .eval(black_box("crypto.randomUUID()"), eval_opts())
                .expect("eval");
            black_box(result);
        });
    });

    group.finish();
}

// ─── 4. DOM operations ───────────────────────────────────────────────────────

fn bench_dom_ops(c: &mut Criterion) {
    let mut kernel = make_kernel();

    // Load a page with DOM elements
    kernel.page_load(
        "<html><body><div id='t1'></div><div id='t2'></div><div class='item'></div><div class='item'></div></body></html>",
        None,
    );

    let mut group = c.benchmark_group("dom_ops");

    group.bench_function("getElementById", |b| {
        b.iter(|| {
            let result = kernel
                .eval(black_box("document.getElementById('t1')"), eval_opts())
                .expect("eval");
            black_box(result);
        });
    });

    group.bench_function("querySelector", |b| {
        b.iter(|| {
            let result = kernel
                .eval(black_box("document.querySelector('.item')"), eval_opts())
                .expect("eval");
            black_box(result);
        });
    });

    group.bench_function("querySelectorAll", |b| {
        b.iter(|| {
            let result = kernel
                .eval(
                    black_box("document.querySelectorAll('.item').length"),
                    eval_opts(),
                )
                .expect("eval");
            black_box(result);
        });
    });

    group.finish();
}

// ─── 5. Crypto operations ────────────────────────────────────────────────────

fn bench_crypto(c: &mut Criterion) {
    let mut kernel = make_kernel();

    let mut group = c.benchmark_group("crypto");

    group.bench_function("getRandomValues_16", |b| {
        b.iter(|| {
            let result = kernel
                .eval(
                    black_box("crypto.getRandomValues(new Uint8Array(16)).length"),
                    eval_opts(),
                )
                .expect("eval");
            black_box(result);
        });
    });

    group.bench_function("randomUUID", |b| {
        b.iter(|| {
            let result = kernel
                .eval(black_box("crypto.randomUUID()"), eval_opts())
                .expect("eval");
            black_box(result);
        });
    });

    group.finish();
}

// ─── 6. Multi-context isolation ──────────────────────────────────────────────

fn bench_multi_context(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_context");
    group.sample_size(20); // Fewer samples for expensive operations

    group.bench_function("create_2_contexts", |b| {
        b.iter(|| {
            let mut k1 = make_kernel();
            let mut k2 = make_kernel();
            let r1 = k1.eval(black_box("1+1"), eval_opts()).expect("eval");
            let r2 = k2.eval(black_box("2+2"), eval_opts()).expect("eval");
            k1.dispose();
            k2.dispose();
            black_box((r1, r2));
        });
    });

    group.finish();
}

// ─── 7. Multi-thread speedup (REQ-PERF-003) ──────────────────────────────────

fn bench_multithread(c: &mut Criterion) {
    let mut group = c.benchmark_group("multithread");
    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(10));

    // Single-thread baseline
    group.bench_function("single_thread_8x", |b| {
        b.iter(|| {
            for _ in 0..8 {
                let mut k = make_kernel();
                black_box(
                    k.eval(black_box("navigator.userAgent"), eval_opts())
                        .expect("eval"),
                );
                k.dispose();
            }
        });
    });

    // 8-thread parallel (each thread creates its own context)
    group.bench_function("8_threads_parallel", |b| {
        b.iter(|| {
            let handles: Vec<_> = (0..8)
                .map(|_| {
                    std::thread::spawn(|| {
                        let mut k = make_kernel();
                        // Eval and convert to string (Send-safe)
                        let r = k.eval("navigator.userAgent", eval_opts()).is_ok();
                        k.dispose();
                        r
                    })
                })
                .collect();
            for h in handles {
                black_box(h.join().expect("thread"));
            }
        });
    });

    group.finish();
}

// ─── 8. Memory stability (100 rounds) ────────────────────────────────────────

fn bench_memory_stability(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_stability");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(30));

    group.bench_function("100_context_cycles", |b| {
        b.iter(|| {
            for _ in 0..100 {
                let mut kernel = make_kernel();
                kernel
                    .eval(black_box("navigator.userAgent + screen.width"), eval_opts())
                    .ok();
                kernel.dispose();
            }
        });
    });

    group.finish();
}

// ─── 8. Throughput: ops/second estimate ──────────────────────────────────────

fn bench_throughput(c: &mut Criterion) {
    let mut kernel = make_kernel();

    let mut group = c.benchmark_group("throughput");
    group.throughput(criterion::Throughput::Elements(1));

    group.bench_function("eval_1plus1_throughput", |b| {
        b.iter(|| {
            black_box(kernel.eval(black_box("1+1"), eval_opts()).expect("eval"));
        });
    });

    group.bench_function("navigator_ua_throughput", |b| {
        b.iter(|| {
            black_box(
                kernel
                    .eval(black_box("navigator.userAgent"), eval_opts())
                    .expect("eval"),
            );
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_context_lifecycle,
    bench_eval_simple,
    bench_browser_api,
    bench_dom_ops,
    bench_crypto,
    bench_multi_context,
    bench_multithread,
    bench_throughput,
);

criterion_main!(benches);
