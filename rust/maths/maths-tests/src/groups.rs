use itertools::Itertools;
use std::fmt::{Debug, Display};

pub struct Group<T: Display + PartialEq> {
    name: String,
    neuteural_element: T,
    elements: Vec<T>,
    inverse_method: fn(&T) -> T,
    composition_method: fn((&T, &T)) -> T,
}

impl<T> Debug for Group<T>
where
    T: Display + PartialEq + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\nGroup: {}, elements: {:#?}, neuteural_element: {}\n}}",
            self.name, self.elements, self.neuteural_element
        )
    }
}

impl<T: Display + PartialEq> Group<T> {
    pub fn inverse(&self, num: &T) -> T {
        (self.inverse_method)(num)
    }
    pub fn compose(&self, nums: (&T, &T)) -> T {
        (self.composition_method)(nums)
    }
    pub fn check_validity(&self) -> (bool, bool) {
        let elements = &self.elements;
        let mut is_commutative = true;
        let mut is_inversable = true;
        for (a, b) in elements.iter().tuple_windows() {
            if self.compose((a, b)) == self.compose((b, a)) {
                println!("{a}, {b} are commutative");
            } else {
                println!("{a}, {b} are not commutative");
                is_commutative = false;
                break;
            }
        }

        for element in elements {
            if self.compose((&self.inverse(element), element)) == self.neuteural_element {
                println!("{element} is inversable");
            } else {
                is_inversable = false;
                println!("{element} isn't inversable");
                break;
            }
        }
        (is_commutative, is_inversable)
    }
    pub fn new(
        name: String,
        neuteural: T,
        inverse_method: fn(&T) -> T,
        composition_method: fn((&T, &T)) -> T,
        elements: Vec<T>,
    ) -> Group<T> {
        Group {
            name,
            neuteural_element: neuteural,
            composition_method,
            inverse_method,
            elements,
        }
    }
}
