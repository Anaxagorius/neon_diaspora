use eframe::egui::{self, Align2, Color32, FontId, LayerId, Order, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

use crate::neon_text;
use crate::theme;

const AVATAR_SIZE: f32 = 52.0;

// ---------------------------------------------------------------------------
// Buddy sprite images — embedded at compile time
// ---------------------------------------------------------------------------

/// Raw JPEG bytes for each buddy that has an image, indexed by buddy slot.
/// `None` means no image; the procedural avatar will be drawn instead.
static BUDDY_IMAGE_BYTES: &[Option<&[u8]>] = &[
    Some(include_bytes!("../assets/buddies/neon_ghost.jpg")),        // 0  Neon Ghost
    Some(include_bytes!("../assets/buddies/rain_slick_ronin.jpg")),  // 1  Rain-Slick Ronin
    Some(include_bytes!("../assets/buddies/packet_rat.jpg")),        // 2  Packet Rat
    Some(include_bytes!("../assets/buddies/chrome_nomad.jpg")),      // 3  Chrome Nomad
    Some(include_bytes!("../assets/buddies/static_whisper.jpg")),    // 4  Static Whisper
    Some(include_bytes!("../assets/buddies/grid_rat.jpg")),          // 5  Grid Rat
    Some(include_bytes!("../assets/buddies/holo_scribe.jpg")),       // 6  Holo Scribe
    Some(include_bytes!("../assets/buddies/synth_alley_cat.jpg")),   // 7  Synth Alley Cat
    Some(include_bytes!("../assets/buddies/void_courier.jpg")),      // 8  Void Courier
    Some(include_bytes!("../assets/buddies/neuro_drifter.jpg")),     // 9  Neuro Drifter
    Some(include_bytes!("../assets/buddies/rust_prophet.jpg")),      // 10 Rust Prophet
    Some(include_bytes!("../assets/buddies/pulse_jack.jpg")),        // 11 Pulse Jack
    Some(include_bytes!("../assets/buddies/cipher_bloom.jpg")),      // 12 Cipher Bloom
    None,                                                             // 13 Drip Saint
    Some(include_bytes!("../assets/buddies/wire_widow.jpg")),        // 14 Wire Widow
    Some(include_bytes!("../assets/buddies/glitch_monk.jpg")),       // 15 Glitch Monk
    Some(include_bytes!("../assets/buddies/blackout_bard.jpg")),     // 16 Blackout Bard
    None,                                                             // 17 Shard Diver
    None,                                                             // 18 Echo Broker
    Some(include_bytes!("../assets/buddies/neon_mender.jpg")),       // 19 Neon Mender
    Some(include_bytes!("../assets/buddies/fog_runner.jpg")),        // 20 Fog Runner
    Some(include_bytes!("../assets/buddies/data_vagrant.jpg")),      // 21 Data Vagrant
    Some(include_bytes!("../assets/buddies/circuit_poet.jpg")),      // 22 Circuit Poet
    Some(include_bytes!("../assets/buddies/voltage_nun.jpg")),       // 23 Voltage Nun
    Some(include_bytes!("../assets/buddies/proxy_child.jpg")),       // 24 Proxy Child
    None,                                                             // 25 Signal Thief
    Some(include_bytes!("../assets/buddies/carbon_angel.jpg")),      // 26 Carbon Angel
    Some(include_bytes!("../assets/buddies/null_priest.jpg")),       // 27 Null Priest
    None,                                                             // 28 Razor Cartographer
    Some(include_bytes!("../assets/buddies/bit_hustler.jpg")),       // 29 Bit Hustler
];

/// Cache of loaded `TextureHandle`s — one per buddy slot.
/// Slots beyond `BUDDY_IMAGE_BYTES` length, or slots marked `None`, stay `None`.
pub struct BuddySprites {
    textures: Vec<Option<egui::TextureHandle>>,
}

impl BuddySprites {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
        }
    }

    /// Decode every JPEG and upload it to the GPU the first time this is called.
    pub fn ensure_loaded(&mut self, ctx: &egui::Context) {
        if !self.textures.is_empty() {
            return;
        }
        self.textures = BUDDY_IMAGE_BYTES
            .iter()
            .enumerate()
            .map(|(i, slot)| {
                slot.and_then(|bytes| {
                    let img = image::load_from_memory(bytes)
                        .ok()?
                        .into_rgba8();
                    let size = [img.width() as usize, img.height() as usize];
                    let color_image =
                        egui::ColorImage::from_rgba_unmultiplied(size, &img.into_raw());
                    Some(ctx.load_texture(
                        format!("buddy_sprite_{i}"),
                        color_image,
                        egui::TextureOptions::LINEAR,
                    ))
                })
            })
            .collect();
    }

    /// Return the texture for the given buddy slot, if one was loaded.
    pub fn get(&self, index: usize) -> Option<&egui::TextureHandle> {
        self.textures.get(index)?.as_ref()
    }
}

// ---------------------------------------------------------------------------
// Procedural avatar styles (fallback when no sprite image is available)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
enum AvatarStyle {
    Ghost,
    Ronin,
    Rat,
    Wanderer,
    Signal,
    Tunnel,
    Scribe,
    Feral,
    Courier,
    Dreamer,
}

pub fn initials(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();
    match words.len() {
        0 => "?".to_string(),
        1 => words[0].chars().take(2).collect::<String>().to_uppercase(),
        _ => {
            let a = words[0].chars().next().unwrap_or('?');
            let b = words[1].chars().next().unwrap_or('?');
            format!("{a}{b}").to_uppercase()
        }
    }
}

fn palette(index: usize) -> (Color32, Color32, Color32) {
    let hue = (index as f32 * 47.0) % 360.0;
    let primary = hsl(hue, 0.75, 0.45);
    let accent = hsl((hue + 40.0) % 360.0, 0.9, 0.6);
    let ring = hsl((hue + 180.0) % 360.0, 0.85, 0.55);
    (primary, accent, ring)
}

fn hsl(h: f32, s: f32, l: f32) -> Color32 {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match h as i32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color32::from_rgb(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn style_for(index: usize) -> AvatarStyle {
    match index % 10 {
        0 => AvatarStyle::Ghost,
        1 => AvatarStyle::Ronin,
        2 => AvatarStyle::Rat,
        3 => AvatarStyle::Wanderer,
        4 => AvatarStyle::Signal,
        5 => AvatarStyle::Tunnel,
        6 => AvatarStyle::Scribe,
        7 => AvatarStyle::Feral,
        8 => AvatarStyle::Courier,
        _ => AvatarStyle::Dreamer,
    }
}

// ---------------------------------------------------------------------------
// Public drawing helpers
// ---------------------------------------------------------------------------

const AVATAR_HOVER_SCALE: f32 = 1.75;

pub fn buddy_avatar(
    ui: &mut Ui,
    index: usize,
    name: &str,
    owned: u32,
    sprite: Option<&egui::TextureHandle>,
) -> Response {
    let size = Vec2::splat(AVATAR_SIZE);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    draw_buddy_avatar(ui.painter(), rect, index, name, owned, sprite);

    if response.hovered() {
        let expanded = AVATAR_SIZE * AVATAR_HOVER_SCALE;
        let expand_rect = Rect::from_center_size(rect.center(), Vec2::splat(expanded));
        let layer = LayerId::new(Order::Tooltip, egui::Id::new("avatar_hover").with(index));
        let hover_painter = ui.ctx().layer_painter(layer);
        draw_buddy_avatar(&hover_painter, expand_rect, index, name, owned, sprite);
    }

    response
}

pub fn draw_buddy_avatar(
    painter: &egui::Painter,
    rect: Rect,
    index: usize,
    name: &str,
    owned: u32,
    sprite: Option<&egui::TextureHandle>,
) {
    let center = rect.center();
    let r = rect.width().min(rect.height()) * 0.42;
    let (_, _, ring) = palette(index);

    if owned > 0 {
        painter.circle_filled(
            center,
            r + 6.0,
            neon_text::glow_color(theme::NEON_GREEN, 0.2),
        );
    }

    painter.circle_filled(center, r + 3.0, theme::BG_DARK);
    painter.circle_stroke(center, r + 3.0, Stroke::new(2.0, ring));

    if let Some(texture) = sprite {
        // Render photo sprite clipped to a circle using a triangle-fan mesh.
        add_circular_image(painter, center, r, texture);
    } else {
        // Procedural fallback
        let (primary, accent, _) = palette(index);
        let style = style_for(index);
        let init = initials(name);

        painter.circle_filled(center, r, Color32::from_rgb(12, 18, 28));
        painter.circle_filled(center, r * 0.92, primary);

        draw_silhouette(painter, center, r, style, accent, index);

        painter.text(
            center + Vec2::new(0.0, r * 0.55),
            Align2::CENTER_CENTER,
            init,
            FontId::proportional(11.0),
            theme::TEXT_PRIMARY,
        );
    }

    // Re-stroke ring on top so it frames the image cleanly.
    painter.circle_stroke(center, r + 3.0, Stroke::new(2.0, ring));

    if owned > 0 {
        let badge = center + Vec2::new(r * 0.65, -r * 0.65);
        painter.circle_filled(badge, 7.0, theme::NEON_GREEN);
        painter.circle_stroke(badge, 7.0, Stroke::new(1.0, theme::BG_DARK));
        painter.text(
            badge,
            Align2::CENTER_CENTER,
            "✓",
            FontId::proportional(9.0),
            theme::BG_DARK,
        );
    }
}

/// Render a texture clipped to a filled circle using a triangle-fan mesh.
///
/// The UV mapping centres the texture on the circle: the top of the
/// image (v = 0) maps to the top of the circle, and the sides are
/// cropped if the source aspect ratio is not 1:1.
fn add_circular_image(
    painter: &egui::Painter,
    center: Pos2,
    r: f32,
    texture: &egui::TextureHandle,
) {
    let segments: u32 = 64;
    let mut mesh = egui::Mesh::with_texture(texture.id());

    // Centre vertex — UV (0.5, 0.5)
    mesh.vertices.push(egui::epaint::Vertex {
        pos: center,
        uv: egui::Pos2::new(0.5, 0.5),
        color: Color32::WHITE,
    });

    for i in 0..segments {
        let angle = i as f32 * std::f32::consts::TAU / segments as f32 - std::f32::consts::FRAC_PI_2;
        let cos = angle.cos();
        let sin = angle.sin();
        mesh.vertices.push(egui::epaint::Vertex {
            pos: Pos2::new(center.x + cos * r, center.y + sin * r),
            uv: egui::Pos2::new(0.5 + cos * 0.5, 0.5 + sin * 0.5),
            color: Color32::WHITE,
        });
    }

    for i in 0..segments {
        mesh.indices.push(0); // centre
        mesh.indices.push(i + 1);
        mesh.indices.push((i + 1) % segments + 1);
    }

    painter.add(egui::Shape::Mesh(mesh.into()));
}

// ---------------------------------------------------------------------------
// Procedural silhouette drawing
// ---------------------------------------------------------------------------

fn draw_silhouette(
    painter: &egui::Painter,
    center: Pos2,
    r: f32,
    style: AvatarStyle,
    accent: Color32,
    index: usize,
) {
    let head_r = r * 0.28;
    let head = center + Vec2::new(0.0, -r * 0.12);
    let body_top = head + Vec2::new(0.0, head_r);
    let body_bottom = center + Vec2::new(0.0, r * 0.35);

    painter.circle_filled(head, head_r, Color32::from_rgba_premultiplied(8, 12, 20, 220));

    match style {
        AvatarStyle::Ghost => {
            painter.line_segment(
                [body_top, body_bottom],
                Stroke::new(2.5, accent),
            );
            painter.circle_stroke(head, head_r + 4.0, Stroke::new(1.5, neon_text::glow_color(accent, 0.5)));
        }
        AvatarStyle::Ronin => {
            painter.line_segment([body_top, body_bottom], Stroke::new(3.0, accent));
            let visor_y = head.y;
            painter.line_segment(
                [
                    Pos2::new(head.x - head_r * 0.9, visor_y),
                    Pos2::new(head.x + head_r * 0.9, visor_y),
                ],
                Stroke::new(2.0, theme::NEON_CYAN),
            );
        }
        AvatarStyle::Rat => {
            painter.line_segment([body_top, body_bottom], Stroke::new(2.5, accent));
            for dx in [-1.0_f32, 1.0] {
                painter.line_segment(
                    [
                        head + Vec2::new(dx * head_r * 0.5, -head_r),
                        head + Vec2::new(dx * head_r * 1.1, -head_r * 1.6),
                    ],
                    Stroke::new(2.0, accent),
                );
            }
        }
        AvatarStyle::Wanderer => {
            painter.line_segment([body_top, body_bottom], Stroke::new(2.5, accent));
            painter.line_segment(
                [
                    head + Vec2::new(-head_r * 1.4, -head_r * 0.2),
                    head + Vec2::new(head_r * 1.4, -head_r * 0.2),
                ],
                Stroke::new(2.0, accent),
            );
        }
        AvatarStyle::Signal => {
            painter.line_segment([body_top, body_bottom], Stroke::new(2.5, accent));
            for i in 0..3 {
                let t = i as f32 * 0.35;
                painter.circle_stroke(
                    head + Vec2::new(head_r * 0.8 + t * 5.0, 0.0),
                    3.0 + t * 3.0,
                    Stroke::new(1.0, neon_text::glow_color(accent, 0.6)),
                );
            }
        }
        AvatarStyle::Tunnel => {
            painter.rect_filled(
                Rect::from_center_size(body_top, Vec2::new(r * 0.35, r * 0.5)),
                2.0,
                Color32::from_rgba_premultiplied(8, 12, 20, 200),
            );
            painter.rect_stroke(
                Rect::from_center_size(body_top, Vec2::new(r * 0.35, r * 0.5)),
                2.0,
                Stroke::new(1.5, accent),
                egui::StrokeKind::Outside,
            );
        }
        AvatarStyle::Scribe => {
            painter.line_segment([body_top, body_bottom], Stroke::new(2.5, accent));
            let pad = Rect::from_center_size(
                head + Vec2::new(r * 0.35, 0.0),
                Vec2::splat(r * 0.35),
            );
            painter.rect_filled(pad, 2.0, theme::BG_PANEL);
            painter.rect_stroke(pad, 2.0, Stroke::new(1.0, theme::NEON_CYAN), egui::StrokeKind::Outside);
        }
        AvatarStyle::Feral => {
            painter.line_segment([body_top, body_bottom], Stroke::new(2.5, accent));
            for dx in [-1.0_f32, 1.0] {
                let tip = head + Vec2::new(dx * head_r * 0.8, -head_r * 1.3);
                painter.add(egui::Shape::convex_polygon(
                    vec![
                        head + Vec2::new(dx * head_r * 0.3, -head_r * 0.5),
                        tip,
                        head + Vec2::new(dx * head_r * 1.0, -head_r * 0.3),
                    ],
                    accent,
                    Stroke::NONE,
                ));
            }
        }
        AvatarStyle::Courier => {
            painter.line_segment([body_top, body_bottom], Stroke::new(2.5, accent));
            let pack = Rect::from_center_size(
                body_top + Vec2::new(r * 0.3, r * 0.12),
                Vec2::new(r * 0.3, r * 0.28),
            );
            painter.rect_filled(pack, 2.0, accent);
        }
        AvatarStyle::Dreamer => {
            painter.line_segment([body_top, body_bottom], Stroke::new(2.5, accent));
            painter.line_segment(
                [
                    head + Vec2::new(-head_r, head_r * 0.2),
                    head + Vec2::new(-head_r * 1.8, head_r * 0.5),
                ],
                Stroke::new(1.5, theme::NEON_PURPLE),
            );
            painter.circle_filled(
                head + Vec2::new(-head_r * 1.8, head_r * 0.5),
                2.5,
                theme::NEON_PURPLE,
            );
        }
    }

    // Cyber scan line across face
    let scan_y = head.y + (index as f32 % 5.0) * 1.5 - 2.0;
    painter.line_segment(
        [
            Pos2::new(head.x - head_r, scan_y),
            Pos2::new(head.x + head_r, scan_y),
        ],
        Stroke::new(1.0, neon_text::glow_color(theme::NEON_CYAN, 0.45)),
    );
}