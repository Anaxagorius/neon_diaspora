use eframe::egui::{self, Color32, Rect, RichText, Stroke, Vec2};

use crate::avatars;
use crate::data::{authorities, buddies, mentors};
use crate::game::achievements;
use crate::game::milestones;
use crate::game::clicker::{self, CLICK_UPGRADES};
use crate::game::crafting::RECIPES;
use crate::game::state::{GameState, MsgKind};
use crate::game::storyline;
use crate::neon_text::{self, draw_neon_text};
use crate::theme;

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Search,
    Journey,
    Achievements,
    Buddies,
    Mentors,
    Authorities,
    Upgrades,
    Crafting,
    Rebirth,
}

pub struct NeonDiasporaApp {
    state: GameState,
    tab: Tab,
    auto_save_timer: f64,
}

impl NeonDiasporaApp {
    pub fn new() -> Self {
        Self {
            state: GameState::load(),
            tab: Tab::Search,
            auto_save_timer: 0.0,
        }
    }

    fn setup_theme(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.panel_fill = theme::BG_DARK;
        style.visuals.window_fill = theme::BG_PANEL;
        style.visuals.extreme_bg_color = theme::BG_DARK;
        style.visuals.widgets.noninteractive.bg_fill = theme::BG_PANEL_ALT;
        style.visuals.widgets.inactive.bg_fill = theme::BG_PANEL;
        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(25, 40, 60);
        style.visuals.widgets.active.bg_fill = Color32::from_rgb(0, 80, 100);
        style.visuals.selection.bg_fill = Color32::from_rgba_premultiplied(0, 200, 255, 60);
        style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, theme::NEON_DIM);
        style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.5, theme::TEXT_PRIMARY);
        style.visuals.override_text_color = Some(theme::TEXT_PRIMARY);
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::proportional(14.0),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::proportional(14.0),
        );
        ctx.set_style(style);
    }
}

impl eframe::App for NeonDiasporaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Self::setup_theme(ctx);

        let dt = ctx.input(|i| i.unstable_dt) as f64;
        self.state.tick(dt);
        self.auto_save_timer += dt;
        if self.auto_save_timer > 30.0 {
            self.state.save();
            self.auto_save_timer = 0.0;
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(16));

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            self.render_header(ui);
        });

        if let Some(ach_id) = self.state.achievement_notification {
            egui::TopBottomPanel::bottom("ach_notify")
                .min_height(64.0)
                .show(ctx, |ui| {
                    if let Some(ach) = achievements::achievement_by_id(ach_id) {
                        self.render_achievement_notification(ui, ach);
                    }
                });
        } else if let Some(beat_id) = self.state.story_notification {
            egui::TopBottomPanel::bottom("story_notify")
                .min_height(80.0)
                .show(ctx, |ui| {
                    if let Some(beat) = storyline::beat_by_id(beat_id) {
                        self.render_story_notification(ui, beat);
                    }
                });
        } else if let Some(msg) = self.state.current_message.clone() {
            egui::TopBottomPanel::bottom("encouragement")
                .min_height(52.0)
                .show(ctx, |ui| {
                    self.render_encouragement(ui, &msg);
                });
        }

        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(160.0)
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab {
                Tab::Search => self.render_search(ui),
                Tab::Journey => self.render_journey(ui),
                Tab::Achievements => self.render_achievements(ui),
                Tab::Buddies => self.render_entity_shop(ui, ShopKind::Buddy),
                Tab::Mentors => self.render_entity_shop(ui, ShopKind::Mentor),
                Tab::Authorities => self.render_entity_shop(ui, ShopKind::Authority),
                Tab::Upgrades => self.render_upgrades(ui),
                Tab::Crafting => self.render_crafting(ui),
                Tab::Rebirth => self.render_rebirth(ui),
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.state.save();
    }
}

impl NeonDiasporaApp {
    fn render_header(&self, ui: &mut egui::Ui) {
        let header_rect = ui.max_rect();
        ui.painter().rect_stroke(
            header_rect,
            0.0,
            Stroke::new(1.0, neon_text::glow_color(theme::NEON_CYAN, 0.3)),
            egui::StrokeKind::Outside,
        );

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            neon_text::neon_pulse_heading(ui, "NEON DIASPORA", theme::NEON_CYAN, 24.0);
            ui.separator();
            neon_text::neon_label_glow(
                ui,
                "Search the sprawl. Find your parents.",
                theme::NEON_BLUE,
                14.0,
                2.0,
            );
        });
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            stat_label(ui, "Clues", self.state.clues, theme::NEON_GREEN);
            ui.separator();
            stat_label(ui, "Per Sec", self.state.total_cps(), theme::NEON_BLUE);
            ui.separator();
            stat_label(ui, "Per Click", self.state.click_value(), theme::NEON_CYAN);
            ui.separator();
            stat_label(ui, "Rebirth ◆", self.state.rebirth_tokens, theme::NEON_PURPLE);
            ui.separator();
            stat_label(ui, "Prestige ★", self.state.prestige_tokens, theme::TEXT_WARN);
        });
        ui.add_space(4.0);
    }

    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        neon_text::neon_heading(ui, "NAVIGATE", theme::NEON_CYAN, 12.0);
        ui.add_space(4.0);

        tab_button(ui, &mut self.tab, Tab::Search, "◉ SEARCH");
        tab_button(ui, &mut self.tab, Tab::Journey, "◈ JOURNEY");
        tab_button(ui, &mut self.tab, Tab::Achievements, "★ ACHIEV.");
        tab_button(ui, &mut self.tab, Tab::Buddies, "◎ BUDDIES");
        tab_button(ui, &mut self.tab, Tab::Mentors, "◇ MENTORS");
        tab_button(ui, &mut self.tab, Tab::Authorities, "◆ AUTHORITIES");
        tab_button(ui, &mut self.tab, Tab::Upgrades, "↑ UPGRADES");
        tab_button(ui, &mut self.tab, Tab::Crafting, "⚙ CRAFTING");
        tab_button(ui, &mut self.tab, Tab::Rebirth, "∞ REBIRTH");

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        neon_text::neon_heading(ui, "RESOURCES", theme::NEON_GREEN, 12.0);
        ui.add_space(4.0);
        resource_line(ui, "Scrap", self.state.scrap);
        resource_line(ui, "Data Chips", self.state.data_chips);
        resource_line(ui, "Neural Fil.", self.state.neural_filament);

        ui.add_space(16.0);
        ui.separator();
        ui.add_space(8.0);
        neon_text::neon_dim(ui, format!("Clicks: {}", self.state.total_clicks), 12.0);
        neon_text::neon_dim(ui, format!("Rebirths: {}", self.state.total_rebirths), 12.0);
        neon_text::neon_dim(ui, format!("Prestiges: {}", self.state.total_prestiges), 12.0);
        neon_text::neon_dim(
            ui,
            format!(
                "Achievements: {}/{}",
                self.state.achievements_unlocked.len(),
                achievements::ACHIEVEMENTS.len()
            ),
            12.0,
        );
    }

    fn render_search(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);

            neon_text::neon_body(ui, "The rain never stops in Sector 7.", theme::NEON_BLUE, 15.0);
            neon_text::neon_body(
                ui,
                "A young man with Down syndrome searches the neon wilderness",
                theme::TEXT_DIM,
                14.0,
            );
            neon_text::neon_body(
                ui,
                "for the parents who vanished the night of the eclipse.",
                theme::TEXT_DIM,
                14.0,
            );

            ui.add_space(30.0);

            let available = ui.available_size();
            let size = available.x.min(available.y * 0.55).min(280.0);
            let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click());

            if response.clicked() {
                self.state.click();
            }

            let painter = ui.painter();
            let center = rect.center();
            let pulse = self.state.click_pulse;
            let anim = self.state.click_anim;

            let outer_r = size * 0.48;
            let glow_r = outer_r + 8.0 + pulse * 12.0;
            painter.circle_filled(center, glow_r, theme::CLICKER_GLOW);
            painter.circle_stroke(center, outer_r + 4.0, Stroke::new(2.0, theme::CLICKER_RING));
            painter.circle_filled(center, outer_r, theme::BG_PANEL_ALT);
            painter.circle_stroke(center, outer_r, Stroke::new(3.0, theme::NEON_GREEN));

            let inner_r = outer_r * 0.65;
            let inner_color = Color32::from_rgb(
                (0.0 + anim * 60.0) as u8,
                (180.0 + anim * 75.0) as u8,
                (120.0 + anim * 40.0) as u8,
            );
            painter.circle_filled(center, inner_r, inner_color);
            painter.circle_stroke(center, inner_r, Stroke::new(2.0, theme::NEON_CYAN));

            draw_neon_text(
                painter,
                center + Vec2::new(0.0, -12.0),
                egui::Align2::CENTER_CENTER,
                "SEARCH",
                egui::FontId::proportional(20.0),
                theme::NEON_CYAN,
                3.5 + pulse * 2.0,
            );
            draw_neon_text(
                painter,
                center + Vec2::new(0.0, 12.0),
                egui::Align2::CENTER_CENTER,
                "for Clues",
                egui::FontId::proportional(14.0),
                theme::NEON_GREEN,
                2.5,
            );

            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            ui.add_space(20.0);
            neon_text::neon_label_glow(
                ui,
                "Click the node to gather Clues",
                theme::NEON_GREEN,
                13.0,
                2.5,
            );

            ui.add_space(16.0);
            if let Some(beat) = storyline::latest_unlocked(&self.state) {
                ui.group(|ui| {
                    neon_text::neon_label_glow(ui, "◈ CURRENT CHAPTER", theme::NEON_PINK, 12.0, 2.5);
                    neon_text::neon_label(ui, beat.title, theme::NEON_CYAN, 14.0);
                    let excerpt: String = beat.text.chars().take(200).collect();
                    let excerpt = if beat.text.len() > 200 {
                        format!("{excerpt}…")
                    } else {
                        excerpt
                    };
                    neon_text::neon_body(ui, excerpt, theme::TEXT_DIM, 13.0);
                });
            }

            ui.add_space(16.0);
            ui.horizontal(|ui| {
                let w = 200.0;
                progress_bar(ui, "Rebirth Progress", self.state.rebirth_progress(), w, theme::NEON_PURPLE);
                ui.add_space(16.0);
                progress_bar(ui, "Prestige Progress", self.state.prestige_progress(), w, theme::TEXT_WARN);
            });
        });
    }

    fn render_achievement_notification(&self, ui: &mut egui::Ui, ach: &achievements::Achievement) {
        let bar_rect = ui.max_rect();
        ui.painter().rect_filled(bar_rect, 0.0, neon_text::glow_color(theme::TEXT_WARN, 0.12));
        ui.painter().rect_stroke(
            bar_rect,
            0.0,
            Stroke::new(1.5, neon_text::glow_color(theme::TEXT_WARN, 0.7)),
            egui::StrokeKind::Outside,
        );
        ui.horizontal_centered(|ui| {
            neon_text::neon_label(ui, format!("{} ACHIEVEMENT", ach.icon), theme::TEXT_WARN, 14.0);
            neon_text::neon_label(ui, ach.name, theme::NEON_CYAN, 15.0);
            neon_text::neon_body(ui, ach.description, theme::TEXT_DIM, 13.0);
        });
    }

    fn render_achievements(&self, ui: &mut egui::Ui) {
        neon_text::neon_heading(ui, "ACHIEVEMENTS", theme::TEXT_WARN, 20.0);
        neon_text::neon_body(
            ui,
            "Standard milestones for your search — clicks, clues, allies, rebirth, and story progress.",
            theme::TEXT_DIM,
            13.0,
        );
        ui.add_space(6.0);
        progress_bar(
            ui,
            "Unlocked",
            achievements::progress(&self.state),
            400.0,
            theme::TEXT_WARN,
        );
        ui.add_space(12.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for ach in achievements::ACHIEVEMENTS {
                let unlocked = achievements::is_unlocked(&self.state, ach.id);
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        neon_text::neon_label(
                            ui,
                            ach.icon,
                            if unlocked { theme::TEXT_WARN } else { theme::NEON_DIM },
                            18.0,
                        );
                        ui.vertical(|ui| {
                            neon_text::neon_label(
                                ui,
                                ach.name,
                                if unlocked { theme::NEON_GREEN } else { theme::TEXT_PRIMARY },
                                15.0,
                            );
                            neon_text::neon_body(ui, ach.description, theme::TEXT_DIM, 12.0);
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if unlocked {
                                neon_text::neon_label(ui, "✓ Unlocked", theme::NEON_GREEN, 12.0);
                            } else {
                                neon_text::neon_dim(ui, "Locked", 12.0);
                            }
                        });
                    });
                });
                ui.add_space(4.0);
            }
        });
    }

    fn render_story_notification(&self, ui: &mut egui::Ui, beat: &storyline::StoryBeat) {
        let bar_rect = ui.max_rect();
        ui.painter().rect_filled(bar_rect, 0.0, neon_text::glow_color(theme::NEON_PINK, 0.12));
        ui.painter().rect_stroke(
            bar_rect,
            0.0,
            Stroke::new(1.5, neon_text::glow_color(theme::NEON_PINK, 0.7)),
            egui::StrokeKind::Outside,
        );
        ui.vertical_centered(|ui| {
            neon_text::neon_label_glow(ui, "◈ NEW CHAPTER UNLOCKED", theme::NEON_PINK, 13.0, 3.0);
            neon_text::neon_label(ui, beat.title, theme::NEON_CYAN, 15.0);
            let preview: String = beat.text.chars().take(140).collect();
            let preview = if beat.text.len() > 140 {
                format!("{preview}…")
            } else {
                preview
            };
            neon_text::neon_body(ui, preview, theme::TEXT_DIM, 13.0);
        });
    }

    fn render_journey(&self, ui: &mut egui::Ui) {
        neon_text::neon_heading(ui, "THE SEARCH — HIS STORY", theme::NEON_PINK, 20.0);
        neon_text::neon_body(
            ui,
            "The journey of a young man with Down syndrome, searching the neon sprawl for parents lost the night of the eclipse.",
            theme::TEXT_DIM,
            13.0,
        );
        ui.add_space(6.0);
        progress_bar(
            ui,
            "Journey Progress",
            storyline::journey_progress(&self.state),
            400.0,
            theme::NEON_PINK,
        );
        ui.add_space(12.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            let beats = storyline::unlocked_beats(&self.state);
            let latest_id = beats.last().map(|b| b.id);

            for beat in beats {
                let is_latest = latest_id == Some(beat.id);
                ui.group(|ui| {
                    if is_latest {
                        let r = ui.max_rect();
                        ui.painter().rect_stroke(
                            r.expand(2.0),
                            4.0,
                            Stroke::new(1.5, neon_text::glow_color(theme::NEON_CYAN, 0.7)),
                            egui::StrokeKind::Outside,
                        );
                    }
                    neon_text::neon_label_glow(
                        ui,
                        beat.chapter,
                        theme::NEON_PURPLE,
                        12.0,
                        if is_latest { 3.0 } else { 1.5 },
                    );
                    neon_text::neon_heading(
                        ui,
                        beat.title,
                        if is_latest { theme::NEON_CYAN } else { theme::TEXT_PRIMARY },
                        16.0,
                    );
                    neon_text::neon_body(
                        ui,
                        beat.text,
                        if is_latest { theme::NEON_BLUE } else { theme::TEXT_DIM },
                        13.0,
                    );
                    if is_latest {
                        neon_text::neon_label(ui, "— current chapter —", theme::NEON_GREEN, 11.0);
                    }
                });
                ui.add_space(8.0);
            }

            let locked = storyline::BEATS.len() - self.state.story_unlocked.len();
            if locked > 0 {
                ui.add_space(8.0);
                neon_text::neon_dim(
                    ui,
                    format!("{locked} chapters remain hidden. Keep searching."),
                    12.0,
                );
            }
        });
    }

    fn render_encouragement(&self, ui: &mut egui::Ui, msg: &crate::game::state::EncouragementMsg) {
        let color = match msg.kind {
            MsgKind::Buddy => theme::NEON_GREEN,
            MsgKind::Mentor => theme::NEON_PURPLE,
            MsgKind::Authority => theme::TEXT_WARN,
        };
        let bar_rect = ui.max_rect();
        ui.painter().rect_filled(
            bar_rect,
            0.0,
            neon_text::glow_color(color, 0.08),
        );
        ui.painter().rect_stroke(
            bar_rect,
            0.0,
            Stroke::new(1.0, neon_text::glow_color(color, 0.5)),
            egui::StrokeKind::Outside,
        );
        ui.horizontal_centered(|ui| {
            if msg.kind == MsgKind::Buddy {
                if let Some(i) = buddies::BUDDIES.iter().position(|b| b.name == msg.speaker) {
                    let owned = self.state.buddy_owned.get(i).copied().unwrap_or(1);
                    avatars::buddy_avatar(ui, i, &msg.speaker, owned.max(1));
                    ui.add_space(6.0);
                }
            }
            neon_text::neon_label_glow(ui, &msg.speaker, color, 15.0, 3.5);
            neon_text::neon_dim(ui, "—", 14.0);
            neon_text::neon_body(ui, &msg.text, theme::TEXT_PRIMARY, 14.0);
        });
    }

    fn render_upgrades(&mut self, ui: &mut egui::Ui) {
        neon_text::neon_heading(ui, "CLICKER ADVANCEMENT", theme::NEON_CYAN, 18.0);
        neon_text::neon_body(
            ui,
            format!(
                "{} upgrade types — {} total owned. Buy repeatedly to boost click power.",
                CLICK_UPGRADES,
                self.state.total_click_upgrades_owned(),
            ),
            theme::TEXT_DIM,
            13.0,
        );
        neon_text::neon_body(
            ui,
            format!(
                "Milestone bonus: every {} purchases of the same upgrade adds +1× base click power (forever).",
                milestones::MILESTONE_INTERVAL
            ),
            theme::NEON_GREEN,
            12.0,
        );
        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for i in 0..CLICK_UPGRADES {
                let def = &clicker::UPGRADE_DEFS[i];
                let owned = self.state.click_upgrade_owned.get(i).copied().unwrap_or(0);
                let cost = clicker::upgrade_cost(i, owned);
                let can_buy = self.state.clues >= cost;
                upgrade_row(ui, i, def, owned, cost, can_buy, || self.state.buy_click_upgrade(i));
            }
        });
    }

    fn render_crafting(&mut self, ui: &mut egui::Ui) {
        neon_text::neon_heading(ui, "CRAFTING BAY", theme::NEON_CYAN, 18.0);
        neon_text::neon_body(
            ui,
            "Gather scrap, chips, and filament passively. Craft permanent multipliers.",
            theme::TEXT_DIM,
            13.0,
        );
        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, recipe) in RECIPES.iter().enumerate() {
                let owned = self.state.crafts_owned[i];
                let can_craft = !owned
                    && self.state.scrap >= recipe.scrap_cost
                    && self.state.data_chips >= recipe.chip_cost
                    && self.state.neural_filament >= recipe.filament_cost
                    && self.state.clues >= recipe.clue_cost;

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            neon_text::neon_label(
                                ui,
                                recipe.name,
                                if owned { theme::NEON_GREEN } else { theme::TEXT_PRIMARY },
                                14.0,
                            );
                            neon_text::neon_body(ui, recipe.description, theme::TEXT_DIM, 12.0);
                            neon_text::neon_dim(
                                ui,
                                format!(
                                    "Scrap {} | Chips {} | Fil. {} | Clues {}",
                                    theme::format_number(recipe.scrap_cost),
                                    theme::format_number(recipe.chip_cost),
                                    theme::format_number(recipe.filament_cost),
                                    theme::format_number(recipe.clue_cost),
                                ),
                                11.0,
                            );
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if owned {
                                neon_text::neon_label(ui, "Crafted", theme::NEON_GREEN, 13.0);
                            } else if ui
                                .add_enabled(can_craft, egui::Button::new("Craft"))
                                .clicked()
                            {
                                self.state.craft_item(i);
                            }
                        });
                    });
                });
            }
        });
    }

    fn render_rebirth(&mut self, ui: &mut egui::Ui) {
        neon_text::neon_heading(ui, "REBIRTH & PRESTIGE", theme::NEON_CYAN, 18.0);
        ui.add_space(12.0);

        ui.group(|ui| {
            neon_text::neon_heading(ui, "REBIRTH", theme::NEON_PURPLE, 16.0);
            neon_text::neon_body(
                ui,
                "Sacrifice clues and buddies to gain Rebirth Tokens. Mentors persist.",
                theme::TEXT_DIM,
                13.0,
            );
            neon_text::neon_body(
                ui,
                format!(
                    "Requires {} lifetime Clues (current: {})",
                    theme::format_number(500_000.0),
                    theme::format_number(self.state.rebirth_cycle_clues),
                ),
                theme::TEXT_DIM,
                13.0,
            );
            progress_bar(ui, "", self.state.rebirth_progress(), 300.0, theme::NEON_PURPLE);
            neon_text::neon_label_glow(
                ui,
                format!(
                    "Would earn: {} Rebirth Tokens",
                    theme::format_number(self.state.rebirth_preview_tokens()),
                ),
                theme::NEON_PURPLE,
                14.0,
                3.0,
            );
            if ui
                .add_enabled(self.state.rebirth_available(), egui::Button::new("REBIRTH"))
                .clicked()
            {
                self.state.perform_rebirth();
            }
        });

        ui.add_space(16.0);

        ui.group(|ui| {
            neon_text::neon_heading(ui, "PRESTIGE", theme::TEXT_WARN, 16.0);
            neon_text::neon_body(
                ui,
                "Sacrifice rebirth progress for Prestige Tokens. Authorities await.",
                theme::TEXT_DIM,
                13.0,
            );
            neon_text::neon_body(
                ui,
                format!(
                    "Requires {} Rebirth Tokens & 3+ rebirths (current tokens: {}, rebirths: {})",
                    theme::format_number(25.0),
                    theme::format_number(self.state.rebirth_tokens),
                    self.state.total_rebirths,
                ),
                theme::TEXT_DIM,
                13.0,
            );
            progress_bar(ui, "", self.state.prestige_progress(), 300.0, theme::TEXT_WARN);
            neon_text::neon_label_glow(
                ui,
                format!(
                    "Would earn: {} Prestige Tokens",
                    theme::format_number(self.state.prestige_preview_tokens()),
                ),
                theme::TEXT_WARN,
                14.0,
                3.0,
            );
            if ui
                .add_enabled(self.state.prestige_available(), egui::Button::new("PRESTIGE"))
                .clicked()
            {
                self.state.perform_prestige();
            }
        });
    }

    fn render_entity_shop(&mut self, ui: &mut egui::Ui, kind: ShopKind) {
        let (title, subtitle, currency_color) = match kind {
            ShopKind::Buddy => (
                "CITY DIASPORA — BUDDIES",
                "Hire street allies to gather Clues idly. Paid in Clues.",
                theme::NEON_GREEN,
            ),
            ShopKind::Mentor => (
                "MENTORS",
                "Specialized guides. Purchased with Rebirth Tokens.",
                theme::NEON_PURPLE,
            ),
            ShopKind::Authority => (
                "THE AUTHORITIES",
                "Official power of the sprawl. Purchased with Prestige Tokens.",
                theme::TEXT_WARN,
            ),
        };

        neon_text::neon_heading(ui, title, currency_color, 18.0);
        neon_text::neon_body(ui, subtitle, theme::TEXT_DIM, 13.0);
        if matches!(kind, ShopKind::Buddy) {
            neon_text::neon_body(
                ui,
                format!(
                    "Loyalty bonus: every {} purchases of the same Buddy adds +1× base output per unit (forever).",
                    milestones::MILESTONE_INTERVAL
                ),
                theme::NEON_GREEN,
                12.0,
            );
        }
        ui.add_space(8.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            match kind {
                ShopKind::Buddy => {
                    for (i, def) in buddies::BUDDIES.iter().enumerate() {
                        let owned = self.state.buddy_owned[i];
                        let cost = GameState::entity_cost(def, owned);
                        let can_buy = self.state.clues >= cost;
                        buddy_row(ui, i, def, owned, cost, can_buy, || self.state.buy_buddy(i));
                    }
                }
                ShopKind::Mentor => {
                    for (i, def) in mentors::MENTORS.iter().enumerate() {
                        let owned = self.state.mentor_owned[i];
                        let cost = GameState::entity_cost(def, owned);
                        let can_buy = self.state.rebirth_tokens >= cost;
                        entity_row(ui, def, owned, cost, can_buy, || self.state.buy_mentor(i));
                    }
                }
                ShopKind::Authority => {
                    for (i, def) in authorities::AUTHORITIES.iter().enumerate() {
                        let owned = self.state.authority_owned[i];
                        let cost = GameState::entity_cost(def, owned);
                        let can_buy = self.state.prestige_tokens >= cost;
                        entity_row(ui, def, owned, cost, can_buy, || self.state.buy_authority(i));
                    }
                }
            }
        });
    }
}

enum ShopKind {
    Buddy,
    Mentor,
    Authority,
}

fn stat_label(ui: &mut egui::Ui, label: &str, value: f64, color: Color32) {
    neon_text::neon_dim(ui, format!("{}: ", label), 14.0);
    neon_text::neon_label_glow(ui, theme::format_number(value), color, 16.0, 3.5);
}

fn resource_line(ui: &mut egui::Ui, label: &str, value: f64) {
    ui.horizontal(|ui| {
        neon_text::neon_dim(ui, label, 12.0);
        neon_text::neon_label_glow(ui, theme::format_number(value), theme::NEON_BLUE, 13.0, 2.5);
    });
}

fn tab_button(ui: &mut egui::Ui, current: &mut Tab, tab: Tab, label: &str) {
    let selected = *current == tab;
    let color = if selected { theme::NEON_CYAN } else { theme::NEON_DIM };
    if neon_text::neon_selectable(ui, selected, label, color, 13.0).clicked() {
        *current = tab;
    }
}

fn progress_bar(ui: &mut egui::Ui, label: &str, progress: f64, width: f32, color: Color32) {
    if !label.is_empty() {
        neon_text::neon_dim(ui, label, 12.0);
    }
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 16.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 2.0, theme::BG_PANEL_ALT);
    let fill = Rect::from_min_size(rect.min, Vec2::new(rect.width() * progress as f32, rect.height()));
    painter.rect_filled(fill, 2.0, color);
    painter.rect_filled(fill.expand(2.0), 2.0, neon_text::glow_color(color, 0.25));
    painter.rect_stroke(rect, 2.0, Stroke::new(1.5, neon_text::glow_color(color, 0.6)), egui::StrokeKind::Outside);
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{:.1}%", progress * 100.0),
        egui::FontId::proportional(11.0),
        theme::TEXT_PRIMARY,
    );
}

fn upgrade_row(
    ui: &mut egui::Ui,
    index: usize,
    def: &clicker::ClickUpgradeDef,
    owned: u32,
    cost: f64,
    can_buy: bool,
    mut buy: impl FnMut() -> bool,
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    neon_text::neon_label(
                        ui,
                        format!("{}. {}", index + 1, def.name),
                        theme::NEON_CYAN,
                        15.0,
                    );
                    if owned > 0 {
                        neon_text::neon_label(ui, format!("×{}", owned), theme::NEON_GREEN, 14.0);
                        neon_text::neon_label(
                            ui,
                            format!("Tier {}", milestones::milestone_tier(owned)),
                            theme::TEXT_WARN,
                            11.0,
                        );
                    }
                });
                neon_text::neon_body(ui, def.description, theme::TEXT_DIM, 12.0);
                let base = clicker::upgrade_base_power(index);
                let rate_line = if owned == 0 {
                    format!(
                        "Base +{} click — reaches +{} at 25 owned",
                        theme::format_number(base),
                        theme::format_number(base * 2.0),
                    )
                } else {
                    format!(
                        "+{} click each — next +{} at {}",
                        theme::format_number(GameState::upgrade_per_unit_output(index, owned)),
                        theme::format_number(base),
                        milestones::next_milestone_at(owned),
                    )
                };
                neon_text::neon_dim(ui, rate_line, 11.0);
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn = egui::Button::new(
                    RichText::new(theme::format_number(cost))
                        .color(if can_buy { theme::NEON_GREEN } else { theme::TEXT_DIM }),
                );
                if ui.add_enabled(can_buy, btn).clicked() {
                    buy();
                }
            });
        });
    });
}

fn buddy_row(
    ui: &mut egui::Ui,
    index: usize,
    def: &crate::data::EntityDef,
    owned: u32,
    cost: f64,
    can_buy: bool,
    mut buy: impl FnMut() -> bool,
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            avatars::buddy_avatar(ui, index, def.name, owned);
            ui.add_space(4.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    neon_text::neon_label(ui, def.name, theme::NEON_CYAN, 15.0);
                    if owned > 0 {
                        neon_text::neon_label(ui, format!("×{}", owned), theme::NEON_GREEN, 14.0);
                        neon_text::neon_label(
                            ui,
                            format!("Tier {}", milestones::milestone_tier(owned)),
                            theme::TEXT_WARN,
                            11.0,
                        );
                    }
                });
                neon_text::neon_body(ui, def.flavor, theme::TEXT_DIM, 12.0);
                let rate_line = if owned == 0 {
                    format!(
                        "Base {}/s — reaches {}/s at 25 owned",
                        theme::format_number(def.base_power),
                        theme::format_number(def.base_power * 2.0),
                    )
                } else {
                    format!(
                        "{}/s each — next +{} at {}",
                        theme::format_number(GameState::buddy_per_unit_output(def.base_power, owned)),
                        theme::format_number(def.base_power),
                        milestones::next_milestone_at(owned),
                    )
                };
                neon_text::neon_dim(ui, rate_line, 11.0);
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn = egui::Button::new(
                    RichText::new(theme::format_number(cost))
                        .color(if can_buy { theme::NEON_GREEN } else { theme::TEXT_DIM }),
                );
                if ui.add_enabled(can_buy, btn).clicked() {
                    buy();
                }
            });
        });
    });
}

fn entity_row(
    ui: &mut egui::Ui,
    def: &crate::data::EntityDef,
    owned: u32,
    cost: f64,
    can_buy: bool,
    mut buy: impl FnMut() -> bool,
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    neon_text::neon_label(ui, def.name, theme::NEON_CYAN, 15.0);
                    if owned > 0 {
                        neon_text::neon_label(ui, format!("×{}", owned), theme::NEON_GREEN, 14.0);
                    }
                });
                neon_text::neon_body(ui, def.flavor, theme::TEXT_DIM, 12.0);
                neon_text::neon_dim(
                    ui,
                    format!("+{}/s each", theme::format_number(def.base_power)),
                    11.0,
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn = egui::Button::new(
                    RichText::new(theme::format_number(cost))
                        .color(if can_buy { theme::NEON_GREEN } else { theme::TEXT_DIM }),
                );
                if ui.add_enabled(can_buy, btn).clicked() {
                    buy();
                }
            });
        });
    });
}