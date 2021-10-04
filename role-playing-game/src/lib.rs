// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => match self.level {
                0..=9 => Some(Self {
                    health: 100,
                    level: self.level,
                    mana: None,
                }),
                _ => Some(Self {
                    health: 100,
                    level: self.level,
                    mana: Some(100),
                }),
            },
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) => {
                if let Some(diff) = m.checked_sub(mana_cost) {
                    self.mana = Some(diff);
                    2 * mana_cost
                } else {
                    0
                }
            }
            None => {
                self.health = self.health.checked_sub(mana_cost).unwrap_or(0);
                0
            }
        }
    }
}
