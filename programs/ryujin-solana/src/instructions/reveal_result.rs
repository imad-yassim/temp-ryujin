use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use orao_solana_vrf::{program::OraoVrf, state::{NetworkState, Randomness}, CONFIG_ACCOUNT_SEED, RANDOMNESS_ACCOUNT_SEED};

use std::mem::size_of;

use crate::StillProcessing;

use super::initialize_pda::PlayerState;

pub fn get_account_data(account_info: &AccountInfo) -> Result<Randomness> {
  if account_info.data_is_empty() {
    return Err(ProgramError::UninitializedAccount.into());
  }
  let account = Randomness::try_deserialize(&mut &account_info.data.borrow()[..])?;
  Ok(account)
}

pub fn current_state(randomness: &Randomness) ->u64 {
  if let Some(randomness) = randomness.fulfilled() {
      let value = randomness[0..size_of::<u64>()].try_into().unwrap();
      return u64::from_le_bytes(value);
  } else {
      return 100001;

  }
}
pub fn reveal_randomness(
    randomness_account_data: &AccountInfo,
    player_state: &mut Account<PlayerState>,
  ) -> Result<u64> {
  if randomness_account_data.data_is_empty() {
    return Err(ProgramError::UninitializedAccount.into());
  }

  player_state.current_force = [0; 32];

  let rand_acc = get_account_data(randomness_account_data)?;

  let randomness = current_state(&rand_acc);

  msg!("Randomness Value (raw) : {}", randomness);

  if randomness == 10001 {
    return Err(StillProcessing::StillProcessing.into());
  }

  // Ensure randomness value is between 0 and 359
  let result: u64 = (randomness % 10000) as u64;
  msg!("Randomness Value (0-359): {}", result);

  Ok(result)

}


#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct StopSpinningInstruction<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

  #[account(mut, seeds = [b"playerState", user.key.as_ref()], bump)]
  pub player_state: Account<'info, PlayerState>,

  /// CHECK: The account's data is validated manually within the handler.
  #[account(
    mut,
    seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), &force],
    bump,
    seeds::program = orao_solana_vrf::ID
  )]
  pub randomness_account_data: AccountInfo<'info>,

   #[account(
       mut,
       seeds = [CONFIG_ACCOUNT_SEED.as_ref()],
       bump,
       seeds::program = orao_solana_vrf::ID
   )]
   pub config: Account<'info, NetworkState>,
   pub vrf: Program<'info, OraoVrf>,
   pub system_program: Program<'info, System>,

  /// CHECK: The account's data is validated manually within the handler.
  #[account(mut, seeds = [b"vaultAccount".as_ref()], bump )]
  pub vault_account: AccountInfo<'info>,
  

  /// CHECK: The account's data is validated manually within the handler.
  #[account(mut, seeds = [b"WLVaultAccount".as_ref()], bump )]
  pub wl_vault_account: Account<'info, TokenAccount>,
  
  /// CHECK: The account's data is validated manually within the handler.
  #[account(mut, seeds = [b"OGVaultAccount".as_ref()], bump )]
  pub og_vault_account: Account<'info, TokenAccount>,

  #[account(mut)]
  pub user_og_at: Box<Account<'info, TokenAccount>>,
  #[account(mut)]
  pub user_wl_at: Box<Account<'info, TokenAccount>>,
}


