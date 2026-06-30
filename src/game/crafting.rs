pub struct CraftRecipe {
    pub name: &'static str,
    pub description: &'static str,
    pub scrap_cost: f64,
    pub chip_cost: f64,
    pub filament_cost: f64,
    pub clue_cost: f64,
    pub multiplier_bonus: f64,
}

pub const RECIPES: &[CraftRecipe] = &[
    CraftRecipe { name: "Signal Sniffer", description: "+5% clue gain from all sources", scrap_cost: 50.0, chip_cost: 10.0, filament_cost: 5.0, clue_cost: 500.0, multiplier_bonus: 0.05 },
    CraftRecipe { name: "Memory Weave", description: "+8% click power", scrap_cost: 80.0, chip_cost: 20.0, filament_cost: 10.0, clue_cost: 1200.0, multiplier_bonus: 0.08 },
    CraftRecipe { name: "Pulse Anchor", description: "+10% buddy efficiency", scrap_cost: 120.0, chip_cost: 35.0, filament_cost: 15.0, clue_cost: 3000.0, multiplier_bonus: 0.10 },
    CraftRecipe { name: "Neon Lattice", description: "+12% idle generation", scrap_cost: 200.0, chip_cost: 60.0, filament_cost: 30.0, clue_cost: 8000.0, multiplier_bonus: 0.12 },
    CraftRecipe { name: "Cipher Engine", description: "+15% rebirth token gain", scrap_cost: 350.0, chip_cost: 100.0, filament_cost: 50.0, clue_cost: 25000.0, multiplier_bonus: 0.15 },
    CraftRecipe { name: "Void Compass", description: "+18% mentor power", scrap_cost: 500.0, chip_cost: 150.0, filament_cost: 80.0, clue_cost: 75000.0, multiplier_bonus: 0.18 },
    CraftRecipe { name: "Authority Seal", description: "+20% prestige token gain", scrap_cost: 800.0, chip_cost: 250.0, filament_cost: 120.0, clue_cost: 200000.0, multiplier_bonus: 0.20 },
    CraftRecipe { name: "Diaspora Core", description: "+25% all generation", scrap_cost: 1500.0, chip_cost: 500.0, filament_cost: 250.0, clue_cost: 1000000.0, multiplier_bonus: 0.25 },
];

pub fn resource_gather_rate(buddies_owned: u32, mentors_owned: u32) -> (f64, f64, f64) {
    let scrap = 0.02 + buddies_owned as f64 * 0.005 + mentors_owned as f64 * 0.02;
    let chips = 0.005 + buddies_owned as f64 * 0.001 + mentors_owned as f64 * 0.005;
    let filament = 0.002 + buddies_owned as f64 * 0.0005 + mentors_owned as f64 * 0.002;
    (scrap, chips, filament)
}