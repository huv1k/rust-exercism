#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.is_empty() {
        return true;
    }
    for sub_list in second_list.windows(first_list.len()) {
        if first_list == sub_list {
            return true;
        }
    }
    false
}
