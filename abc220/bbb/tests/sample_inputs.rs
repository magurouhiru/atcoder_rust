use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"2
1011 10100
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "220\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"7
123 456
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "15642\n");
    assert!(output.stderr_str().is_empty());
}
