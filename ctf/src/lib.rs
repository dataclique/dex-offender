pub mod anvil;
pub mod ethereum;
pub mod level;
pub mod roles;
pub mod solana;
pub use level::*;
pub use roles::*;

use alloy::primitives::U256;

pub fn to_ether(amount: u64) -> U256 {
    U256::from(amount) * U256::from(10).pow(U256::from(18))
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CTFs {
    pub ethereum: ethereum::Ethereum,
}

impl CTFs {
    pub fn from_file() -> eyre::Result<Self> {
        let ctf_json = std::fs::read_to_string("../ctfs.json")?;
        let ctf: CTFs = serde_json::from_str(&ctf_json)?;
        Ok(ctf)
    }
}
