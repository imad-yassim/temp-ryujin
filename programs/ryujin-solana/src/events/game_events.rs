use anchor_lang::prelude::*;

#[event]
pub struct WLWinnerEvent {
    pub winner_pubkey: Pubkey,
    pub discord_user_id: String,  // Ensure this is passed and recorded somehow
}


#[event]
pub struct OGWinnerEvent {
    pub winner_pubkey: Pubkey,
    pub discord_user_id: String,  // Ensure this is passed and recorded somehow
}
