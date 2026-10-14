#[test]
#[cfg(all(not(miri), not(target_family = "wasm")))]
fn ui() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/compile_pass/*.rs");

    if rustversion::cfg!(nightly) {
        t.compile_fail("tests/ui/compile_fail/*.rs");
    }
}
