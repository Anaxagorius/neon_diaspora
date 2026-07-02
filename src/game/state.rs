use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::data::{authorities, buddies, mentors, EntityDef};
use crate::game::achievements;
use crate::game::clicker::{self, CLICK_UPGRADES};
use crate::game::milestones;
use crate::game::crafting::{self, RECIPES};
use crate::game::storyline;

const REBIRTH_THRESHOLD: f64 = 500_000.0;
const PRESTIGE_THRESHOLD: f64 = 25.0;
const ENCOURAGEMENT_INTERVAL_MIN: f64 = 8.0;
const ENCOURAGEMENT_INTERVAL_MAX: f64 = 25.0;
const SAVE_FILE: &str = "neon_diaspora_save.json";
pub const SAVE_SLOTS: usize = 3;
pub const ACHIEVEMENT_BONUS_PER_UNLOCK: f64 = 0.02;

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub clues: f64,
    pub lifetime_clues: f64,
    #[serde(default)]
    pub rebirth_cycle_clues: f64,
    pub rebirth_tokens: f64,
    pub prestige_tokens: f64,
    pub total_rebirths: u32,
    pub total_prestiges: u32,

    #[serde(default)]
    pub click_upgrade_owned: Vec<u32>,

    #[serde(default, alias = "click_upgrade_level", skip_serializing)]
    click_upgrade_level_legacy: u32,
    pub buddy_owned: Vec<u32>,
    pub mentor_owned: Vec<u32>,
    pub authority_owned: Vec<u32>,
    pub crafts_owned: Vec<bool>,

    pub scrap: f64,
    pub data_chips: f64,
    pub neural_filament: f64,

    pub total_clicks: u64,
    pub last_save: u64,

    pub story_unlocked: Vec<u32>,
    pub achievements_unlocked: Vec<u32>,

    #[serde(skip, default)]
    pub achievement_notification: Option<u32>,
    #[serde(skip, default)]
    pub achievement_notification_timer: f64,

    #[serde(skip, default)]
    pub story_notification: Option<u32>,
    #[serde(skip, default)]
    pub story_notification_timer: f64,
    #[serde(skip, default)]
    pub story_check_timer: f64,

    #[serde(skip, default = "Instant::now")]
    pub last_tick: Instant,
    #[serde(skip, default)]
    pub next_encouragement: f64,
    #[serde(skip, default)]
    pub encouragement_timer: f64,
    #[serde(skip, default)]
    pub current_message: Option<EncouragementMsg>,
    #[serde(skip, default)]
    pub message_timer: f64,
    #[serde(skip, default)]
    pub click_pulse: f32,
    #[serde(skip, default)]
    pub click_anim: f32,
}

#[derive(Clone)]
pub struct EncouragementMsg {
    pub speaker: String,
    pub text: String,
    pub kind: MsgKind,
}

#[derive(Clone, Copy, PartialEq)]
pub enum MsgKind {
    Buddy,
    Mentor,
    Authority,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        let mut state = Self {
            clues: 0.0,
            lifetime_clues: 0.0,
            rebirth_cycle_clues: 0.0,
            rebirth_tokens: 0.0,
            prestige_tokens: 0.0,
            total_rebirths: 0,
            total_prestiges: 0,
            click_upgrade_owned: vec![0; CLICK_UPGRADES],
            click_upgrade_level_legacy: 0,
            buddy_owned: vec![0; buddies::BUDDIES.len()],
            mentor_owned: vec![0; mentors::MENTORS.len()],
            authority_owned: vec![0; authorities::AUTHORITIES.len()],
            crafts_owned: vec![false; RECIPES.len()],
            scrap: 0.0,
            data_chips: 0.0,
            neural_filament: 0.0,
            total_clicks: 0,
            last_save: 0,
            story_unlocked: vec![0],
            achievements_unlocked: Vec::new(),
            achievement_notification: None,
            achievement_notification_timer: 0.0,
            story_notification: None,
            story_notification_timer: 0.0,
            story_check_timer: 0.0,
            last_tick: Instant::now(),
            next_encouragement: 12.0,
            encouragement_timer: 0.0,
            current_message: None,
            message_timer: 0.0,
            click_pulse: 0.0,
            click_anim: 0.0,
        };
        state.schedule_next_encouragement();
        state
    }

    fn save_path() -> PathBuf {
        Self::save_path_for_slot(0)
    }

    pub fn save_path_for_slot(slot: usize) -> PathBuf {
        let slot = slot % SAVE_SLOTS;
        dirs_fallback().join(format!(
            "{}_slot_{}.json",
            SAVE_FILE.trim_end_matches(".json"),
            slot + 1
        ))
    }

    fn load_from_path(path: PathBuf) -> Option<Self> {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(mut state) = serde_json::from_str::<GameState>(&data) {
                state.last_tick = Instant::now();
                state.schedule_next_encouragement();
                if state.story_unlocked.is_empty() {
                    storyline::backfill_unlocks(&mut state);
                }
                achievements::backfill_unlocks(&mut state);
                state.migrate_click_upgrades();
                state.migrate_rebirth_cycle();
                return Some(state);
            }
        }
        None
    }

    pub fn load() -> Self {
        Self::load_slot(0)
    }

    pub fn load_slot(slot: usize) -> Self {
        Self::load_from_path(Self::save_path_for_slot(slot)).unwrap_or_else(Self::new)
    }

    pub fn load_slot_preview(slot: usize) -> Option<Self> {
        let path = Self::save_path_for_slot(slot);
        if !path.exists() {
            return None;
        }
        Self::load_from_path(path)
    }

    pub fn save(&self) {
        self.save_to_slot(0);
    }

    pub fn save_to_slot(&self, slot: usize) {
        let path = Self::save_path_for_slot(slot);
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }

    fn schedule_next_encouragement(&mut self) {
        let mut rng = rand::thread_rng();
        self.next_encouragement =
            rng.gen_range(ENCOURAGEMENT_INTERVAL_MIN..ENCOURAGEMENT_INTERVAL_MAX);
        self.encouragement_timer = 0.0;
    }

    pub fn rebirth_multiplier(&self) -> f64 {
        1.0 + self.rebirth_tokens * 0.08 + self.total_rebirths as f64 * 0.05
    }

    pub fn prestige_multiplier(&self) -> f64 {
        1.0 + self.prestige_tokens * 0.12 + self.total_prestiges as f64 * 0.08
    }

    pub fn craft_multiplier(&self) -> f64 {
        let mut mult = 1.0;
        for (i, &owned) in self.crafts_owned.iter().enumerate() {
            if owned {
                mult += RECIPES[i].multiplier_bonus;
            }
        }
        mult
    }

    pub fn achievement_multiplier(&self) -> f64 {
        1.0 + self.achievements_unlocked.len() as f64 * ACHIEVEMENT_BONUS_PER_UNLOCK
    }

    pub fn click_value(&self) -> f64 {
        let mult = self.rebirth_multiplier()
            * self.prestige_multiplier()
            * self.craft_multiplier()
            * self.achievement_multiplier();
        let bonus: f64 = (0..CLICK_UPGRADES)
            .map(|i| {
                let owned = self.click_upgrade_owned.get(i).copied().unwrap_or(0);
                clicker::upgrade_click_bonus(i, owned)
            })
            .sum();
        (1.0 + bonus) * mult
    }

    pub fn total_click_upgrades_owned(&self) -> u32 {
        self.click_upgrade_owned.iter().sum()
    }

    pub fn click_upgrade_types_owned(&self) -> u32 {
        self.click_upgrade_owned.iter().filter(|&&n| n > 0).count() as u32
    }

    pub fn upgrade_per_unit_output(index: usize, owned: u32) -> f64 {
        milestones::per_unit_power(clicker::upgrade_base_power(index), owned)
    }

    fn migrate_rebirth_cycle(&mut self) {
        // Legacy saves tracked rebirth progress on lifetime_clues (never reset on rebirth).
        if self.rebirth_cycle_clues == 0.0 && self.lifetime_clues > 0.0 && self.total_rebirths == 0 {
            self.rebirth_cycle_clues = self.lifetime_clues;
        }
    }

    fn migrate_click_upgrades(&mut self) {
        if self.click_upgrade_owned.len() != CLICK_UPGRADES {
            self.click_upgrade_owned.resize(CLICK_UPGRADES, 0);
        }
        if self.click_upgrade_owned.iter().sum::<u32>() == 0 && self.click_upgrade_level_legacy > 0 {
            for i in 0..(self.click_upgrade_level_legacy as usize).min(CLICK_UPGRADES) {
                self.click_upgrade_owned[i] = 1;
            }
        }
    }

    pub fn entity_cost(def: &EntityDef, owned: u32) -> f64 {
        def.base_cost * 1.22_f64.powi(owned as i32)
    }

    fn preview_purchase(
        owned: u32,
        currency: f64,
        quantity: Option<u32>,
        mut cost_fn: impl FnMut(u32) -> f64,
    ) -> (u32, f64) {
        let limit = quantity.unwrap_or(u32::MAX);
        let mut purchased = 0;
        let mut total_cost = 0.0;

        while purchased < limit {
            let cost = cost_fn(owned + purchased);
            if total_cost + cost > currency + f64::EPSILON {
                break;
            }
            total_cost += cost;
            purchased += 1;
        }

        (purchased, total_cost)
    }

    pub fn buddy_cps(&self) -> f64 {
        let mult = self.rebirth_multiplier() * self.craft_multiplier() * self.achievement_multiplier();
        buddies::BUDDIES
            .iter()
            .zip(self.buddy_owned.iter())
            .map(|(def, &n)| milestones::total_output(def.base_power, n, mult))
            .sum()
    }

    pub fn buddy_per_unit_output(base_power: f64, owned: u32) -> f64 {
        milestones::per_unit_power(base_power, owned)
    }

    pub fn mentor_cps(&self) -> f64 {
        let mult = self.rebirth_multiplier()
            * self.prestige_multiplier()
            * self.achievement_multiplier()
            * 0.5;
        mentors::MENTORS
            .iter()
            .zip(self.mentor_owned.iter())
            .map(|(def, &n)| def.base_power * n as f64 * mult)
            .sum()
    }

    pub fn authority_cps(&self) -> f64 {
        let mult = self.prestige_multiplier() * self.craft_multiplier() * self.achievement_multiplier();
        authorities::AUTHORITIES
            .iter()
            .zip(self.authority_owned.iter())
            .map(|(def, &n)| def.base_power * n as f64 * mult)
            .sum()
    }

    pub fn total_cps(&self) -> f64 {
        self.buddy_cps() + self.mentor_cps() + self.authority_cps()
    }

    pub fn click(&mut self) {
        let val = self.click_value();
        self.clues += val;
        self.lifetime_clues += val;
        self.rebirth_cycle_clues += val;
        self.total_clicks += 1;
        self.click_pulse = 1.0;
        self.click_anim = 1.0;
        self.check_progress();
    }

    pub fn check_progress(&mut self) {
        achievements::try_unlock_next(self);
        storyline::try_unlock_next(self);
    }

    pub fn tick(&mut self, dt: f64) {
        let cps = self.total_cps();
        if cps > 0.0 {
            self.clues += cps * dt;
            self.lifetime_clues += cps * dt;
            self.rebirth_cycle_clues += cps * dt;
        }

        let buddies_count: u32 = self.buddy_owned.iter().sum();
        let mentors_count: u32 = self.mentor_owned.iter().sum();
        let (scrap_r, chip_r, fil_r) = crafting::resource_gather_rate(buddies_count, mentors_count);
        self.scrap += scrap_r * dt;
        self.data_chips += chip_r * dt;
        self.neural_filament += fil_r * dt;

        if self.click_pulse > 0.0 {
            self.click_pulse = (self.click_pulse - dt as f32 * 3.0).max(0.0);
        }
        if self.click_anim > 0.0 {
            self.click_anim = (self.click_anim - dt as f32 * 2.5).max(0.0);
        }

        if self.current_message.is_some() {
            self.message_timer += dt;
            if self.message_timer > 6.0 {
                self.current_message = None;
            }
        }

        self.encouragement_timer += dt;
        if self.encouragement_timer >= self.next_encouragement {
            self.try_encouragement();
            self.schedule_next_encouragement();
        }

        if self.achievement_notification.is_some() {
            self.achievement_notification_timer += dt;
            if self.achievement_notification_timer > 10.0 {
                self.achievement_notification = None;
            }
        }

        if self.story_notification.is_some() {
            self.story_notification_timer += dt;
            if self.story_notification_timer > 12.0 {
                self.story_notification = None;
            }
        }

        self.story_check_timer += dt;
        if self.story_check_timer >= 2.0 {
            self.story_check_timer = 0.0;
            self.check_progress();
        }
    }

    fn try_encouragement(&mut self) {
        let mut pool: Vec<(MsgKind, usize, usize)> = Vec::new();

        for (i, &n) in self.buddy_owned.iter().enumerate() {
            if n > 0 {
                pool.push((MsgKind::Buddy, i, rand::thread_rng().gen_range(0..n as usize)));
            }
        }
        for (i, &n) in self.mentor_owned.iter().enumerate() {
            if n > 0 {
                pool.push((MsgKind::Mentor, i, 0));
            }
        }
        for (i, &n) in self.authority_owned.iter().enumerate() {
            if n > 0 {
                pool.push((MsgKind::Authority, i, 0));
            }
        }

        if pool.is_empty() {
            return;
        }

        let idx = rand::thread_rng().gen_range(0..pool.len());
        let (kind, entity_idx, _) = pool[idx];

        let (name, messages) = match kind {
            MsgKind::Buddy => {
                let def = &buddies::BUDDIES[entity_idx];
                (def.name, def.messages)
            }
            MsgKind::Mentor => {
                let def = &mentors::MENTORS[entity_idx];
                (def.name, def.messages)
            }
            MsgKind::Authority => {
                let def = &authorities::AUTHORITIES[entity_idx];
                (def.name, def.messages)
            }
        };

        let msg_idx = rand::thread_rng().gen_range(0..messages.len());
        self.current_message = Some(EncouragementMsg {
            speaker: name.to_string(),
            text: messages[msg_idx].to_string(),
            kind,
        });
        self.message_timer = 0.0;
    }

    pub fn buy_click_upgrade(&mut self, index: usize) -> bool {
        self.buy_click_upgrade_quantity(index, Some(1)) > 0
    }

    pub fn preview_click_upgrade_purchase(&self, index: usize, quantity: Option<u32>) -> (u32, f64) {
        if index >= CLICK_UPGRADES {
            return (0, 0.0);
        }
        let owned = self.click_upgrade_owned[index];
        Self::preview_purchase(owned, self.clues, quantity, |current_owned| {
            clicker::upgrade_cost(index, current_owned)
        })
    }

    pub fn buy_click_upgrade_quantity(&mut self, index: usize, quantity: Option<u32>) -> u32 {
        if index >= CLICK_UPGRADES {
            return 0;
        }
        let (purchased, total_cost) = self.preview_click_upgrade_purchase(index, quantity);
        if purchased == 0 {
            return 0;
        }
        self.clues -= total_cost;
        self.click_upgrade_owned[index] += purchased;
        self.check_progress();
        purchased
    }

    pub fn buy_buddy(&mut self, index: usize) -> bool {
        self.buy_buddy_quantity(index, Some(1)) > 0
    }

    pub fn preview_buddy_purchase(&self, index: usize, quantity: Option<u32>) -> (u32, f64) {
        if index >= buddies::BUDDIES.len() {
            return (0, 0.0);
        }
        let def = &buddies::BUDDIES[index];
        let owned = self.buddy_owned[index];
        Self::preview_purchase(owned, self.clues, quantity, |current_owned| {
            Self::entity_cost(def, current_owned)
        })
    }

    pub fn buy_buddy_quantity(&mut self, index: usize, quantity: Option<u32>) -> u32 {
        if index >= buddies::BUDDIES.len() {
            return 0;
        }
        let (purchased, total_cost) = self.preview_buddy_purchase(index, quantity);
        if purchased == 0 {
            return 0;
        }
        self.clues -= total_cost;
        self.buddy_owned[index] += purchased;
        self.check_progress();
        purchased
    }

    pub fn buy_mentor(&mut self, index: usize) -> bool {
        self.buy_mentor_quantity(index, Some(1)) > 0
    }

    pub fn preview_mentor_purchase(&self, index: usize, quantity: Option<u32>) -> (u32, f64) {
        if index >= mentors::MENTORS.len() {
            return (0, 0.0);
        }
        let def = &mentors::MENTORS[index];
        let owned = self.mentor_owned[index];
        Self::preview_purchase(owned, self.rebirth_tokens, quantity, |current_owned| {
            Self::entity_cost(def, current_owned)
        })
    }

    pub fn buy_mentor_quantity(&mut self, index: usize, quantity: Option<u32>) -> u32 {
        if index >= mentors::MENTORS.len() {
            return 0;
        }
        let (purchased, total_cost) = self.preview_mentor_purchase(index, quantity);
        if purchased == 0 {
            return 0;
        }
        self.rebirth_tokens -= total_cost;
        self.mentor_owned[index] += purchased;
        self.check_progress();
        purchased
    }

    pub fn buy_authority(&mut self, index: usize) -> bool {
        self.buy_authority_quantity(index, Some(1)) > 0
    }

    pub fn preview_authority_purchase(&self, index: usize, quantity: Option<u32>) -> (u32, f64) {
        if index >= authorities::AUTHORITIES.len() {
            return (0, 0.0);
        }
        let def = &authorities::AUTHORITIES[index];
        let owned = self.authority_owned[index];
        Self::preview_purchase(owned, self.prestige_tokens, quantity, |current_owned| {
            Self::entity_cost(def, current_owned)
        })
    }

    pub fn buy_authority_quantity(&mut self, index: usize, quantity: Option<u32>) -> u32 {
        if index >= authorities::AUTHORITIES.len() {
            return 0;
        }
        let (purchased, total_cost) = self.preview_authority_purchase(index, quantity);
        if purchased == 0 {
            return 0;
        }
        self.prestige_tokens -= total_cost;
        self.authority_owned[index] += purchased;
        self.check_progress();
        purchased
    }

    pub fn craft_item(&mut self, index: usize) -> bool {
        if index >= RECIPES.len() || self.crafts_owned[index] {
            return false;
        }
        let recipe = &RECIPES[index];
        if self.scrap < recipe.scrap_cost
            || self.data_chips < recipe.chip_cost
            || self.neural_filament < recipe.filament_cost
            || self.clues < recipe.clue_cost
        {
            return false;
        }
        self.scrap -= recipe.scrap_cost;
        self.data_chips -= recipe.chip_cost;
        self.neural_filament -= recipe.filament_cost;
        self.clues -= recipe.clue_cost;
        self.crafts_owned[index] = true;
        self.check_progress();
        true
    }

    pub fn rebirth_available(&self) -> bool {
        self.rebirth_cycle_clues >= REBIRTH_THRESHOLD
    }

    pub fn rebirth_preview_tokens(&self) -> f64 {
        if !self.rebirth_available() {
            return 0.0;
        }
        let base = ((self.rebirth_cycle_clues / REBIRTH_THRESHOLD).sqrt()).floor();
        let craft_bonus = self.craft_multiplier();
        (base * craft_bonus * 0.8).floor().max(1.0)
    }

    pub fn perform_rebirth(&mut self) -> bool {
        if !self.rebirth_available() {
            return false;
        }
        let tokens = self.rebirth_preview_tokens();
        self.rebirth_tokens += tokens;
        self.total_rebirths += 1;

        self.clues = 0.0;
        self.rebirth_cycle_clues = 0.0;
        self.buddy_owned = vec![0; buddies::BUDDIES.len()];
        self.scrap = self.scrap * 0.1;
        self.data_chips = self.data_chips * 0.1;
        self.neural_filament = self.neural_filament * 0.1;

        self.check_progress();
        true
    }

    pub fn prestige_available(&self) -> bool {
        self.rebirth_tokens >= PRESTIGE_THRESHOLD && self.total_rebirths >= 3
    }

    pub fn prestige_preview_tokens(&self) -> f64 {
        if !self.prestige_available() {
            return 0.0;
        }
        let base = (self.rebirth_tokens / PRESTIGE_THRESHOLD).sqrt().floor();
        base.max(1.0)
    }

    pub fn perform_prestige(&mut self) -> bool {
        if !self.prestige_available() {
            return false;
        }
        let tokens = self.prestige_preview_tokens();
        self.prestige_tokens += tokens;
        self.total_prestiges += 1;

        self.clues = 0.0;
        self.lifetime_clues = 0.0;
        self.rebirth_cycle_clues = 0.0;
        self.rebirth_tokens = 0.0;
        self.buddy_owned = vec![0; buddies::BUDDIES.len()];
        self.mentor_owned = vec![0; mentors::MENTORS.len()];
        for owned in &mut self.click_upgrade_owned {
            *owned /= 2;
        }

        self.check_progress();
        true
    }

    pub fn rebirth_progress(&self) -> f64 {
        (self.rebirth_cycle_clues / REBIRTH_THRESHOLD).min(1.0)
    }

    pub fn prestige_progress(&self) -> f64 {
        if self.total_rebirths < 3 {
            return 0.0;
        }
        (self.rebirth_tokens / PRESTIGE_THRESHOLD).min(1.0)
    }
}

fn dirs_fallback() -> PathBuf {
    if let Some(dir) = std::env::var_os("APPDATA") {
        let path = PathBuf::from(dir).join("NeonDiaspora");
        let _ = fs::create_dir_all(&path);
        return path;
    }
    PathBuf::from(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn achievement_bonus_scales_per_unlock() {
        let mut state = GameState::new();
        assert_eq!(state.achievement_multiplier(), 1.0);
        state.achievements_unlocked.extend([1, 2, 3]);
        assert!((state.achievement_multiplier() - 1.06).abs() < f64::EPSILON);
    }

    #[test]
    fn preview_purchase_limits_to_affordable_count() {
        let mut state = GameState::new();
        state.clues = 100.0;

        let (count, total_cost) = state.preview_buddy_purchase(0, Some(10));

        assert!(count > 0);
        assert!(count < 10);
        assert!(total_cost <= state.clues);
    }

    #[test]
    fn preview_purchase_supports_max_mode() {
        let mut state = GameState::new();
        state.clues = 10_000.0;

        let limited = state.preview_click_upgrade_purchase(0, Some(10)).0;
        let maxed = state.preview_click_upgrade_purchase(0, None).0;

        assert!(maxed >= limited);
    }
}