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
