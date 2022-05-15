#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
    let _first_list_length = _first_list.len();
    let _second_list_length = _second_list.len();

    if (_first_list_length == 0) & (_second_list_length > 0) {
        return Comparison::Sublist
    }

    if (_first_list_length > 0) & (_second_list_length == 0) {
        return Comparison::Superlist
    }

    if _first_list == _second_list {
        return Comparison::Equal
    } 

    // check for superlist case
    if _first_list_length > _second_list_length {
        let iteration = _first_list_length - _second_list_length;
        for i in 0..= iteration {
            let sub_list = &_first_list[i..(_second_list_length+i)];
            if _second_list == sub_list {
                return Comparison::Superlist
            }
        }
        return Comparison::Unequal

    }

    // check for sublist case
    if _first_list_length < _second_list_length {
        let iteration = _second_list_length - _first_list_length;
        for i in 0..= iteration {
            let sub_list = &_second_list[i..(_first_list_length+i)];
            if _first_list == sub_list {
                return Comparison::Sublist
            }
        }
        return Comparison::Unequal
    } else {
        return Comparison::Unequal
    }
}
