use std::{fs::read_to_string, path::PathBuf, process::Command};

use lang_tester::LangTester;

static COMMENT_PREFIX: &str = "//";

fn main() {
    LangTester::new()
        .test_dir("tests/files")
        // Only use files named `*.rs` as test files.
        .test_file_filter(|p| p.extension().unwrap().to_str().unwrap() == "ukiyo")
        // Extract the first sequence of commented line(s) as the tests.
        .test_extract(|p| {
            read_to_string(p)
                .unwrap()
                .lines()
                // Skip non-commented lines at the start of the file.
                .skip_while(|l| !l.starts_with(COMMENT_PREFIX))
                // Extract consecutive commented lines.
                .take_while(|l| l.starts_with(COMMENT_PREFIX))
                .map(|l| &l[COMMENT_PREFIX.len()..])
                .collect::<Vec<_>>()
                .join("\n")
        })
        // We have two test commands:
        //   * `Run-time`: if rustc does not error, and the `Compiler` tests
        //     succeed, then the output binary is run.
        .test_cmds(move |p| {
            let mut runner = Command::new("target/debug/ukiyo");
            runner.args(&[p.to_str().unwrap()]);
            vec![("Run-time", runner)]
        })
        .run();
}
