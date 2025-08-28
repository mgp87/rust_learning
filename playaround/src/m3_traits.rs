#[cfg(test)]
mod tests {
    trait Attacker {
        fn choose_style(&self) -> String;
        fn choose_weapon(&self) -> String;
    }

    #[derive(Debug)]
    enum Character {
        Warrior,
        Archer,
        Wizard,
    }

    impl Attacker for Character {
        fn choose_style(&self) -> String {
            match self {
                Character::Warrior => String::from("Muay Thai"),
                Character::Archer => String::from("Archery"),
                Character::Wizard => String::from("Magical"),
            }
        }

        fn choose_weapon(&self) -> String {
            match self {
                Character::Warrior => String::from("Sword"),
                Character::Archer => String::from("Bow"),
                Character::Wizard => String::from("Staff"),
            }
        }
    }

    #[test]
    fn test_traits() {
        let my_character = Character::Warrior;
        assert_eq!(my_character.choose_style(), "Muay Thai");
        assert_eq!(my_character.choose_weapon(), "Sword");

        let my_character2 = Character::Archer;
        assert_eq!(my_character2.choose_style(), "Archery");
        assert_eq!(my_character2.choose_weapon(), "Bow");

        let my_character3 = Character::Wizard;
        assert_eq!(my_character3.choose_style(), "Magical");
        assert_eq!(my_character3.choose_weapon(), "Staff");
    }
}
