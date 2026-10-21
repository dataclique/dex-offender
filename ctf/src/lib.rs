// pub mod abi; // forge-bind ethers bindings, replaced by sol! artifacts in ctf/abi
pub mod ethernaut;
pub mod level;
pub mod paradigm_2023;
pub mod roles;
// pub mod damn_vulnerable_defi;

pub use level::*;
pub use roles::*;

use alloy::primitives::U256;

pub fn to_ether(amount: u64) -> U256 {
    U256::from(amount) * U256::from(10).pow(U256::from(18))
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CTFs {
    pub ethernaut: ethernaut::Ethernaut,
    pub paradigm_2023: paradigm_2023::Paradigm2023,
}

impl CTFs {
    pub fn from_file() -> eyre::Result<Self> {
        let ctf_json = std::fs::read_to_string("../ctfs.json")?;
        let ctf: CTFs = serde_json::from_str(&ctf_json)?;
        Ok(ctf)
    }
}
