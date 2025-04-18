use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::File;
use anyhow::Context;

// Ce module lit et affiche les messages récents de l'historique de conversation.
pub async fn show_recent_messages(count: usize) -> Result<()> {
    let path = ".aider.chat.history.md";
    let file = File::open(path)
        .await
        .with_context(|| format!("Could not open {}", path))?;
    let reader = BufReader::new(file);

    let mut messages: Vec<(String, String)> = Vec::new();   // (role, text)
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;                   // skip blank lines
        }

        if trimmed.starts_with("####") {
            // USER message: keep the text after the hashes
            let text = trimmed.trim_start_matches('#').trim();
            if !text.is_empty() {
                messages.push(("USER".into(), text.to_string()));
            }
        } else {
            // everything else is an ASSISTANT message
            messages.push(("ASSISTANT".into(), trimmed.to_string()));
        }
    }

    // Regrouper les lignes consécutives partageant le même rôle
    let mut grouped: Vec<(String, String)> = Vec::new();
    for (role, msg) in messages.into_iter() {
        if let Some((last_role, last_msg)) = grouped.last_mut() {
            if last_role == &role {
                last_msg.push('\n');
                last_msg.push_str(&msg);
                continue;
            }
        }
        grouped.push((role, msg));
    }

    let start = grouped.len().saturating_sub(count);
    println!("<<<<<<<<<< AGENT_CONVERSATION_HISTORY");
    for (role, msg) in &grouped[start..] {
        println!("{}:\n{}\n", role, msg);
    }
    println!(">>>>>>>>>> AGENT_CONVERSATION_HISTORY");

    Ok(())
}
