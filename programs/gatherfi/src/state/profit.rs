use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ProfitPool {
    pub event: Pubkey,
    
    // Revenue
    pub total_revenue: u64,      // From ticket sales
    pub other_revenue: u64,      // Sponsorships, etc.
    
    // Expenses
    pub total_expenses: u64,     // Paid from escrow
    pub platform_fee: u64,       // 5% of net profit
    
    // Profit calculation
    pub net_profit: i64,         // Can be negative
    
    // Distribution
    pub backer_share: u64,       // 60%
    pub organizer_share: u64,    // 35%
    pub platform_share: u64,     // 5%
    
    // Status
    pub is_calculated: bool,
    pub is_distributed: bool,
    pub distribution_date: Option<i64>,
    
    // Tracking
    pub backers_paid: u32,
    pub total_backers: u32,
    
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct ProfitClaim {
    pub claimant: Pubkey,
    pub event: Pubkey,
    pub profit_pool: Pubkey,
    pub amount: u64,
    pub claimed_at: i64,
    pub bump: u8,
}