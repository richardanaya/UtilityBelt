use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::File;
use anyhow::Context;

pub async fn show_recent_messages(count: usize) -> Result<()> {
    let path = ".aider.chat.history.md";
    let file = File::open(path)
        .await
        .with_context(|| format!("Could not open {}", path))?;
    let reader = BufReader::new(file);

    let mut messages: Vec<String> = Vec::new();   // raw message contents in order
    let mut current = String::new();
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim_start();

        // "#### …" marque un nouveau message USER ; on garde son contenu
        if trimmed.starts_with("####") {
            // clôturer l’éventuel message précédent
            if !current.trim().is_empty() {
                messages.push(current.trim().to_owned());
                current.clear();
            }
            // texte du prompt USER (= titre sans les #)
            let user_line = trimmed.trim_start_matches('#').trim();
            if !user_line.is_empty() {
                current.push_str(user_line);
            }
            continue;
        }

        // ignore any other heading lines that start with '#' (1–3, 5+ hashes)
        if trimmed.starts_with('#') {
            continue;
        }

        // ignorer toute ligne méta ou citation (> …)
        if trimmed.starts_with('>') {
            continue;
        }

        let content = trimmed;

        // Meta‑Zeilen wie „Tokens:“ sowie leere Zeilen ignorieren
        if content.is_empty() || content.starts_with("Tokens:") {
            continue;
        }

        if !current.is_empty() {
            current.push('\n');
        }
        current.push_str(content);
    }

    if !current.trim().is_empty() {
        messages.push(current.trim().to_owned());
    }

    let start = messages.len().saturating_sub(count);
    println!("<<<<<<<<<< AGENT_CONVERSATION_HISTORY");
    for (idx, msg) in messages[start..].iter().enumerate() {
        let role = if (start + idx) % 2 == 0 { "USER" } else { "ASSISTANT" };
        println!("{}:\n{}\n", role, msg);
    }
    println!(">>>>>>>>>> AGENT_CONVERSATION_HISTORY");

    Ok(())
}
