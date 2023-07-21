#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    let check_if_sublist = |_first_list: &[T], _second_list: &[T]| -> bool {
        _first_list.is_empty()
            || _second_list
                .windows(_first_list.len())
                .any(|l| l == _first_list)
    };

    if _first_list == _second_list {
        Comparison::Equal
    } else if check_if_sublist(_first_list, _second_list) {
        Comparison::Sublist
    } else if check_if_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}