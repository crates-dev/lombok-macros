use lombok_macros::*;
use std::{f64::consts::PI, fmt::Debug};

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

#[derive(CustomDebug, Getter, New)]
struct User {
    name: String,
    #[debug(skip)]
    _password: String,
    #[new(skip)]
    email: Option<String>,
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
    #[set(pub, type(AsRef<str>))]
    name: String,
    #[get(type(clone))]
    #[set(pub, type(Into<i32>))]
    value: i32,
    #[set(pub, type(AsRef<[u8]>))]
    data: Vec<u8>,
    #[set(pub, type(Into<Vec<String>>))]
    items: Vec<String>,
}

#[derive(Data, Debug, Clone)]
struct TupleWithResult(
    #[get(pub, type(clone))] String,
    #[get(pub)] Result<i32, &'static str>,
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

#[derive(Data, Debug, Clone)]
struct NestedStruct {
    #[get(pub)]
    name: String,
    #[set(pub)]
    _value: i32,
}

#[derive(Data, Debug, Clone)]
struct ComplexNestedStruct {
    #[get(pub)]
    nested: NestedStruct,
    #[get(pub)]
    nested_list: Vec<NestedStruct>,
    #[set(pub)]
    metadata: std::collections::HashMap<String, String>,
}

#[derive(CustomDebug)]
enum ComplexEnum {
    Simple,
    Tuple(String, i32),
    Struct {
        field1: String,
        #[debug(skip)]
        _secret: String,
        value: f64,
    },
}

#[derive(New)]
struct GenericStruct<T: Default + Clone> {
    #[new(skip)]
    data: T,
    value: i32,
}

#[derive(Data, Debug, Clone)]
struct LifetimesTest<'a, 'b> {
    #[get(pub)]
    name: &'a str,
    #[get(pub)]
    description: &'b str,
}

#[derive(Data, Debug, Clone)]
struct EdgeCaseTest {
    #[get(pub)]
    empty_string: String,
    #[get(pub, type(clone))]
    empty_vec: Vec<i32>,
    #[get(pub, type(clone))]
    zero_value: i32,
    #[get(pub, type(clone))]
    bool_false: bool,
    #[get(pub)]
    option_none: Option<String>,
}

#[derive(Data, Debug, Clone)]
struct CopyTest {
    #[get(skip)]
    _value: i32,
    #[get(pub(crate), type(copy))]
    flag: bool,
    #[get(private, type(copy))]
    count: u64,
}

#[derive(Data)]
struct UnitGetSet {
    #[get(pub)]
    flag: bool,
}

#[derive(New)]
struct AllSkipped {
    #[new(skip)]
    skipped1: String,
    #[new(skip)]
    skipped2: i32,
}

#[derive(Data, Debug, Clone)]
struct MultiAttributes {
    #[get(pub, type(clone))]
    #[set(pub(crate), type(Into<Vec<String>>))]
    complex_field: Vec<String>,
}

fn main() {
    let mut data: LombokTest<usize> = LombokTest {
        list: Vec::new(),
        opt_value: None,
        result_value: Err("error"),
        name: "test".to_string(),
    };
    let name: &mut String = data.get_mut_name();
    *name = "mut".to_string();
    assert_eq!(data.get_name(), "mut");
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
    let try_result: String = tuple_result.get_0();
    assert_eq!(try_result, String::from("test"));
    let try_result: &Result<i32, &str> = tuple_result.try_get_1();
    assert_eq!(*try_result, Err("error"));
    tuple_result.1 = Ok(42);
    let unwrap_result: i32 = tuple_result.get_1();
    assert_eq!(unwrap_result, 42);
    let user: User = User {
        name: "Alice".to_string(),
        _password: "secret123".to_string(),
        email: Some("alice@ltpp.vip".to_string()),
    };
    assert_eq!(user.get_name(), "Alice");
    assert_eq!(user.get_email(), "alice@ltpp.vip".to_string());
    let user_debug: String = format!("{user:?}");
    assert!(user_debug.contains("Alice"));
    assert!(user_debug.contains("alice@ltpp.vip"));
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
    let user = User::new("alice".to_string(), "alice".to_string());
    assert_eq!(user.email, None);
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
    trait_test.set_data([4, 5, 6, 7]);
    let new_items = vec!["new1".to_string(), "new2".to_string()];
    trait_test.set_items(new_items);
    assert_eq!(*trait_test.get_name(), "new name");
    assert_eq!(trait_test.get_value(), 100);
    assert_eq!(*trait_test.get_data(), vec![4, 5, 6, 7]);
    assert_eq!(
        *trait_test.get_items(),
        vec!["new1".to_string(), "new2".to_string()]
    );
    let nested = NestedStruct {
        name: "inner".to_string(),
        _value: 42,
    };
    let mut complex = ComplexNestedStruct {
        nested: nested.clone(),
        nested_list: vec![nested],
        metadata: std::collections::HashMap::new(),
    };
    complex.set_metadata({
        let mut map = std::collections::HashMap::new();
        map.insert("key".to_string(), "value".to_string());
        map
    });
    assert_eq!(complex.get_nested().get_name(), "inner");
    assert_eq!(complex.get_nested_list().len(), 1);
    assert_eq!(complex.get_metadata().get("key").unwrap(), "value");
    let simple = ComplexEnum::Simple;
    let tuple = ComplexEnum::Tuple("test".to_string(), 123);
    let struct_variant = ComplexEnum::Struct {
        field1: "visible".to_string(),
        _secret: "hidden".to_string(),
        value: PI,
    };
    let simple_debug = format!("{simple:?}");
    let tuple_debug = format!("{tuple:?}");
    let struct_debug = format!("{struct_variant:?}");
    assert!(simple_debug.contains("Simple"));
    assert!(tuple_debug.contains("test"));
    assert!(tuple_debug.contains("123"));
    assert!(struct_debug.contains("visible"));
    assert!(!struct_debug.contains("hidden"));
    assert!(struct_debug.contains("3.14"));
    let generic_i32 = GenericStruct::<i32> {
        data: 0,
        value: 100,
    };
    let generic_string = GenericStruct::<String>::new(200);
    assert_eq!(generic_i32.value, 100);
    assert_eq!(generic_i32.data, 0);
    assert_eq!(generic_string.value, 200);
    assert_eq!(generic_string.data, "");
    let name = "rust";
    let description = "language";
    let lifetimes_test = LifetimesTest { name, description };
    assert_eq!(*lifetimes_test.get_name(), "rust");
    assert_eq!(*lifetimes_test.get_description(), "language");
    let edge_case = EdgeCaseTest {
        empty_string: String::new(),
        empty_vec: Vec::new(),
        zero_value: 0,
        bool_false: false,
        option_none: None,
    };
    assert_eq!(edge_case.get_empty_string(), "");
    assert!(edge_case.get_empty_vec().is_empty());
    assert_eq!(edge_case.get_zero_value(), 0);
    assert!(!edge_case.get_bool_false());
    assert!(edge_case.try_get_option_none().is_none());
    let unit_get = UnitGetSet { flag: true };
    let flag_ref = unit_get.get_flag();
    assert!(*flag_ref);
    let constructed = AllSkipped::new();
    assert_eq!(constructed.skipped1, "");
    assert_eq!(constructed.skipped2, 0);
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
    let copy_test = CopyTest {
        _value: 42,
        flag: true,
        count: 1000,
    };
    let copied_flag: bool = copy_test.get_flag();
    let copied_count: u64 = copy_test.get_count();
    assert!(copied_flag);
    assert_eq!(copied_count, 1000);
}
