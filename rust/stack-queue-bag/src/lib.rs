#![feature(box_into_raw_non_null)]
pub mod linked_list;
pub mod stack;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
