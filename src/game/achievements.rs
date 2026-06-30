use crate::game::milestones;
use crate::game::clicker::CLICK_UPGRADES;
use crate::game::crafting::RECIPES;
use crate::game::state::GameState;
use crate::game::storyline;

#[derive(Clone, Copy)]
pub struct Achievement {
    pub id: u32,
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
}

pub const ACHIEVEMENTS: &[Achievement] = &[
    Achievement { id: 0, name: "First Footfall", description: "Click the search node for the first time.", icon: "◎" },
    Achievement { id: 1, name: "Rain Walker", description: "Search 100 times.", icon: "☔" },
    Achievement { id: 2, name: "Never Stops", description: "Search 1,000 times.", icon: "∞" },
    Achievement { id: 3, name: "First Clue", description: "Gather 100 lifetime Clues.", icon: "◆" },
    Achievement { id: 4, name: "Street Detective", description: "Gather 10,000 lifetime Clues.", icon: "◇" },
    Achievement { id: 5, name: "Case File", description: "Gather 100,000 lifetime Clues.", icon: "▣" },
    Achievement { id: 6, name: "Million Witness", description: "Gather 1,000,000 lifetime Clues.", icon: "★" },
    Achievement { id: 7, name: "First Ally", description: "Hire your first Buddy.", icon: "◎" },
    Achievement { id: 8, name: "Diaspora Five", description: "Own 5 Buddies total.", icon: "▤" },
    Achievement { id: 9, name: "Network Ten", description: "Own 10 Buddies total.", icon: "▥" },
    Achievement { id: 10, name: "Full Chorus", description: "Own 50 Buddies total.", icon: "♫" },
    Achievement { id: 11, name: "Loyalty Milestone", description: "Reach 25 purchases of a single Buddy.", icon: "25" },
    Achievement { id: 12, name: "Unwavering", description: "Reach 50 purchases of a single Buddy.", icon: "50" },
    Achievement { id: 13, name: "Forever Loyal", description: "Reach 100 purchases of a single Buddy.", icon: "100" },
    Achievement { id: 14, name: "First Tool", description: "Buy your first clicker upgrade.", icon: "↑" },
    Achievement { id: 15, name: "Fully Equipped", description: "Own at least one of all 50 clicker upgrades.", icon: "⚡" },
    Achievement { id: 16, name: "Tinkerer", description: "Craft your first item.", icon: "⚙" },
    Achievement { id: 17, name: "Master Crafter", description: "Craft all 8 items.", icon: "⚒" },
    Achievement { id: 18, name: "Born Again", description: "Perform your first Rebirth.", icon: "↻" },
    Achievement { id: 19, name: "Third Cycle", description: "Perform 3 Rebirths.", icon: "↻↻" },
    Achievement { id: 20, name: "Ascendant", description: "Perform your first Prestige.", icon: "✦" },
    Achievement { id: 21, name: "First Mentor", description: "Hire your first Mentor.", icon: "◇" },
    Achievement { id: 22, name: "Council of Five", description: "Own 5 Mentors total.", icon: "◈" },
    Achievement { id: 23, name: "Badge of Office", description: "Hire your first Authority.", icon: "◆" },
    Achievement { id: 24, name: "The Law Arrives", description: "Own 5 Authorities total.", icon: "⚖" },
    Achievement { id: 25, name: "Chapter Five", description: "Unlock 5 story chapters.", icon: "📖" },
    Achievement { id: 26, name: "Half the Truth", description: "Unlock 12 story chapters.", icon: "📚" },
    Achievement { id: 27, name: "Full Chronicle", description: "Unlock all story chapters.", icon: "📜" },
    Achievement { id: 28, name: "Steady Pulse", description: "Reach 10 Clues per second.", icon: "♥" },
    Achievement { id: 29, name: "Neon Surge", description: "Reach 1,000 Clues per second.", icon: "⚡" },
    Achievement { id: 30, name: "Pattern Mind", description: "Reach 250 purchases of a single Buddy.", icon: "◉" },
];

pub fn achievement_by_id(id: u32) -> Option<&'static Achievement> {
    ACHIEVEMENTS.iter().find(|a| a.id == id)
}

pub fn is_unlocked(state: &GameState, id: u32) -> bool {
    state.achievements_unlocked.contains(&id)
}

pub fn progress(state: &GameState) -> f64 {
    state.achievements_unlocked.len() as f64 / ACHIEVEMENTS.len() as f64
}

fn max_buddy_owned(state: &GameState) -> u32 {
    state.buddy_owned.iter().copied().max().unwrap_or(0)
}

pub fn trigger_met(state: &GameState, id: u32) -> bool {
    let buddies: u32 = state.buddy_owned.iter().sum();
    let mentors: u32 = state.mentor_owned.iter().sum();
    let authorities: u32 = state.authority_owned.iter().sum();
    let crafts: u32 = state.crafts_owned.iter().filter(|&&c| c).count() as u32;
    let max_buddy = max_buddy_owned(state);

    match id {
        0 => state.total_clicks >= 1,
        1 => state.total_clicks >= 100,
        2 => state.total_clicks >= 1_000,
        3 => state.lifetime_clues >= 100.0,
        4 => state.lifetime_clues >= 10_000.0,
        5 => state.lifetime_clues >= 100_000.0,
        6 => state.lifetime_clues >= 1_000_000.0,
        7 => buddies >= 1,
        8 => buddies >= 5,
        9 => buddies >= 10,
        10 => buddies >= 50,
        11 => max_buddy >= milestones::MILESTONE_INTERVAL,
        12 => max_buddy >= 50,
        13 => max_buddy >= 100,
        14 => state.total_click_upgrades_owned() >= 1,
        15 => state.click_upgrade_types_owned() >= CLICK_UPGRADES as u32,
        16 => crafts >= 1,
        17 => crafts >= RECIPES.len() as u32,
        18 => state.total_rebirths >= 1,
        19 => state.total_rebirths >= 3,
        20 => state.total_prestiges >= 1,
        21 => mentors >= 1,
        22 => mentors >= 5,
        23 => authorities >= 1,
        24 => authorities >= 5,
        25 => state.story_unlocked.len() >= 5,
        26 => state.story_unlocked.len() >= 12,
        27 => state.story_unlocked.len() >= storyline::BEATS.len(),
        28 => state.total_cps() >= 10.0,
        29 => state.total_cps() >= 1_000.0,
        30 => max_buddy >= 250,
        _ => false,
    }
}

pub fn try_unlock_next(state: &mut GameState) -> Option<&'static Achievement> {
    for ach in ACHIEVEMENTS {
        if !state.achievements_unlocked.contains(&ach.id) && trigger_met(state, ach.id) {
            state.achievements_unlocked.push(ach.id);
            state.achievement_notification = Some(ach.id);
            state.achievement_notification_timer = 0.0;
            return Some(ach);
        }
    }
    None
}

pub fn backfill_unlocks(state: &mut GameState) {
    for ach in ACHIEVEMENTS {
        if !state.achievements_unlocked.contains(&ach.id) && trigger_met(state, ach.id) {
            state.achievements_unlocked.push(ach.id);
        }
    }
    state.achievements_unlocked.sort_unstable();
    state.achievements_unlocked.dedup();
}