use anchor_lang::prelude::*;

use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use crate::instructions::vault_managements::*;
use crate::StopSpinningInstruction;



pub fn distribute_reward(ctx: Context<StopSpinningInstruction>, result: u64) -> Result<()> {
  msg!("Randomness Value (0-359): {}", result);
  
  // OG 1%
  if result < 1 {

  }

  // SOLS REWARDS
  // 0.005 SOL ( 5% )
  // 0.05 SOL ( 5% )
  if result < 500 {
    transfer(
      ctx.accounts.system_program.to_account_info(),
      ctx.accounts.vault_account.to_account_info(),
      ctx.accounts.user.to_account_info(),
      5 * LAMPORTS_PER_SOL / 1000,
      None,
    );
  } else if result < 1500 {
  
  }


  Ok(())
}



