use lombok_macros::*;
use std::{f64::consts::PI, fmt::Debug};

#[derive(Data, Debug, Clone)]
struct MultiAttributes {
    #[get(pub, type(clone))]
    #[set(pub(super), type(Into<Vec<String>>))]
    complex_field: Vec<String>,
}

fn main() {
    let multi = MultiAttributes {
        complex_field: vec!["test".to_string()],
    };
    let cloned_field = multi.get_complex_field();
    assert_eq!(cloned_field, vec!["test".to_string()]);
    let mut mutated = multi;
    let new_vec = vec!["new".to_string(), "values".to_string()];
    mutated.set_complex_field(new_vec.clone());
    let updated = mutated.get_complex_field();
    assert_eq!(updated, new_vec);
}
