#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub stats: Stats,
    pub atb: i32,
}

#[derive(Debug)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

impl Character {
    pub fn new(name: &str, hp: i32, attack: i32, defense: i32, speed: i32) -> Self {
        Self {
            name: name.to_string(),
            stats: Stats { hp, attack, defense, speed },
            atb: 0,
        }
    }

    pub fn fill_atb(&mut self) {
        self.atb += self.stats.speed;

        if self.atb > 100 {
            self.atb = 100;
        }
    }

    pub fn is_ready(&self) -> bool {
        self.atb >= 100
    }

    pub fn attack_target(&mut self, target: &mut Character) {
        println!("{}は{}に{}のダメージ!\n\n", self.name, target.name, self.stats.attack);

        target.stats.hp -= self.stats.attack - self.stats.defense;
        self.atb = 0;

        if target.stats.hp <= 0 {
            target.stats.hp = 0;
        }
    }

    pub fn skill_attack(&mut self, target: &mut Character) {
        println!("{}のアポカリプスが4色の輝きで包まれる...", self.name);
        
        target.stats.hp = 0;
        self.atb = 0;

        if target.stats.hp <= 0 {
            target.stats.hp = 0;
        }
    }

    pub fn defend(&mut self) {
        self.stats.hp += self.stats.defense;
        println!("{}は防御態勢に入った！", self.name);
    }

    pub fn is_alive(&self) -> bool {
        self.stats.hp > 0
    }
}