use std::process::{Command, Stdio};
use gtk4::prelude::{TextViewExt, TextBufferExt};
use gtk4::TextView;

/// Run a shell command and update a TextView with stdout and stderr
pub fn operation_with_status(output_view: &TextView, shell: &str, shell_option: &str, command: &str) {
    let result = Command::new(shell)
        .arg(shell_option)
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect(&format!(
            "ERROR in operation_with_status(), is your system using '{shell}', does it support the option '{shell_option}' and has it '{command}' installed?"
        ));

    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);
    let combined = format!("--- stdout ---\n{}-----------------\n\n--- stderr ---\n{}-----------------", stdout, stderr);

    let buffer = output_view.buffer();
    buffer.set_text(&combined);
}
