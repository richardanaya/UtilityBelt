use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::File;
use anyhow::Context;

pub async fn show_recent_messages(count: usize) -> Result<()> {
    let path = ".aider.chat.history.md";
    let file = File::open(path)
        .await
        .with_context(|| format!("Could not open {}", path))?;
    let mut reader = BufReader::new(file);

    let mut messages = Vec::<String>::new();
    let mut current = String::new();
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim_start();

        // Empieza una nueva sección al ver cualquier encabezado '#'
        if trimmed.starts_with('#') {
            if !current.trim().is_empty() {
                messages.push(current.trim().to_owned());
            }
            current.clear();

            // Solo los encabezados de nivel 4 son parte del propio mensaje
            if trimmed.starts_with("####") {
                let heading = trimmed.trim_start_matches('#').trim();
                current.push_str(heading);
            }
            continue;
        }

        // Agrega líneas que empiezan con '>'
        if trimmed.starts_with('>') {
            let content = trimmed.trim_start_matches('>').trim_start();
            if !content.is_empty() {
                if !current.is_empty() {
                    current.push('\n');
                }
                current.push_str(content);
            }
        }
    }

    // Guarda el último mensaje, si existe
    if !current.trim().is_empty() {
        messages.push(current.trim().to_owned());
    }

    // Imprime los N últimos
    let start = messages.len().saturating_sub(count);
    for msg in &messages[start..] {
        println!("{}\n", msg);
    }

    Ok(())
}
