use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn should_output_correct_info() {
    let exe = env!("CARGO_BIN_EXE_rudim");

    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn rudim binary");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open child stdin");
        stdin
            .write_all(b"info\nexit\n")
            .expect("Failed to write CLI input");
    }

    let output = child
        .wait_with_output()
        .expect("Failed to read process output");

    assert!(
        output.status.success(),
        "rudim process exited with non-zero status"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout was not valid UTF-8");
    let first_line = stdout.lines().next().unwrap_or_default();

    let expected = format!("Rudim v{} by znxftw", env!("CARGO_PKG_VERSION"));
    assert_eq!(expected, first_line);
}
