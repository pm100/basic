use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use test_generator::test_resources;

fn basic_command() -> Command {
    let mut path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
    path.push("basic");
    path.set_extension(env::consts::EXE_EXTENSION);
    let mut command = Command::new(path.into_os_string());
    command.current_dir(env!("CARGO_MANIFEST_DIR"));
    command
}

fn parse_bas_file(path: &PathBuf) -> (bool, Vec<String>, Vec<String>) {
    let content = fs::read_to_string(path).unwrap();
    let mut skip = false;
    let mut exact = vec![];
    let mut contains = vec![];
    for line in content.lines() {
        // Strip optional BASIC line number so both `REM SKIP` and `10 REM SKIP` work.
        let trimmed = line.trim().trim_start_matches(|c: char| c.is_ascii_digit()).trim_start();
        let upper = trimmed.to_uppercase();
        if upper.starts_with("REM SKIP") {
            skip = true;
        } else if upper.starts_with("REM EXPECT_CONTAINS: ") {
            let val = &trimmed[trimmed.to_uppercase().find("EXPECT_CONTAINS: ").unwrap() + 17..];
            contains.push(val.to_owned());
        } else if upper.starts_with("REM EXPECT: ") {
            let val = &trimmed[trimmed.to_uppercase().find("EXPECT: ").unwrap() + 8..];
            exact.push(val.to_owned());
        }
    }
    (skip, exact, contains)
}

#[test_resources("tests/bas/test_*.bas")]
fn run_bas_integration_test(filename: &str) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(filename);
    let (skip, exact, contains) = parse_bas_file(&path);

    if skip {
        return;
    }

    let output = basic_command()
        .arg(&path)
        .output()
        .expect("Failed to run basic interpreter");

    assert!(
        output.status.success(),
        "basic exited with failure:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();

    let mut pos = 0;
    for expected_line in &exact {
        let found = lines[pos..].iter().position(|l| l == expected_line);
        assert!(
            found.is_some(),
            "Expected line not found: {:?}\nActual output:\n{}",
            expected_line,
            stdout
        );
        pos += found.unwrap() + 1;
    }

    for substr in &contains {
        assert!(
            stdout.contains(substr.as_str()),
            "Expected stdout to contain {:?}\nActual output:\n{}",
            substr,
            stdout
        );
    }
}
