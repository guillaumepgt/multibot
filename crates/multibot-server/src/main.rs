//! MultiBot server binary entrypoint.

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!(
        tools_version = multibot_tools::tools_version(),
        "MultiBot server initialized"
    );
    println!("MultiBot server v{}", multibot_core::version());
    Ok(())
}
