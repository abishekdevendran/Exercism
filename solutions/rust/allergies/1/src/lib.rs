pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

const ALLERGENS_LIST: [Allergen; 8] = [
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
        // todo!("Given the '{score}' score, construct a new Allergies struct.");
        let allergens = ALLERGENS_LIST
            .iter()
            .enumerate()
            .filter_map(|(idx, &el)| (score & (1 << idx) != 0).then_some(el))
            .collect();
        Self { allergens }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
