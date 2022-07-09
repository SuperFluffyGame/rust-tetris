use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    real_tetris::run().await
}
