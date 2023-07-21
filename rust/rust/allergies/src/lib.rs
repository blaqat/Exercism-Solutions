pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
    None,
}

const ALLERGEN: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            allergies: (0..8)
                .map(|n| (score >> n) & 1)
                .zip(ALLERGEN)
                .filter_map(
                    |(score, allergen)| {
                        if score == 1 {
                            Some(allergen)
                        } else {
                            None
                        }
                    },
                )
                .collect::<Vec<Allergen>>(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
