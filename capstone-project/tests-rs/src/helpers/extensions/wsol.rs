use {
    crate::helpers::suite::{
        core::{extension::send_tx, token::WithTokenKeys, App, ProgramId},
        types::{AppToken, AppUser},
    },
    anchor_lang::Result,
    litesvm::types::TransactionMetadata,
    solana_program::system_instruction,
    spl_token::instruction as token_instruction,
};

pub trait WsolExtension {
    /// wrap SOL - convert native SOL to wrapped SOL tokens
    fn wsol_try_wrap(&mut self, user: AppUser, amount_lamports: u64)
        -> Result<TransactionMetadata>;

    /// unwrap SOL - convert wrapped SOL tokens back to native SOL
    fn wsol_try_unwrap(&mut self, user: AppUser) -> Result<TransactionMetadata>;
}

impl WsolExtension for App {
    fn wsol_try_wrap(&mut self, user: AppUser, amount: u64) -> Result<TransactionMetadata> {
        let ProgramId { token_program, .. } = self.program_id;

        let payer = user.pubkey();
        let signers = [user.keypair()];

        // ata
        let wsol_ata = self.get_or_create_ata(user, &payer, &AppToken::WSOL.pubkey(self))?;

        // transfer SOL
        let transfer_ix = system_instruction::transfer(&payer, &wsol_ata, amount);

        // sync native instruction to convert SOL to wrapped SOL
        let sync_native_ix = token_instruction::sync_native(&token_program, &wsol_ata)?;

        send_tx(
            &mut self.litesvm,
            &[transfer_ix, sync_native_ix],
            &payer,
            &signers,
        )
    }

    fn wsol_try_unwrap(&mut self, user: AppUser) -> Result<TransactionMetadata> {
        let ProgramId { token_program, .. } = self.program_id;

        let payer = user.pubkey();
        let signers = [user.keypair()];

        // ata
        let wsol_ata = self.get_or_create_ata(user, &payer, &AppToken::WSOL.pubkey(self))?;

        // close the account instruction - this unwraps the SOL
        let close_account_ix =
            token_instruction::close_account(&token_program, &wsol_ata, &payer, &payer, &[])?;

        send_tx(&mut self.litesvm, &[close_account_ix], &payer, &signers)
    }
}
