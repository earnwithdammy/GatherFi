use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EventCategory {
    Owambe,         // Traditional Nigerian party
    Concert,        // Music concert
    TechMeetup,     // Developer/tech event
    Wedding,        // Wedding ceremony
    ChurchEvent,    // Religious gathering
    CampusEvent,    // University event
    Conference,     // Business conference
    Festival,       // Cultural festival
    Sports,         // Sporting event
    Other,
}

#[account]
#[derive(Default)]
pub struct Event {
    // Basic info
    pub organizer: Pubkey,
    pub name: String,
    pub description: String,
    pub category: EventCategory,
    
    // Funding
    pub target_amount: u64,
    pub amount_raised: u64,
    pub min_contribution: u64,
    
    // Ticketing
    pub ticket_price: u64,
    pub tickets_sold: u32,
    pub max_tickets: u32,
    pub revenue_from_tickets: u64,
    
    // Timing & location
    pub event_date: i64,
    pub location: String,
    pub city: String,
    pub state: String,
    pub country: String,  // Always "Nigeria"
    
    // Status flags
    pub is_active: bool,
    pub is_funded: bool,
    pub is_cancelled: bool,
    pub is_paused: bool,
    pub is_finalized: bool,
    
    // Governance
    pub total_backers: u32,
    pub total_votes: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub voting_ends_at: i64,
    
    // Timestamps
    pub created_at: i64,
    pub updated_at: i64,
    pub funding_deadline: i64,
    
    // PDAs
    pub escrow: Pubkey,
    pub profit_pool: Pubkey,
    pub budget: Pubkey,
    
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct Contribution {
    pub contributor: Pubkey,
    pub event: Pubkey,
    pub amount: u64,
    pub voting_power: u64,  // 1 lamport = 1 vote
    pub claimed_profits: u64,
    pub claimed_refund: bool,
    pub created_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BudgetItem {
    pub name: String,
    pub description: String,
    pub amount: u64,
    pub vendor: String,
    pub category: BudgetCategory,
    pub is_paid: bool,
    pub paid_at: Option<i64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BudgetCategory {
    Venue,
    Catering,
    Entertainment,
    Logistics,
    Marketing,
    Staff,
    Equipment,
    Other,
}

// Nigerian cities and states validation
pub const NIGERIAN_STATES: [&str; 37] = [
    "Abia", "Adamawa", "Akwa Ibom", "Anambra", "Bauchi", "Bayelsa", "Benue", "Borno", 
    "Cross River", "Delta", "Ebonyi", "Edo", "Ekiti", "Enugu", "FCT", "Gombe", 
    "Imo", "Jigawa", "Kaduna", "Kano", "Katsina", "Kebbi", "Kogi", "Kwara", 
    "Lagos", "Nasarawa", "Niger", "Ogun", "Ondo", "Osun", "Oyo", "Plateau", 
    "Rivers", "Sokoto", "Taraba", "Yobe", "Zamfara"
];

pub const NIGERIAN_CITIES: [(&str, &str); 20] = [
    ("Lagos", "Lagos"),
    ("Abuja", "FCT"),
    ("Port Harcourt", "Rivers"),
    ("Ibadan", "Oyo"),
    ("Kano", "Kano"),
    ("Benin City", "Edo"),
    ("Kaduna", "Kaduna"),
    ("Abeokuta", "Ogun"),
    ("Jos", "Plateau"),
    ("Ilorin", "Kwara"),
    ("Owerri", "Imo"),
    ("Enugu", "Enugu"),
    ("Calabar", "Cross River"),
    ("Uyo", "Akwa Ibom"),
    ("Akure", "Ondo"),
    ("Sokoto", "Sokoto"),
    ("Maiduguri", "Borno"),
    ("Yola", "Adamawa"),
    ("Bauchi", "Bauchi"),
    ("Makurdi", "Benue"),
];

pub fn validate_nigerian_location(location: &str) -> Result<(String, String)> {
    for (city, state) in NIGERIAN_CITIES.iter() {
        if location.contains(city) {
            return Ok((city.to_string(), state.to_string()));
        }
    }
    Err(error!(GatherFiError::InvalidNigerianCity))
}