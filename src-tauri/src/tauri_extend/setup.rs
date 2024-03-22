use anyhow::Result;

use crate::global;

pub async fn run(app: &mut tauri::App) -> Result<()> {
    global::init_global(app).await;
    Ok(())
}
