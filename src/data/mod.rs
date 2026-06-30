pub mod authorities;
pub mod buddies;
pub mod mentors;

pub struct EntityDef {
    pub name: &'static str,
    pub flavor: &'static str,
    pub base_cost: f64,
    pub base_power: f64,
    pub messages: &'static [&'static str],
}