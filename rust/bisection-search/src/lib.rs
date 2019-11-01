fn bisection_search_helper<T>(list: Vec<T>, search: T, low: usize, high: usize) -> bool
where
    T: PartialEq + PartialOrd,
{
    if high == low {
        return list[low] == search;
    }

    let mid = (low + high) / 2;

    if list[mid] == search {
        return true;
    } else if list[mid] > search {
        if low == mid {
            return false;
        } else {
            return bisection_search_helper(list, search, low, mid - 1);
        }
    } else {
        return bisection_search_helper(list, search, mid + 1, high);
    }
}

pub fn bisection_search<T: PartialEq>(list: Vec<T>, search: T) -> Option<bool>
where
    T: PartialEq + PartialOrd,
{
    let len = list.len();
    match list.len() {
        0 => Some(false),
        _ => Some(bisection_search_helper(list, search, 0, len - 1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_gets_1() {
        let v = vec![1, 4, 3, 5, 6, 7, 9, 10, 56, 34, 2];
        assert_eq!(bisection_search(v, 1), Some(true));
    }
    #[test]
    fn it_gets_4() {
        let v = vec![1, 4, 3, 5, 6, 7, 9, 10, 56, 34, 2];
        assert_eq!(bisection_search(v, 4), Some(true));
    }
}
