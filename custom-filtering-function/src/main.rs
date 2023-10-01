#![allow(unused)]

struct FilterCondition {
    filter_field: i32,
}

impl FilterCondition {
    fn is_match(&self, item: &i32) -> bool {
        item % 2 == self.filter_field
    }
}

fn custom_filter(collection: &[i32], filter_condition: &FilterCondition) -> Vec<i32> {
    let mut filtered_collection = Vec::new();

    for item in collection {
        if filter_condition.is_match(item) {
            filtered_collection.push(item.clone());
        }
    }

    filtered_collection
}

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let filter_condition = FilterCondition { filter_field: 0 };

    let res = custom_filter(&v, &filter_condition);
    assert_eq!(res, vec![2, 4, 6, 8, 10]);

    println!("{:#?}", res)
}
