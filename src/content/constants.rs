pub const MAX_GAME_SECONDS: u32 = 300;

/// Based on a day from 8AM to 22PM (14 hours)
///
/// -> 8AM to midday (4 hours)
///
/// -> 4 / 14 = 0.28 * `MAX_GAME_SECONDS`
pub const MIDDAY_TIME: u32 = (MAX_GAME_SECONDS as f32 * 0.28) as u32;

/// 0.79 -> based on 8AM to 22PM (14 hours) -> 8AM to evening (11 hours) -> 11 / 14 = 0.79
pub const EVENING_TIME: u32 = (MAX_GAME_SECONDS as f32 * 0.79) as u32;
