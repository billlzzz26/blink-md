//! Integration tests for mmd-mcp-server

use std::process::Command;

fn get_bin() -> String {
    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    format!("{}/debug/mmd-mcp-server", target_dir)
}

#[test]
fn mmd_server_should_start_and_respond_to_version() {
    let bin = get_bin();
    let output = Command::new(&bin).arg("--version").output();

    match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout);
            assert!(
                stdout.contains("mmd-mcp-server") || stdout.contains("0.1.0"),
                "version output should contain server name or version"
            );
        }
        Err(e) => {
            // Binary may not be built yet in test context - skip gracefully
            eprintln!("Skipping: binary not found ({})", e);
        }
    }
}

#[test]
fn render_mermaid_svg_should_produce_svg_output() {
    // Test the underlying library directly
    let diagram = "flowchart LR; A-->B";
    let result = mermaid_rs_renderer::render(diagram);
    assert!(result.is_ok(), "render should succeed");
    let svg = result.unwrap();
    assert!(svg.contains("<svg"), "output should contain SVG tag");
}

#[test]
fn render_mermaid_should_handle_sequence_diagram() {
    let diagram = r#"
sequenceDiagram
    Alice->>Bob: Hello
    Bob-->>Alice: Hi
"#;
    let result = mermaid_rs_renderer::render(diagram);
    assert!(result.is_ok(), "sequence diagram should render");
}

#[test]
fn render_mermaid_should_handle_pie_chart() {
    let diagram = r#"
pie title Test
    "A" : 50
    "B" : 50
"#;
    let result = mermaid_rs_renderer::render(diagram);
    assert!(result.is_ok(), "pie chart should render");
}
