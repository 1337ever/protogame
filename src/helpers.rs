use crate::reagents::Reagent;
use std::collections::HashMap;

//copypasta
pub fn count_element_function<I>(it: I) -> HashMap<I::Item, u32>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut result = HashMap::new();

    for item in it {
        *result.entry(item).or_insert(0) += 1;
    }

    result
}

pub fn count<I>(it: I, item: &I::Item) -> u32
where
    I: IntoIterator,
    I::Item: PartialEq,
{
    it.into_iter().filter(|x| x == item).count() as u32
}

//function to spit out a vec of reagents based on a hashmap giving reagents and quantities
//bc i chose a weird system where the quantity of a reagent is the number of times it appears in the vec
pub fn gen_reagents(dict: HashMap<Reagent, usize>) -> Vec<Reagent> {
    let mut result: Vec<Reagent> = Vec::new();

    for (reagent, quantity) in dict {
        let mut count = 0;
        while count < quantity {
            result.push(reagent);
            count += 1;
        }
    }
    result
}
