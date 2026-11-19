// anchor_lang::solana_program::system_program is exactly what the generated
// interface embeds, so the test compares against it despite the deprecation.
#![allow(deprecated)]

use ackee_level_1_cpi::{accounts, instruction};
use anchor_lang::solana_program::{
    instruction::AccountMeta, pubkey::Pubkey, system_program,
};
use anchor_lang::{InstructionData, ToAccountMetas};

#[test]
fn access_vault_preserves_idl_accounts_and_instruction_bytes() {
    let explorer = Pubkey::new_from_array([1; 32]);
    let ancient_vault = Pubkey::new_from_array([2; 32]);
    let accounts = accounts::AccessVault {
        explorer,
        ancient_vault,
        system_program: system_program::ID,
    };

    assert_eq!(
        accounts.to_account_metas(None),
        vec![
            AccountMeta::new(explorer, true),
            AccountMeta::new(ancient_vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ]
    );
    assert_eq!(
        instruction::AccessVault { _pin: 0 }.data(),
        vec![22, 90, 183, 142, 98, 79, 189, 155, 0]
    );
}
