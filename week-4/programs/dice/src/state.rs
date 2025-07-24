use anchor_lang::prelude::*;

pub const HOUSE_EDGE: u16 = 100;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub bump: u8,
    pub id: u128,
    pub player: Pubkey,
    pub amount: u64,
    pub slot: u64,
    pub roll: u8,
}

impl Bet {
    pub fn to_slice(&self) -> Vec<u8> {
        let mut s = self.player.to_bytes().to_vec();
        s.extend_from_slice(&self.amount.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&self.id.to_le_bytes());
        s.extend_from_slice(&[self.roll, self.bump]);

        s
    }
}
