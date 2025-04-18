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

    let mut messages: Vec<(String, String)> = Vec::new();   // (role, content)
    let mut role = String::from("MESSAGE");
    let mut current = String::new();
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim_start();

        // Empieza una nueva sección al ver cualquier encabezado '#'
        if trimmed.starts_with('#') {
            // Guarda el mensaje acumulado antes de iniciar uno nuevo
            if !current.trim().is_empty() {
                messages.push((role.clone(), current.trim().to_owned()));
            }
            current.clear();

            // Determina el rol según el encabezado de nivel 4
            let heading = trimmed.trim_start_matches('#').trim().to_lowercase();
            role = if heading.contains("assistant") {
                "ASSISTANT".to_string()
            } else if heading.contains("user") {
                "USER".to_string()
            } else {
                "MESSAGE".to_string()
            };

            // no copiamos el encabezado al contenido, solo cambiamos el rol
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
        messages.push((role.clone(), current.trim().to_owned()));
    }

    // Imprime los N últimos
    let start = messages.len().saturating_sub(count);
    println!("<<<<<<<<<< HISTORY");
    for (role, msg) in &messages[start..] {
        println!("{}:\n{}\n", role, msg);
    }
    println!(">>>>>>>>>> HISTORY");

    Ok(())
}
