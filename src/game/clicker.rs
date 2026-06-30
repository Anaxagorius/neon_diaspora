pub const CLICK_UPGRADES: usize = 50;

pub struct ClickUpgradeDef {
    pub name: &'static str,
    pub description: &'static str,
}

pub const UPGRADE_DEFS: [ClickUpgradeDef; CLICK_UPGRADES] = [
    ClickUpgradeDef { name: "Worn Sneakers", description: "Scuffed soles grip the rain-slick pavement." },
    ClickUpgradeDef { name: "Street Map Fragment", description: "A torn map piece from the old district." },
    ClickUpgradeDef { name: "Cracked Datapad", description: "Slow but functional — loads one clue at a time." },
    ClickUpgradeDef { name: "Visual Schedule Chip", description: "Glows with your daily search routine. Keeps you on track." },
    ClickUpgradeDef { name: "Mesh Tuner", description: "Picks up stray signals from public nodes." },
    ClickUpgradeDef { name: "Rain Hood", description: "Stay dry. Stay searching." },
    ClickUpgradeDef { name: "Scrap Binoculars", description: "Salvaged optics see three blocks further." },
    ClickUpgradeDef { name: "Pulse Scanner Mk I", description: "Detects heartbeat signatures nearby." },
    ClickUpgradeDef { name: "Graffiti Decoder", description: "Reads diaspora tags for hidden messages." },
    ClickUpgradeDef { name: "Pattern Tracker", description: "Links Clues the way your mind links numbers." },
    ClickUpgradeDef { name: "Signal Amp", description: "Amplifies weak clue transmissions." },
    ClickUpgradeDef { name: "Shadow Cloak", description: "Blend into the underpass shadows." },
    ClickUpgradeDef { name: "Memory Shard", description: "A crystal storing someone's last sighting." },
    ClickUpgradeDef { name: "Neural Patch v2", description: "Reduces search fatigue by 12%." },
    ClickUpgradeDef { name: "Drone Scout", description: "A fist-sized drone scouts ahead." },
    ClickUpgradeDef { name: "Cipher Ring", description: "Decrypts low-level corp chatter." },
    ClickUpgradeDef { name: "Acid-Proof Gloves", description: "Search toxic zones without damage." },
    ClickUpgradeDef { name: "Echo Recorder", description: "Captures ambient voices for analysis." },
    ClickUpgradeDef { name: "Flux Compass", description: "Points toward clue density hotspots." },
    ClickUpgradeDef { name: "Chrome Knuckles", description: "For when the city gets hostile." },
    ClickUpgradeDef { name: "Deep Scan Mk II", description: "Scans beneath the street level." },
    ClickUpgradeDef { name: "Holo Projector", description: "Projects your parents' faces for ID." },
    ClickUpgradeDef { name: "Wire Cutter Pro", description: "Access locked junction boxes." },
    ClickUpgradeDef { name: "Null Filter", description: "Filters noise from real signals." },
    ClickUpgradeDef { name: "Voltage Boots", description: "Electrified soles grip any surface." },
    ClickUpgradeDef { name: "Ghost Protocol Chip", description: "Makes your search signature invisible." },
    ClickUpgradeDef { name: "Quantum Lens", description: "Sees probability trails of movement." },
    ClickUpgradeDef { name: "Dark Fiber Tap", description: "Taps hidden communication lines." },
    ClickUpgradeDef { name: "Pulse Scanner Mk III", description: "Triangulates biometric signatures." },
    ClickUpgradeDef { name: "Override Key", description: "Opens municipal security doors." },
    ClickUpgradeDef { name: "Silicon Imprint", description: "Reads data from dead microchips." },
    ClickUpgradeDef { name: "Neon Resonator", description: "Clues resonate and multiply nearby." },
    ClickUpgradeDef { name: "Cryo Extractor", description: "Pulls frozen evidence from ice blocks." },
    ClickUpgradeDef { name: "Mesh Overdrive", description: "Overclocks your neural search patterns." },
    ClickUpgradeDef { name: "Bit Forge", description: "Crafts clues from raw data fragments." },
    ClickUpgradeDef { name: "Static Shield", description: "Blocks corp counter-surveillance." },
    ClickUpgradeDef { name: "Archivist Link", description: "Direct connection to the lost persons vault." },
    ClickUpgradeDef { name: "Eclipse Recorder", description: "Replays the night they vanished." },
    ClickUpgradeDef { name: "Rebirth Catalyst", description: "Each rebirth empowers your click." },
    ClickUpgradeDef { name: "Prestige Amplifier", description: "Authority backing boosts each click." },
    ClickUpgradeDef { name: "Diaspora Beacon", description: "Broadcasts your search to all allies." },
    ClickUpgradeDef { name: "Neural Forge Mk IV", description: "Military-grade cognitive enhancement." },
    ClickUpgradeDef { name: "Void Walker", description: "Steps through network dead zones." },
    ClickUpgradeDef { name: "Final Cipher", description: "The master key to encrypted clues." },
    ClickUpgradeDef { name: "Parental Resonance", description: "Tuned to your family's unique signature." },
    ClickUpgradeDef { name: "City Heart Sync", description: "Synchronized with the diaspora pulse." },
    ClickUpgradeDef { name: "Omni Scanner", description: "Scans all frequencies simultaneously." },
    ClickUpgradeDef { name: "Truth Engine", description: "Separates lies from evidence instantly." },
    ClickUpgradeDef { name: "Neon Apotheosis", description: "You become the clue the city needs." },
    ClickUpgradeDef { name: "The Last Search", description: "Everything you are, focused on finding them." },
];

pub fn upgrade_base_cost(index: usize) -> f64 {
    15.0 * 1.32_f64.powi(index as i32)
}

pub fn upgrade_base_power(index: usize) -> f64 {
    0.04 * 1.24_f64.powi(index as i32)
}

pub fn upgrade_cost(index: usize, owned: u32) -> f64 {
    upgrade_base_cost(index) * 1.22_f64.powi(owned as i32)
}

pub fn upgrade_click_bonus(index: usize, owned: u32) -> f64 {
    crate::game::milestones::total_output(upgrade_base_power(index), owned, 1.0)
}