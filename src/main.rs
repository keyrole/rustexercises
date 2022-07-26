mod server;
mod client;
fn main() -> std::io::Result<()> {
    // server::run()?;
    client::run()?;
    // client::run_1()?;
    Ok(())
}
