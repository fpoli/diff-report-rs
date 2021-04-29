use crate::diagnostics::{Diagnostic, Level, Message};
use crate::diff::{parse_diff, FileChanges};
use crate::intervals::intersect_intervals;
use anyhow::{bail, Context, Result};
use clap::{App, Arg};
use std::io::{self, BufRead, Write};
use std::process::Command;

mod diagnostics;
mod diff;
mod intervals;

fn main() -> Result<()> {
    let matches = App::new("Report-by-diff")
        .arg(Arg::with_name("first_ref")
            .help("The first commit to pass to 'git diff'"))
        .arg(Arg::with_name("second_ref")
            .help("The second commit to pass to 'git diff'"))
        .get_matches();

    // Read arguments
    let mut git_refs = vec![];
    if let Some(first_ref) = matches.value_of("first_ref") {
        git_refs.push(first_ref.to_string());
    }
    if let Some(second_ref) = matches.value_of("second_ref") {
        git_refs.push(second_ref.to_string());
    }

    // Obtain diff
    let output = Command::new("git")
        .arg("diff")
        .arg("--unified=0")
        .args(git_refs)
        .output()
        .expect("failed to execute git diff");
    if !output.stderr.is_empty() {
        io::stderr()
            .write_all(&output.stderr)
            .with_context(|| "Failed to report the stderr of `git diff`")?;
    }
    if !output.status.success() {
        bail!(
            "`git diff` terminated with exit status {:?}",
            output.status.code().unwrap()
        );
    }
    let diff = String::from_utf8_lossy(&output.stdout);
    let file_changes = parse_diff(&diff)?;

    // Filter and report JSON diagnostic messages from standard input
    for line in io::stdin().lock().lines() {
        let line = line.with_context(|| "Failed to read line from standard input")?;
        let diagnostic: Diagnostic = serde_json::from_str(&line)
            .with_context(|| format!("Failed to parse JSON from standard input: {:?}", line))?;
        // Ignore diagnostics that don't have a message
        if let Some(message) = diagnostic.message {
            if should_report_message(&message, &file_changes) {
                println!("{}", message.rendered);
            }
        }
    }

    Ok(())
}

/// Return `false` iff the message is a warning not related to changed lines.
fn should_report_message(message: &Message, file_changes: &FileChanges) -> bool {
    if matches!(message.level, Level::Warning) {
        let mut intersects_changes = false;
        for span in &message.spans {
            if let Some(file_changes) = file_changes.get(&span.file_name).as_ref() {
                if intersect_intervals(span.line_start, span.line_end, file_changes) {
                    intersects_changes = true;
                    break;
                }
            }
        }
        if !intersects_changes {
            return false;
        }
    }
    true
}
