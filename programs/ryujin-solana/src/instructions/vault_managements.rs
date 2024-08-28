use crate::errors::ErrorCode;

use anchor_lang::prelude::*;


pub fn transfer<'a>(
  system_program: AccountInfo<'a>,
  from: AccountInfo<'a>,
  to: AccountInfo<'a>,
  amount: u64,
  seeds: Option<&[&[&[u8]]]> // Use Option to explicitly handle the presence or absence of seeds
) -> Result<()> {
  let amount_needed = amount;
  if amount_needed > from.lamports() {
      msg!("Need {} lamports, but only have {}", amount_needed, from.lamports());
      return Err(ErrorCode::NotEnoughFundsToPlay.into());
  }

  let transfer_accounts = anchor_lang::system_program::Transfer {
      from: from.to_account_info(),
      to: to.to_account_info(),
  };

  let transfer_ctx = match seeds {
      Some(seeds) => CpiContext::new_with_signer(system_program, transfer_accounts, seeds),
      None => CpiContext::new(system_program, transfer_accounts),
  };

  anchor_lang::system_program::transfer(transfer_ctx, amount)
}



