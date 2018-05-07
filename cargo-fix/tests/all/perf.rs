use super::project;

#[test]
fn fix_perf() {
    use std::fmt::Write;
    use std::time::Instant;

    let mut input = String::from("");
    (0..1000).for_each(|n| {
        write!(&mut input, "pub fn foo_{}() -> u32 {{ let mut x = 3; x }}", n)
            .unwrap()
    });

    let p = project()
        .file("src/lib.rs", &input)
        .build();

    let start = Instant::now();
    p.expect_cmd("cargo-fix fix")
        .stdout("")
        .stderr_contains("[FIXING] src/lib.rs (1000 fixes)")
        .run();

    let duration = start.elapsed();
    println!("{} took {:?}", "fix_10k_warnings", duration);
}
