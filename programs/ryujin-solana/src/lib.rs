
pub mod errors;
pub mod instructions;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

use instructions::vault_managements::*;
use instructions::start_game::*;
use instructions::reveal_result::*;
use instructions::initialize_pda::*;

use anchor_lang::{
  solana_program::account_info::AccountInfo,
  AccountDeserialize,
};

declare_id!("RYUqKUNAX3nFc2kPPBXLHzm326n5qsUveCJ3Z6Jj1fX");

const GAME_PRICE: u64 = (1 / 10) * LAMPORTS_PER_SOL;
// const _WL_TOKEN: &str = "8KrDpQqz7hvTgk3BP4tmLN2thtAErmdcd5Z7sk4wAQAd";
// const _OG_TOKEN: &str = "2fyaGAUav6v8uU2bbiWsYArgeaUoD3DzRcaYndC1AgVG";

#[program]
pub mod ryujin_solana {



use instructions::reward_distribution::distribute_reward;

use super::*;

  pub fn initialize_game(ctx: Context<InitializePDAInstruction>) -> Result<()> {
    initialize(ctx)
  }

  pub fn start_game(ctx: Context<StartSpinning>, force: [u8; 32]) -> Result<()> {
    //  Player Status Verification.
    let player_state: &mut Account<'_, PlayerState> = &mut ctx.accounts.player_state;

    if player_state.allowed_user != ctx.accounts.user.key() {
      return Err(CantStartNewGame::UserNotAllowed.into());
    }
    if  player_state.current_force != [0; 32] {
      return Err(CantStartNewGame::SpinWaitingForReveal.into());
    }


    //  Taking the game collateral before requesting randomness.
    msg!("Taking the game collateral before requesting randomness...");
    transfer(
      ctx.accounts.system_program.to_account_info(),
      ctx.accounts.user.to_account_info(),  // Include the user_account
      ctx.accounts.vault_account.to_account_info(),
      GAME_PRICE,
      None,
    )?;
    msg!("Payment retrieved successfully.");
    // Requesting orao randomness.
    request_randomness(ctx, force)
  }

  pub fn stop_spinning(ctx: Context<StopSpinningInstruction>, _force: [u8; 32]) -> Result<()> {
    //  Player Status Verification.
    let player_state: &mut Account<'_, PlayerState> = &mut ctx.accounts.player_state;
    let wl_vault_account = &ctx.accounts.wl_vault_account;
    let og_vault_account = &ctx.accounts.og_vault_account;

    if player_state.allowed_user != ctx.accounts.user.key() {
      return Err(CantStartNewGame::UserNotAllowed.into());
    }
    if player_state.current_force == [0; 32]{
      return Err(CantRevealGameResult::EmptyForce.into());
    }

    // Ensure that the program-derived PDA is the authority
    let (expected_wl_vault_authority, _wl_vault_bump) = Pubkey::find_program_address(&[b"WLVaultAccount"], ctx.program_id);
    let (expected_og_vault_authority, _og_vault_bump) = Pubkey::find_program_address(&[b"OGVaultAccount"], ctx.program_id);

    if wl_vault_account.key() != expected_wl_vault_authority {
        return Err(ProgramError::IncorrectProgramId.into());
    }

    if og_vault_account.key() != expected_og_vault_authority {
      return Err(ProgramError::IncorrectProgramId.into());
    }

    let player_state: &mut Account<'_, PlayerState> = &mut ctx.accounts.player_state;
    let randomness_account_data: &AccountInfo = &mut ctx.accounts.randomness_account_data;

    // Reveal result and distribute rewards.
    let result = reveal_randomness(randomness_account_data, player_state)?;

    distribute_reward(ctx, result)

    // Ok(())

  }


   // Settle the flip after randomness is revealed
  //  pub fn settle_flip(ctx: Context<SettleWheel>, escrow_bump: u8) -> Result<()> {
  //   let player_state: &mut Account<'_, PlayerState> = &mut ctx.accounts.player_state;
  //   let rand_acc = get_account_data(&ctx.accounts.randomness_account_data)?;

  //   let randomness = current_state(&rand_acc);
  //   if randomness == 0 {
  //       return err!(StillProcessing::StillProcessing)
  //   }
  //   let randomness_result: u16 = (randomness % 360) as u16;

  //   msg!("VRF result is: {}", randomness);


  //   // Update and log the result
  //   player_state.latest_flip_result = randomness_result;

  //   let seed_prefix = b"stateEscrow".as_ref();
  //   let escrow_seed = &[&seed_prefix[..], &[escrow_bump]];
  //   let seeds_slice: &[&[u8]] = escrow_seed;
  //   let binding = [seeds_slice];
  //   let seeds: Option<&[&[&[u8]]]> = Some(&binding);


  //   msg!("Random number:  {}", randomness_result);

  //   if randomness_result < 10 {
  //     msg!("You win!");
  //     let sol_win_amount: u64 = 1 / 10;

  //     let rent = Rent::get()?;
  //     let needed_lamports = sol_win_amount + rent.minimum_balance(ctx.accounts.vault_account.data_len());
  //     if needed_lamports > ctx.accounts.vault_account.lamports() {
  //         msg!("Not enough funds in treasury to pay out the user. Please try again later");
  //     } else {
  //       transfer(
  //         ctx.accounts.system_program.to_account_info(),
  //         ctx.accounts.vault_account.to_account_info(), // Transfer from the vault
  //         ctx.accounts.user.to_account_info(), // Payout to the user's wallet
  //         (1 / 100) * LAMPORTS_PER_SOL,
  //         seeds // Include seeds
  //       )?;
  //     }
  //   } else if randomness_result < 30 {

  //   } else if randomness_result < 200 {

  //   } else {
  //     // On lose, we keep the user's initial colletaral and they are
  //     // allowed to play again.
  //     msg!("You lose!");
  // }

  //   Ok(())
  // }

}



// === Errors ===
#[error_code]
pub enum CantStartNewGame {
    #[msg("Can't start a new game until you reveal previous spin.")]
    SpinWaitingForReveal,
    #[msg("Can't start a new game. User not allowed")]
    UserNotAllowed
}

#[error_code]
pub enum CantRevealGameResult {
    #[msg("Can't reveal game. Force is empty.")]
    EmptyForce,
    #[msg("Can't start a new game. User not allowed")]
    UserNotAllowed
}


#[error_code]
pub enum StillProcessing {
    #[msg("Randomness is still being fulfilled")]
    StillProcessing
}

#[error_code]
pub enum InvalidAmount {
    #[msg("Amount must be greater than 0.05 SOL")]
    InvalidAmount
}