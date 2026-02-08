enum CharacterClass {
    Warrior(u32),
    Mage(u32),
    Archer(u32),
}

struct GameCharacter {
    name: String,
    level: u32,
    class: CharacterClass,
}

impl GameCharacter {
    fn attack(&self) -> u32 {
        match self.class {
            CharacterClass::Warrior(strength) => strength * self.level,
            CharacterClass::Mage(intelligence) => intelligence * self.level / 2,
            CharacterClass::Archer(agility) => agility * self.level * 2,
        }
    }

    fn describe(&self) -> String {
        match self.class {
            CharacterClass::Warrior(strength) => format!("Воин уровня {} атака {}", self.level, self.attack()),
            CharacterClass::Mage(intelligence) => format!("Маг уровня {} атака {}", self.level, self.attack()),
            CharacterClass::Archer(agility) => format!("Лучник уровня {} атака {}", self.level, self.attack()),
        }
    }
}

fn main() {
    let charc = GameCharacter {
        name: String::from("Vasya"),
        level: 52,
        class: CharacterClass::Mage(12)
    };

    println!("Атака: {}", charc.attack());
    println!("Информация: {}", charc.describe());
}