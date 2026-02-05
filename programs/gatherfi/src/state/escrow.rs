use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Escrow {
    pub event: Pubkey,
    pub total_amount: u64,
    pub released_amount: u64,
    pub balance: u64,
    
    // Milestones
    pub milestone_count: u8,
    pub current_milestone: u8,
    pub milestones: Vec<Milestone>,
    
    // Security
    pub is_locked: bool,
    pub requires_approval: bool,
    pub approvers: Vec<Pubkey>,
    pub approvals_needed: u8,
    
    pub created_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Milestone {
    pub index: u8,
    pub description: String,
    pub amount: u64,
    pub due_date: i64,
    pub is_released: bool,
    pub released_at: Option<i64>,
    pub released_by: Option<Pubkey>,
    pub requires_vote: bool,
}