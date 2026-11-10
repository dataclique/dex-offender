use ctf::solana::rpc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let url = rpc::rpc_url()?;
    let keypair_path = rpc::keypair_path();
    let offender = rpc::deploy_level_one(&url, &keypair_path).await?;
    println!(
        "Ancient Vault ready at {}",
        ctf::solana::bootcamp::level_1::PROGRAM_ID
    );
    println!("RPC: {url}");
    println!("Offender: {}", offender.pubkey());
    println!("Offender keypair: {}", keypair_path.display());
    println!("Only level 1 is enabled. Run cargo run to practice.");
    Ok(())
}
