// #![windows_subsystem = "windows"]

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    real_tetris::run().await
}
