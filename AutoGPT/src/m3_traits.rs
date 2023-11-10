trait  Attacker {
    fn  choose_style(&self) -> String;
}

#[derive(Debug)]
// trits used to define shared interface for methods that different types need to impliment
enum  character {
    Warrior,
    Archer,
    Wizard
}

impl Attacker for character{
    fn  choose_style(&self) -> String {
     match self {
         character::Warrior => "wing chun".to_string(),
         character::Archer => "king fu".to_string(),
         character::Wizard => "thai ci".to_string()
     }
    }
}

#[cfg(test)]

mod test{
    use super::*;

    #[test]
    fn test_traits(){
     let my_character :character = character::Archer;
     let choosen_style:String = my_character.choose_style();
     dbg!(choosen_style);

    }
}

