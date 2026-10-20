pub mod hack01_fallback;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        println!("Initializing accounts...");
        let roles = ctf::Roles::new(ctf::ANVIL_HTTP)?;

        ctf::check_exploit(&roles, hack01_fallback::Exploit).await?;

        Ok(())
    }
}
