pub mod hack01_fallback;
pub mod hack02_fallout;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test() -> eyre::Result<()> {
        println!("Initializing accounts...");
        let roles = ctf::Roles::new(ctf::ANVIL_HTTP)?;

        ctf::check_exploit(&roles, hack01_fallback::Exploit).await?;
        ctf::check_exploit(&roles, hack02_fallout::Exploit).await?;

        Ok(())
    }
}
