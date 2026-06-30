use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

use crate::neon_text;
use crate::theme;

const AVATAR_SIZE: f32 = 52.0;

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

pub fn buddy_avatar(ui: &mut Ui, index: usize, name: &str, owned: u32) -> Response {
    let size = Vec2::splat(AVATAR_SIZE);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    draw_buddy_avatar(ui.painter(), rect, index, name, owned);
    response
}

pub fn draw_buddy_avatar(
    painter: &egui::Painter,
    rect: Rect,
    index: usize,
    name: &str,
    owned: u32,
) {
    let center = rect.center();
    let r = AVATAR_SIZE * 0.42;
    let (primary, accent, ring) = palette(index);
    let style = style_for(index);
    let init = initials(name);

    if owned > 0 {
        painter.circle_filled(
            center,
            r + 6.0,
            neon_text::glow_color(theme::NEON_GREEN, 0.2),
        );
    }

    painter.circle_filled(center, r + 3.0, theme::BG_DARK);
    painter.circle_stroke(center, r + 3.0, Stroke::new(2.0, ring));
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