use lombok_macros::*;
use std::fmt::Debug;

#[derive(Data, Debug, Clone, DisplayDebugFormat)]
struct LombokTest<'a, 'b, T: Clone + Debug> {
    #[get(pub(crate))]
    #[set(pub(crate))]
    list: Vec<String>,
    #[get(pub(crate))]
    opt_str_lifetime_a: Option<&'a T>,
    #[set(private)]
    #[get_mut(pub(crate))]
    opt_str_lifetime_b: Option<&'b str>,
}

fn main() {
    let mut data: LombokTest<usize> = LombokTest {
        list: Vec::new(),
        opt_str_lifetime_a: None,
        opt_str_lifetime_b: None,
    };
    let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    data.set_list(list.clone());
    match data.get_list() {
        left_val => {
            assert_eq!(*left_val, list);
        }
    }
    let get_opt_str_lifetime_a: Option<usize> = data.get_opt_str_lifetime_a().cloned();
    assert_eq!(get_opt_str_lifetime_a, None);
    let get_mut_opt_str_lifetime_b: &mut Option<&str> = data.get_mut_opt_str_lifetime_b();
    *get_mut_opt_str_lifetime_b = Some("opt_str_lifetime_b");
    assert_eq!(
        data.get_mut_opt_str_lifetime_b().clone(),
        Some("opt_str_lifetime_b")
    );
    println!("{}", data.to_string());
}
