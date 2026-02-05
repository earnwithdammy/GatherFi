use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Budget {
    pub event: Pubkey,
    pub organizer: Pubkey,
    pub items: Vec<BudgetItem>,
    pub total_amount: u64,
    pub amount_spent: u64,
    pub amount_remaining: u64,
    
    // Voting
    pub is_approved: bool,
    pub votes_for: u64,
    pub votes_against: u64,
    pub total_voters: u32,
    pub voting_ends_at: i64,
    
    // Status
    pub is_locked: bool,
    pub is_completed: bool,
    
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct Vote {
    pub voter: Pubkey,
    pub budget: Pubkey,
    pub event: Pubkey,
    pub amount: u64,
    pub approve: bool,
    pub voted_at: i64,
    pub bump: u8,
}