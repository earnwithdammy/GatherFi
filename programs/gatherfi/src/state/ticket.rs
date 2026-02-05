use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TicketType {
    Regular,
    VIP,
    EarlyBird,
    Student,
    Group,
    VVIP,
    Backstage,
    Table,
}

#[account]
#[derive(Default)]
pub struct Ticket {
    pub mint: Pubkey,
    pub event: Pubkey,
    pub owner: Pubkey,
    pub ticket_number: u32,
    pub ticket_type: TicketType,
    pub zone: String,
    pub seat: Option<String>,
    
    // Status
    pub is_checked_in: bool,
    pub is_refunded: bool,
    pub is_transferred: bool,
    
    // Financial
    pub purchase_price: u64,
    pub purchase_time: i64,
    
    // Check-in
    pub checked_in_time: Option<i64>,
    pub check_in_staff: Option<Pubkey>,
    
    // Metadata
    pub metadata_uri: String,
    
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct TicketCounter {
    pub count: u32,
    pub bump: u8,
}