use anchor_lang::prelude::*;
use orao_solana_vrf::{state::Randomness, RANDOMNESS_ACCOUNT_SEED};

use std::mem::size_of;

use crate::StillProcessing;

pub fn reveal_randomness(ctx: Context<StopSpinningInstruction>) -> Result<()> {
  if ctx.accounts.randomness_account_data.data_is_empty() {
    return Err(ProgramError::UninitializedAccount.into());
  }

  let rand_acc: Randomness = Randomness::try_deserialize(&mut &ctx.accounts.randomness_account_data.data.borrow()[..])?;

  let  randomness: u64 = if let Some(rand_acc) = rand_acc.fulfilled() {
    let value = rand_acc[0..size_of::<u64>()].try_into().unwrap();
    u64::from_le_bytes(value)
  } else {
    return Err(StillProcessing::StillProcessing.into());
  };

  let result = (randomness % 360) as u16;
  msg!("The value is: {}", result);

  Ok(())
}


#[derive(Accounts)]
#[instruction(force: [u8; 32])]
pub struct StopSpinningInstruction<'info> {
  #[account(mut)]
  pub user: Signer<'info>,

  /// CHECK: The account's data is validated manually within the handler.
  #[account( mut, seeds = [RANDOMNESS_ACCOUNT_SEED.as_ref(), &force], bump, seeds::program = orao_solana_vrf::ID )]
  pub randomness_account_data: AccountInfo<'info>,

  /// CHECK: The account's data is validated manually within the handler.
  #[account(mut)]
  pub vault_account: AccountInfo<'info>,
}


