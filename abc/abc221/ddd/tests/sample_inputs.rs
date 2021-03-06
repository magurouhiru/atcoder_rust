use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"3
1 2
2 3
3 1
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2 2 0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
1000000000 1000000000
1000000000 1000000000
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0 1000000000\n");
    assert!(output.stderr_str().is_empty());
}
