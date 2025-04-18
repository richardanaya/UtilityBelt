use anyhow::Context;
use anyhow::Result;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

// Ce module lit et affiche les messages récents de l'historique de conversation.
pub async fn show_recent_messages(count: usize) -> Result<()> {
    let path = ".aider.chat.history.md";
    let file = File::open(path)
        .await
        .with_context(|| format!("Could not open {}", path))?;
    let reader = BufReader::new(file);

    let mut messages: Vec<(String, String)> = Vec::new(); // (role, text)
    let mut lines = reader.lines();

    // iterate through all lines
    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue; // skip blank lines
        }

        if trimmed.starts_with("####") {
            let text = trimmed.trim_start_matches('#').trim();
            if !text.is_empty() {
                messages.push(("COMMAND".into(), text.to_string()));
            }
        } else if trimmed.starts_with("# aider chat started at") {
            let text = trimmed.trim_start_matches('#').trim();
            if !text.is_empty() {
                messages.push(("SESSION-START".into(), text.to_string()));
            }
        } else {
            let last = messages.last_mut();
            if let Some((role, msg)) = last {
                if role == "RESPONSE" {
                    msg.push('\n');
                    msg.push_str(trimmed);
                    continue;
                } else {
                    messages.push(("RESPONSE".into(), trimmed.to_string()));
                }
            } else {
                messages.push(("RESPONSE".into(), trimmed.to_string()));
            }
        }
    }

    let start = messages.len().saturating_sub(count);
    println!("<aider-history>\n");
    for (role, msg) in &messages[start..] {
        println!(
            "<{}>\n{}\n</{}>\n",
            role.to_lowercase(),
            msg,
            role.to_lowercase()
        );
    }
    println!("</aider-history>");

    Ok(())
}
