use lombok_macros::*;
use std::fmt::Debug;

#[derive(Data, Debug, Clone, DisplayDebugFormat)]
struct LombokTest<'a, T: Clone + Debug> {
    #[get(pub(crate))]
    #[set(pub(crate))]
    list: Vec<String>,
    #[get(pub(crate))]
    opt_value: Option<&'a T>,
    #[get(pub(crate))]
    result_value: Result<&'a T, &'static str>,
    #[get_mut(pub(crate))]
    #[set(private)]
    name: String,
}

#[derive(CustomDebug, New)]
struct User {
    name: String,
    #[debug(skip)]
    _password: String,
    #[new(skip)]
    email: String,
}

#[derive(Data, Debug, Clone)]
struct TupleStruct(
    #[get(pub)] String,
    #[set(pub)] i32,
    #[get(pub)]
    #[set(pub)]
    bool,
);

#[derive(Data, Debug, Clone)]
struct TraitTestStruct {
    #[set(pub, AsRef)]
    name: String,
    #[set(pub, Into)]
    value: i32,
    #[set(pub, AsRef)]
    data: Vec<u8>,
    #[set(pub, Into)]
    items: Vec<String>,
}

#[derive(Data, Debug, Clone)]
struct TupleWithResult(#[get(pub)] String, #[get(pub)] Result<i32, &'static str>);

#[derive(CustomDebug)]
enum Response {
    Success {
        data: String,
    },
    Error {
        message: String,
        #[debug(skip)]
        _internal_code: u32,
    },
}

#[derive(New)]
struct Person {
    name: String,
    _age: u32,
}

#[derive(New)]
#[new(pub)]
struct PublicPerson {
    _name: String,
    _age: u32,
}

#[derive(New)]
#[new(pub(crate))]
struct CratePerson {
    _name: String,
    _age: u32,
}

#[derive(New)]
#[new(private)]
struct PrivatePerson {
    _name: String,
    _age: u32,
}

#[derive(Data, New)]
#[new(private)]
struct Product {
    id: u64,
    name: String,
    _price: f64,
    #[new(skip)]
    _description: String,
}

#[derive(New)]
struct TuplePoint(f64, #[new(skip)] f64, f64);

fn main() {
    let mut data: LombokTest<usize> = LombokTest {
        list: Vec::new(),
        opt_value: None,
        result_value: Err("error"),
        name: "test".to_string(),
    };
    let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    data.set_list(list.clone());
    assert_eq!(*data.get_list(), list);
    let opt_value: &Option<&usize> = data.try_get_opt_value();
    assert_eq!(*opt_value, None);
    data.set_opt_value(Some(&42));
    let try_opt_value: &Option<&usize> = data.try_get_opt_value();
    assert_eq!(try_opt_value, &Some(&42));
    let unwrap_value: &usize = data.get_opt_value();
    assert_eq!(unwrap_value, &42);
    let result_value: &Result<&usize, &str> = data.try_get_result_value();
    assert_eq!(*result_value, Err("error"));
    data.set_result_value(Ok(&100));
    let try_result_value: &Result<&usize, &str> = data.try_get_result_value();
    assert_eq!(try_result_value, &Ok(&100));
    let unwrap_result: &usize = data.get_result_value();
    assert_eq!(unwrap_result, &100);
    let name_mut: &mut String = data.get_mut_name();
    *name_mut = "updated".to_string();
    assert!(!data.to_string().is_empty());
    let mut tuple_data: TupleStruct = TupleStruct("hello".to_string(), 42, true);
    let field0: &String = tuple_data.get_0();
    assert_eq!(field0, "hello");
    tuple_data.set_1(100);
    let field2: &bool = tuple_data.get_2();
    assert!(*field2);
    tuple_data.set_2(false);
    let mut tuple_result = TupleWithResult("test".to_string(), Err("error"));
    let try_result: &String = tuple_result.get_0();
    assert_eq!(*try_result, String::from("test"));
    let try_result: &Result<i32, &str> = tuple_result.try_get_1();
    assert_eq!(*try_result, Err("error"));
    tuple_result.1 = Ok(42);
    let unwrap_result: i32 = tuple_result.get_1();
    assert_eq!(unwrap_result, 42);
    let user: User = User {
        name: "Alice".to_string(),
        _password: "secret123".to_string(),
        email: "alice@example.com".to_string(),
    };
    let user_debug: String = format!("{user:?}");
    assert!(user_debug.contains("Alice"));
    assert!(user_debug.contains("alice@example.com"));
    assert!(!user_debug.contains("secret123"));
    let success: Response = Response::Success {
        data: "Operation completed".to_string(),
    };
    let success_debug: String = format!("{success:?}");
    assert!(success_debug.contains("Operation completed"));
    let error: Response = Response::Error {
        message: "Something went wrong".to_string(),
        _internal_code: 500,
    };
    let error_debug: String = format!("{error:?}");
    assert!(error_debug.contains("Something went wrong"));
    assert!(!error_debug.contains("500"));
    let person = Person::new("Alice".to_string(), 30);
    assert_eq!(person.name, "Alice");
    let user = User::new("alice".to_string(), "alice@example.com".to_string());
    assert_eq!(user.email, "");
    let product = Product::new(1, "Laptop".to_string(), 999.99);
    assert_eq!(*product.get_id(), 1);
    assert_eq!(product.get_name(), "Laptop");
    let tuple_point = TuplePoint::new(10.5, 30.5);
    assert_eq!(tuple_point.0, 10.5);
    assert_eq!(tuple_point.1, 0.0);
    assert_eq!(tuple_point.2, 30.5);
    let public_person = PublicPerson::new("Alice".to_string(), 25);
    assert_eq!(public_person._name, "Alice");
    assert_eq!(public_person._age, 25);
    let crate_person = CratePerson::new("Bob".to_string(), 35);
    assert_eq!(crate_person._name, "Bob");
    assert_eq!(crate_person._age, 35);
    let private_person = PrivatePerson::new("Charlie".to_string(), 45);
    assert_eq!(private_person._name, "Charlie");
    assert_eq!(private_person._age, 45);
    let mut trait_test = TraitTestStruct {
        name: "test".to_string(),
        value: 42,
        data: vec![1, 2, 3],
        items: vec!["item1".to_string(), "item2".to_string()],
    };
    trait_test.set_name("new name");
    trait_test.set_value(100);
    trait_test.set_data(&[4, 5, 6, 7]);
    let new_items = vec!["new1".to_string(), "new2".to_string()];
    trait_test.set_items(new_items);
}
