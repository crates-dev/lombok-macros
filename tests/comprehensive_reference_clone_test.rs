use lombok_macros::*;

#[derive(Getter, Clone)]
struct ComprehensiveTestStruct {
    // Basic types
    #[get(pub, reference)]
    name_ref: String,

    #[get(pub, clone)]
    name_clone: String,

    // Option types
    #[get(pub, reference)]
    optional_string_ref: Option<String>,

    #[get(pub, clone)]
    optional_string_clone: Option<String>,

    // Vec types
    #[get(pub, reference)]
    vec_ref: Vec<i32>,

    #[get(pub, clone)]
    vec_clone: Vec<i32>,

    // With visibility and other attributes
    #[get(pub(crate), reference)]
    _crate_ref: String,

    #[get(private, clone)]
    _private_clone: String,
}

#[test]
fn test_comprehensive_reference_clone() {
    let test = ComprehensiveTestStruct {
        name_ref: "reference_name".to_string(),
        name_clone: "clone_name".to_string(),
        optional_string_ref: Some("opt_ref".to_string()),
        optional_string_clone: Some("opt_clone".to_string()),
        vec_ref: vec![1, 2, 3],
        vec_clone: vec![4, 5, 6],
        _crate_ref: "crate_reference".to_string(),
        _private_clone: "private_clone".to_string(),
    };

    // Test basic reference returns &String
    let name_ref: &String = test.get_name_ref();
    assert_eq!(*name_ref, "reference_name");

    // Test basic clone returns String
    let name_clone: String = test.get_name_clone();
    assert_eq!(name_clone, "clone_name");

    // Test option reference returns &Option<String>
    let opt_ref: &Option<String> = test.get_optional_string_ref();
    assert_eq!(opt_ref, &Some("opt_ref".to_string()));

    // Test option clone returns Option<String>
    let opt_clone: Option<String> = test.get_optional_string_clone();
    assert_eq!(opt_clone, Some("opt_clone".to_string()));

    // Test vec reference returns &Vec<i32>
    let vec_ref: &Vec<i32> = test.get_vec_ref();
    assert_eq!(vec_ref, &vec![1, 2, 3]);

    // Test vec clone returns Vec<i32>
    let vec_clone: Vec<i32> = test.get_vec_clone();
    assert_eq!(vec_clone, vec![4, 5, 6]);
}

#[derive(Getter, Clone)]
struct TestDefaultBehavior {
    #[get(pub)]
    default_field: String,

    #[get(pub)]
    default_option: Option<String>,
}

#[test]
fn test_default_behavior() {
    let test = TestDefaultBehavior {
        default_field: "default".to_string(),
        default_option: Some("option_default".to_string()),
    };

    // Default should return reference for non-Option types
    let default_ref: &String = test.get_default_field();
    assert_eq!(*default_ref, "default");

    // Default should return inner value for Option types (cloned)
    let default_option: String = test.get_default_option();
    assert_eq!(default_option, "option_default");
}
