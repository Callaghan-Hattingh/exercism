#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // todo!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    else if _first_list.contains(_second_list) {
        return Comparison::Sublist;
        
    }x
    else {
        return Comparison::Unequal;
    }
}
