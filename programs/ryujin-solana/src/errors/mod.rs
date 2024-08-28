use anchor_lang::error_code;

// === Errors ===
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access attempt.")]
    Unauthorized,
    GameStillActive,
    NotEnoughFundsToPlay,
    RandomnessAlreadyRevealed,
    RandomnessNotResolved,
    RandomnessExpired,
}