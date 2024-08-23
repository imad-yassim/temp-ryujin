use crate::states::vault_states::*;

use anchor_lang::prelude::*;
use anchor_lang::system_program;



pub fn transfer_lamports<'info>(program: AccountInfo<'info> ,from: AccountInfo<'info>, to : AccountInfo<'info>, amount: u64) -> Result<()> {
  let cpi_context = CpiContext::new(
    program,
    system_program::Transfer { from, to },
  );

  system_program::transfer(cpi_context, amount)
}


#[derive(Accounts)]
pub struct FillVaultInstruction<'info> {
  #[account(mut)]
  pub player: Signer<'info>,

  #[account(
    init_if_needed,
    seeds = [b"payment_account"],
    payer = player,
    space = 8,
    bump,
  )]
  pub vault_account: Account<'info, VaultAccount>,
  pub system_program: Program<'info, System>,
 }


