use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
  #[msg("Exchange or cancel request is not valid in the current stage.")]
  InvalidStage,
  #[msg("Not enough funds to perform the requested operation.")]
  InsufficientFunds,
  #[msg("The mint account associated with this trade is not valid.")]
  InvalidMint,
  #[msg("Missing mint for trade")]
  MissingMint,
  #[msg("Invalid trade p2p type. Maybe missing all mint address")]
  InvalidTradeType,
  #[msg("Invalid mint between two token account")]
  InvalidAccount,
  #[msg("Duplicate two mint")]
  DuplicateMint,
  #[msg("Account does not have invalid owner!")]
  InvalidOwner,
  #[msg("Invalid partner with specify partner set in create trade")]
  InvalidPartner,
  #[msg("Trade value and Receive value must be larger than zero")]
  ZeroValue,
  #[msg("instruction data missing params")]
  MissingParams,
}
