use egui::Color32;

pub const BG_DARK: Color32 = Color32::from_rgb(8, 12, 18);
pub const BG_PANEL: Color32 = Color32::from_rgb(14, 20, 32);
pub const BG_PANEL_ALT: Color32 = Color32::from_rgb(18, 26, 42);
pub const NEON_GREEN: Color32 = Color32::from_rgb(0, 255, 136);
pub const NEON_BLUE: Color32 = Color32::from_rgb(0, 200, 255);
pub const NEON_CYAN: Color32 = Color32::from_rgb(0, 255, 255);
pub const NEON_PURPLE: Color32 = Color32::from_rgb(180, 80, 255);
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(235, 250, 255);
pub const TEXT_DIM: Color32 = Color32::from_rgb(170, 215, 235);
pub const NEON_DIM: Color32 = Color32::from_rgb(120, 200, 230);
pub const TEXT_WARN: Color32 = Color32::from_rgb(255, 200, 80);
pub const NEON_PINK: Color32 = Color32::from_rgb(255, 60, 180);
pub const GRIT_BORDER: Color32 = Color32::from_rgb(40, 60, 80);
pub const CLICKER_GLOW: Color32 = Color32::from_rgba_premultiplied(0, 255, 136, 40);
pub const CLICKER_CORE: Color32 = Color32::from_rgb(0, 180, 120);
pub const CLICKER_RING: Color32 = Color32::from_rgb(0, 220, 255);

pub fn format_number(n: f64) -> String {
    if n < 0.0 {
        return format!("-{}", format_number(-n));
    }
    if n < 1000.0 {
        if n.fract().abs() < 0.01 {
            format!("{:.0}", n)
        } else if n < 10.0 {
            format!("{:.2}", n)
        } else {
            format!("{:.1}", n)
        }
    } else if n < 1_000_000.0 {
        format!("{:.2}K", n / 1_000.0)
    } else if n < 1_000_000_000.0 {
        format!("{:.2}M", n / 1_000_000.0)
    } else if n < 1_000_000_000_000.0 {
        format!("{:.2}B", n / 1_000_000_000.0)
    } else {
        format!("{:.2}T", n / 1_000_000_000_000.0)
    }
}