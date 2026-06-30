use eframe::egui::{self, Align2, Color32, FontId, Pos2, Response, RichText, Ui};

use crate::theme;

pub fn glow_color(color: Color32, alpha: f32) -> Color32 {
    Color32::from_rgba_premultiplied(
        color.r(),
        color.g(),
        color.b(),
        (alpha * 255.0) as u8,
    )
}

pub fn brighten(color: Color32) -> Color32 {
    Color32::from_rgb(
        color.r().saturating_add(30),
        color.g().saturating_add(30),
        color.b().saturating_add(30),
    )
}

/// Single-pass bright text with one soft shadow — safe for short labels only.
pub fn draw_neon_text(
    painter: &egui::Painter,
    pos: Pos2,
    align: Align2,
    text: &str,
    font_id: FontId,
    color: Color32,
    _intensity: f32,
) {
    let shadow = glow_color(color, 0.35);
    painter.text(
        pos + egui::vec2(1.0, 1.0),
        align,
        text,
        font_id.clone(),
        shadow,
    );
    painter.text(pos, align, text, font_id, brighten(color));
}

pub fn neon_label(ui: &mut Ui, text: impl Into<String>, color: Color32, size: f32) -> Response {
    ui.label(
        RichText::new(text.into())
            .size(size)
            .color(brighten(color))
            .strong(),
    )
}

pub fn neon_label_glow(
    ui: &mut Ui,
    text: impl Into<String>,
    color: Color32,
    size: f32,
    _glow: f32,
) -> Response {
    neon_label(ui, text, color, size)
}

pub fn neon_heading(ui: &mut Ui, text: impl Into<String>, color: Color32, size: f32) -> Response {
    ui.label(
        RichText::new(text.into())
            .size(size)
            .color(brighten(color))
            .strong(),
    )
}

pub fn neon_body(ui: &mut Ui, text: impl Into<String>, color: Color32, size: f32) -> Response {
    ui.label(
        RichText::new(text.into())
            .size(size)
            .color(color)
            .line_height(Some(size * 1.45)),
    )
}

pub fn neon_dim(ui: &mut Ui, text: impl Into<String>, size: f32) -> Response {
    neon_body(ui, text, theme::TEXT_DIM, size)
}

pub fn neon_pulse_heading(ui: &mut Ui, text: &str, color: Color32, size: f32) -> Response {
    neon_heading(ui, text, color, size)
}

pub fn neon_selectable(
    ui: &mut Ui,
    selected: bool,
    text: &str,
    color: Color32,
    size: f32,
) -> Response {
    let label_color = if selected {
        brighten(color)
    } else {
        color
    };
    let rich = RichText::new(text)
        .size(size)
        .color(label_color)
        .strong();
    let response = ui.selectable_label(selected, rich);
    if selected {
        let rect = response.rect;
        ui.painter().rect_stroke(
            rect.expand(1.0),
            3.0,
            egui::Stroke::new(1.0, brighten(color)),
            egui::StrokeKind::Outside,
        );
    }
    response
}