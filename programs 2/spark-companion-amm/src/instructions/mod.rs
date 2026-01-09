/// ============================================================================
/// SPARKCOMPANION AMM INSTRUCTIONS
/// ============================================================================
/// All instruction handlers for the AMM program.
/// 
/// Labels: [INSTRUCTIONS] [HANDLERS] [ENTRY-POINTS]

pub mod initialize_amm_global;
pub mod create_pool;
pub mod open_position;
pub mod increase_liquidity;
pub mod decrease_liquidity;
pub mod swap;
pub mod collect_fees;
pub mod initialize_tick_array;
pub mod reward_operations;

pub use initialize_amm_global::*;
pub use create_pool::*;
pub use open_position::*;
pub use increase_liquidity::*;
pub use decrease_liquidity::*;
pub use swap::*;
pub use collect_fees::*;
pub use initialize_tick_array::*;
pub use reward_operations::*;
