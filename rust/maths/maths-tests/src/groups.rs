use itertools::Itertools;
use std::fmt::{Debug, Display};

pub struct Group<const N: usize, T: Display + PartialEq> {
    name: String,
    neuteural_element: T,
    elements: [T; N],
    inverse_method: fn(&T) -> T,
    composition_method: fn((&T, &T)) -> T,
}

impl<const N: usize, T> Debug for Group<N, T>
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

impl<const N: usize, T: Display + PartialEq> Group<N, T> {
    pub fn inverse(&self, num: &T) -> T {
        (self.inverse_method)(num)
    }
    pub fn compose(&self, nums: (&T, &T)) -> T {
        (self.composition_method)(nums)
    }
    pub fn is_commutative(&self) -> bool {
        let elements = &self.elements;
        let mut is_commutative = true;
        for (a, b) in elements.iter().tuple_windows() {
            if self.compose((a, b)) != self.compose((b, a)) {
                is_commutative = false;
                break;
            }
        }
        is_commutative
    }

    pub fn simd_is_commutative(&self) -> bool {
        const CHUNK_SIZE: usize = 8;
        let elements = &self.elements;
        let mut is_commutative = true;
        for (a_chunks, b_chunks) in elements.chunks_exact(CHUNK_SIZE).tuple_windows() {
            for (a, b) in a_chunks.iter().zip(b_chunks) {
                println!("{a},{b}");
                if self.compose((a, b)) != self.compose((b, a)) {
                    is_commutative = false;
                    break;
                }
            }
        }
        is_commutative
    }

    pub fn is_inversable(&self) -> bool {
        let elements = &self.elements;
        let mut is_inversable = true;

        for element in elements {
            if self.compose((&self.inverse(element), element)) != self.neuteural_element {
                is_inversable = false;
                break;
            }
        }
        is_inversable
    }
    pub fn new<F: Fn((&T, &T)) -> T>(
        name: String,
        neuteural: T,
        inverse_method: fn(&T) -> T,
        composition_method: F,
        elements: [T; N],
    ) -> Group<N, T> {
        Group {
            name,
            neuteural_element: neuteural,
            composition_method,
            inverse_method,
            elements,
        }
    }
}
