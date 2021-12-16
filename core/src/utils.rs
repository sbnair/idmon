use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};


pub fn verify_signer(account: &AccountInfo) -> ProgramResult {
    if account.is_signer {
        return Ok(());
    }

    Err(ProgramError::MissingRequiredSignature)
}

pub fn verify_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(ProgramError::IllegalOwner)
    } else {
        Ok(())
    }
}
