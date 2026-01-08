use lombok_macros::*;
use std::fmt::Debug;

#[derive(Data, Debug, Clone, DisplayDebugFormat)]
struct LombokTest<'a, T: Clone + Debug> {
    #[get(pub(crate))]
    #[set(pub(crate))]
    list: Vec<String>,
    #[get(pub(crate))]
    opt_value: Option<&'a T>,
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
        name: "test".to_string(),
    };
    let list: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    data.set_list(list.clone());
    assert_eq!(*data.get_list(), list);
    let opt_value: &Option<&usize> = data.get_opt_value();
    assert_eq!(*opt_value, None);
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
}
