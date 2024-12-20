use crate::errors::ErrorCode;

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::{self, AssociatedToken}, token::{self, Mint, Token, TokenAccount, Transfer}};


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



pub fn transfer_spl_tokens<'a>(
  to: AccountInfo<'a>,
  to_ata: Account<'a, TokenAccount>,
  mint: Account<'a, Mint>,
  vault_authority: AccountInfo<'a>,
  vault_ata: Account<'a, TokenAccount>,
  token_program: Program<'a, Token>,
  associated_token_program: Program<'a, AssociatedToken>,
  system_program: Program<'a, System>,
  amount: u64
  ) -> Result<()> {


  // CHECK IF RECIPIENT HAVE AN ASSOCIATED TOKEN ACCOUNT
  if to_ata.amount == 0 && to_ata.owner == Pubkey::default() {
    // IF NO CREATE ONE FOR HIM
    let cpi_accounts = associated_token::Create  {
      payer: to.to_account_info(),
      associated_token: to_ata.to_account_info(),
      authority: to.to_account_info(),
      mint: mint.to_account_info(),
      system_program: system_program.to_account_info(),
      token_program: token_program.to_account_info(),
    };

    let cpi_program = associated_token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    associated_token::create(cpi_ctx)?;
  }


  // SEND TOKEN TO USER ASSOCIATED TOKEN ACCOUNT
  let transfer_ctx = CpiContext::new(
    token_program.to_account_info(),
    Transfer {
        from: vault_ata.to_account_info(),
        to: to_ata.to_account_info(),
        authority: vault_authority.to_account_info(),
    },
  );
  token::transfer(transfer_ctx, amount)

}