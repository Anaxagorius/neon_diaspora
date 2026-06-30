use crate::game::clicker::CLICK_UPGRADES;
use crate::game::state::GameState;

#[derive(Clone, Copy)]
pub struct StoryBeat {
    pub id: u32,
    pub chapter: &'static str,
    pub title: &'static str,
    pub text: &'static str,
}

pub const BEATS: &[StoryBeat] = &[
    StoryBeat {
        id: 0,
        chapter: "I. THE NIGHT OF THE ECLIPSE",
        title: "Where It Began",
        text: "They called it an eclipse, but the sky didn't go dark — it went wrong. Colors inverted. \
               Rain fell upward for eleven seconds. When gravity corrected itself, your parents were gone. \
               You were sixteen, born with Down syndrome in a city that files people like you under \
               'non-viable.' The corps looked right through you that night. That was three years ago. \
               You still search. You always will.",
    },
    StoryBeat {
        id: 1,
        chapter: "II. SECTOR 7",
        title: "First Search",
        text: "Your visual schedule app glows on a cracked datapad — SEARCH, REST, SEARCH again. Sector 7 \
               smells like fried noodles and burnt circuitry. You crouch beneath a flickering holo-sign \
               and tap the mesh node three times the way your mother taught you. A single Clue loads. \
               A timestamp. A blurred face. A direction. It isn't much. It is everything.",
    },
    StoryBeat {
        id: 2,
        chapter: "II. SECTOR 7",
        title: "The Pattern",
        text: "Ten searches. Ten fragments. Your mind doesn't work the way corp scanners expect — \
               you see patterns in numbers the way others see faces. The Clues form a rhythm your \
               mother would recognize. She taught you counting games before the vanishing. Someone is \
               leaving these for you. Or someone is leading you.",
    },
    StoryBeat {
        id: 3,
        chapter: "II. SECTOR 7",
        title: "Noodle Stall Memory",
        text: "A hundred Clues deep and the rain hasn't stopped. You pass the noodle stall where your \
               father used to buy you dinner on credit. The vendor looks through you — past you — the \
               way the whole sprawl does when it sees Down syndrome first and a son second. You don't \
               blame him. But the stall's security cam still runs. You flag it. Clue #112.",
    },
    StoryBeat {
        id: 4,
        chapter: "III. THE DIASPORA",
        title: "Not Alone",
        text: "You can't search every alley alone with one cracked datapad and a world that underestimates \
               you. The diaspora — the cast-out, the augmented, the forgotten — watches you work. \
               A Neon Ghost offers to scrape signals while you sleep against a transformer box. You pay \
               in Clues. For the first time since the eclipse, you are not invisible in this.",
    },
    StoryBeat {
        id: 5,
        chapter: "III. THE DIASPORA",
        title: "The Network Forms",
        text: "Five allies now. Five pairs of eyes in five districts. Packet Rats tunnel through data. \
               Chrome Nomads map the underpasses. A Rain-Slick Ronin says he knows what it's like when \
               the city decides you're not worth saving. They don't ask why you search. They know. \
               Everyone in the diaspora lost someone the corps pretended never existed.",
    },
    StoryBeat {
        id: 6,
        chapter: "IV. TOOLS OF THE SEARCH",
        title: "Worn Sneakers",
        text: "You salvage your first upgrade from a dead runner's stash — sneakers with grip polymer \
               still sticky on the soles. Long walks hurt your joints but you don't stop. The sneakers \
               help. Every tool you carry is an argument against giving up. Your mother used to say \
               persistence is a technology anyone can learn.",
    },
    StoryBeat {
        id: 7,
        chapter: "IV. TOOLS OF THE SEARCH",
        title: "Deeper Into the Grid",
        text: "A thousand Clues and the city opens reluctantly. You find your mother's old transit pass \
               — deactivated, but the chip still holds her route history. She rode the mag-lev to \
               Industrial Ward every Tuesday. Your father didn't. They weren't taken together at home. \
               They were taken apart, on purpose.",
    },
    StoryBeat {
        id: 8,
        chapter: "III. THE DIASPORA",
        title: "Critical Mass",
        text: "Ten allies spread across the sprawl like a nervous system. Clues flow in while you \
               sort them by color the way your visual schedule taught you — red for urgent, blue for \
               follow-up. Someone reports movement near the quarantine fence — two figures, hooded, \
               matching your parents' height profiles. By the time you arrive, only wet footprints remain.",
    },
    StoryBeat {
        id: 9,
        chapter: "IV. TOOLS OF THE SEARCH",
        title: "Pattern Tracker",
        text: "Upgrade ten: a pattern tracker that maps Clue connections the way your mind already does \
               — luminous threads between street corners, transit hubs, dead cameras. The corps never \
               believed you could hold this much data in your head. They were wrong. The search is no \
               longer random. It is a map. And maps have destinations.",
    },
    StoryBeat {
        id: 10,
        chapter: "V. THE UNDERCITY",
        title: "Scrap and Memory",
        text: "Below the streets, the undercity breathes. Your allies haul scrap from shattered server \
               farms — copper, silicon, memory filament tangled like nerve tissue. You learn to build here. \
               Not weapons. Tools. Every crafted device is a question posed to the city: where are they?",
    },
    StoryBeat {
        id: 11,
        chapter: "V. THE UNDERCITY",
        title: "The Signal Sniffer",
        text: "Your first craft: a signal sniffer tuned to your family's pulse-code. It whines when you \
               pass certain buildings, a dog that smells ghosts. On Block C it nearly deafens you. \
               You mark the building. You mark the fear. You do not mark retreat.",
    },
    StoryBeat {
        id: 12,
        chapter: "VI. INDUSTRIAL WARD",
        title: "Ten Thousand Clues",
        text: "Industrial Ward reeks of coolant and compliance. Corp towers lean over the streets like \
               judges. Ten thousand Clues point here — detention logistics, unmarked vans, power surges \
               on the eclipse night. Your legs ache on the grate but you keep going. A Grid Rat steadies \
               you. 'Warehouse district,' it whispers. 'They moved people through warehouses.'",
    },
    StoryBeat {
        id: 13,
        chapter: "VI. INDUSTRIAL WARD",
        title: "Halfway Up",
        text: "Twenty-five upgrades deep and the corps still underestimate you — a young man with Down \
               syndrome chasing clues they thought he'd never find. You are not a netrunner. You are a son \
               with a forged compass and a refusal that outlasts titanium. The Clues converge on Warehouse 12. \
               Sealed. Guarded. Real.",
    },
    StoryBeat {
        id: 14,
        chapter: "VII. REBIRTH",
        title: "Burn It Down to Remember",
        text: "You rebirth. Sacrifice everything you built — the allies, the Clues, the momentum — \
               for something the corps can't confiscate: memory sharpened into instinct. You wake in \
               Sector 7 again. Same rain. Same datapad. Different eyes. You know which alleys matter now. \
               The search restarts faster.",
    },
    StoryBeat {
        id: 15,
        chapter: "VIII. MENTORS",
        title: "Cipher Sage Kael",
        text: "A Mentor finds you in the ruins of a burned academy. Kael lost students to the same \
               relocation program that took your parents. He doesn't offer pity. He offers structure: \
               how to read corp denial as confirmation, how to turn rebirth tokens into doctrine. \
               'You are not broken,' he says. 'You are evidence they failed to erase.'",
    },
    StoryBeat {
        id: 16,
        chapter: "VII. REBIRTH",
        title: "The Third Cycle",
        text: "Three rebirths. Three times you've torn down your network and rebuilt it sharper. The \
               diaspora speaks your name now — not your real name, the one your parents gave you, but \
               the one the streets gave you: The Searcher. You hate it. You need it. Prestige waits \
               beyond the next sacrifice.",
    },
    StoryBeat {
        id: 17,
        chapter: "VIII. MENTORS",
        title: "The Lesson",
        text: "Five Mentors guide you now. Dr. Vex Null maps neural pathways. Mother Wire rebuilds your \
               mesh contacts. The Archivist 7 pulls files the corps deleted. Chrome Tutor Rex says your \
               mind was never the problem — the city's prejudice was. Each lesson costs rebirth tokens. \
               You pay gladly.",
    },
    StoryBeat {
        id: 18,
        chapter: "IX. THE AUTHORITIES",
        title: "Commissioner Vex",
        text: "Prestige buys what Clues cannot: jurisdiction. Commissioner Vex opens File 7741 with \
               a hand that shakes only once. 'Your parents were flagged as containment risks,' she says. \
               'The eclipse wasn't astronomical. It was a cover for extraction.' You sit down, breathe \
               slow the way your therapist taught you. At least now you know the enemy has a name.",
    },
    StoryBeat {
        id: 19,
        chapter: "IX. THE AUTHORITIES",
        title: "Official Power",
        text: "The Authorities move through the sprawl like winter — slow, cold, unstoppable. Raids on \
               black sites. Warrants the corps can't bury. Agent Chrome finds a detention ledger with \
               two entries matching your parents' biometrics. Status: TRANSFERRED. Destination: redacted. \
               You redact the redaction.",
    },
    StoryBeat {
        id: 20,
        chapter: "X. WAREHOUSE 12",
        title: "The Door",
        text: "Every upgrade maxed. Every craft humming. Every ally, Mentor, and Authority converging \
               on a single rusted door in Industrial Ward. Warehouse 12 smells like your mother's \
               perfume mixed with sterilizer. Your heart races but you count to ten and keep walking. \
               You knock. The door knocks back.",
    },
    StoryBeat {
        id: 21,
        chapter: "X. WAREHOUSE 12",
        title: "Inside",
        text: "They aren't dead. They aren't free. Your parents sit in containment pods behind glass \
               that reflects your face back younger, more whole. Your mother sees you and mouths a word \
               you can't hear through the barrier. Your father pounds once. Once. The eclipse wasn't \
               the end of the search. It was the middle.",
    },
    StoryBeat {
        id: 22,
        chapter: "XI. THE FINAL SEARCH",
        title: "One Million Clues",
        text: "A million Clues and the city finally confesses. The relocation program wasn't random \
               abduction — it was harvesting people with a rare neural resonance your family carries. \
               The eclipse masked the extraction. You carry the same resonance — the pattern mind they \
               labeled defective. That's why you survived. That's why you can't stop.",
    },
    StoryBeat {
        id: 23,
        chapter: "XI. THE FINAL SEARCH",
        title: "Diaspora Rising",
        text: "The whole diaspora marches on Warehouse 12. Runners, chrome nomads, wire widows, \
               ghost monks — everyone the city threw away, returned for what the city stole. You walk \
               at the front. Not because you're fastest. Because this search started with you, and \
               you will finish it.",
    },
    StoryBeat {
        id: 24,
        chapter: "XII. NEON DIASPORA",
        title: "The Reunion Waits",
        text: "You haven't found them yet. Not fully. Not free. But for the first time in three years, \
               you know where they are, why they were taken, and who took them. The rain still falls in \
               Sector 7. The Clues still gather. The search continues — but now the city searches with \
               you. And somewhere behind glass, your parents wait for the son who never stopped looking.",
    },
];

pub fn trigger_met(state: &GameState, beat_id: u32) -> bool {
    let buddies: u32 = state.buddy_owned.iter().sum();
    let mentors: u32 = state.mentor_owned.iter().sum();
    let authorities: u32 = state.authority_owned.iter().sum();
    let crafts: u32 = state.crafts_owned.iter().filter(|&&c| c).count() as u32;

    match beat_id {
        0 => true,
        1 => state.total_clicks >= 1,
        2 => state.total_clicks >= 10,
        3 => state.lifetime_clues >= 100.0,
        4 => buddies >= 1,
        5 => buddies >= 5,
        6 => state.total_click_upgrades_owned() >= 1,
        7 => state.lifetime_clues >= 1_000.0,
        8 => buddies >= 10,
        9 => state.total_click_upgrades_owned() >= 10,
        10 => state.scrap >= 50.0,
        11 => crafts >= 1,
        12 => state.lifetime_clues >= 10_000.0,
        13 => state.total_click_upgrades_owned() >= 25,
        14 => state.total_rebirths >= 1,
        15 => mentors >= 1,
        16 => state.total_rebirths >= 3,
        17 => mentors >= 5,
        18 => state.total_prestiges >= 1,
        19 => authorities >= 1,
        20 => state.click_upgrade_types_owned() >= CLICK_UPGRADES as u32,
        21 => state.lifetime_clues >= 1_000_000.0,
        22 => buddies >= 30,
        23 => authorities >= 5,
        24 => state.total_prestiges >= 1 && authorities >= 10 && state.lifetime_clues >= 500_000.0,
        _ => false,
    }
}

pub fn beat_by_id(id: u32) -> Option<&'static StoryBeat> {
    BEATS.iter().find(|b| b.id == id)
}

pub fn latest_unlocked(state: &GameState) -> Option<&'static StoryBeat> {
    state
        .story_unlocked
        .iter()
        .max()
        .and_then(|id| beat_by_id(*id))
}

pub fn unlocked_beats(state: &GameState) -> Vec<&'static StoryBeat> {
    let mut beats: Vec<&'static StoryBeat> = state
        .story_unlocked
        .iter()
        .filter_map(|id| beat_by_id(*id))
        .collect();
    beats.sort_by_key(|b| b.id);
    beats
}

pub fn journey_progress(state: &GameState) -> f64 {
    state.story_unlocked.len() as f64 / BEATS.len() as f64
}

pub fn try_unlock_next(state: &mut GameState) -> Option<&'static StoryBeat> {
    for beat in BEATS {
        if !state.story_unlocked.contains(&beat.id) && trigger_met(state, beat.id) {
            state.story_unlocked.push(beat.id);
            if state.achievement_notification.is_none() {
                state.story_notification = Some(beat.id);
                state.story_notification_timer = 0.0;
            }
            return Some(beat);
        }
    }
    None
}

pub fn backfill_unlocks(state: &mut GameState) {
    for beat in BEATS {
        if !state.story_unlocked.contains(&beat.id) && trigger_met(state, beat.id) {
            state.story_unlocked.push(beat.id);
        }
    }
    state.story_unlocked.sort_unstable();
    state.story_unlocked.dedup();
}