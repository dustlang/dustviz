// crates/dustviz/src/render/svg.rs
//
// SVG renderer via Graphviz `dot -Tsvg`.
//
// This is best-effort: if `dot` is unavailable or fails, return a clear diagnostic.

use std::io::Write;
use std::process::{Command, Stdio};

use crate::util::diagnostics::Diagnostic;

pub fn render_svg(dot: &str) -> Result<String, Diagnostic> {
    let mut child = Command::new("dot")
        .arg("-Tsvg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Diagnostic::message("Graphviz 'dot' not found in PATH; install Graphviz to enable SVG output.")
            } else {
                Diagnostic::message(format!("failed to spawn Graphviz 'dot': {}", e))
            }
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(dot.as_bytes())
            .map_err(|e| Diagnostic::message(format!("failed to write DOT to 'dot' stdin: {}", e)))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| Diagnostic::message(format!("failed to run Graphviz 'dot': {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Diagnostic::message(format!(
            "Graphviz 'dot' failed: {}",
            stderr.trim()
        )));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| Diagnostic::message(format!("Graphviz 'dot' returned invalid UTF-8: {}", e)))
}
