use lombok_macros::*;

#[derive(Getter, Clone)]
struct TestStruct {
    #[get(pub, reference)]
    field_ref: String,

    #[get(pub, clone)]
    field_clone: String,

    #[get(pub)]
    field_default: String,

    #[get(pub, reference)]
    optional_ref: Option<String>,

    #[get(pub, clone)]
    optional_clone: Option<String>,
}

#[test]
fn test_reference_clone_getters() {
    let test = TestStruct {
        field_ref: "reference_field".to_string(),
        field_clone: "clone_field".to_string(),
        field_default: "default_field".to_string(),
        optional_ref: Some("optional_ref".to_string()),
        optional_clone: Some("optional_clone".to_string()),
    };

    // Test reference getter - should return &String
    let ref_result: &String = test.get_field_ref();
    assert_eq!(*ref_result, "reference_field");

    // Test clone getter - should return String
    let clone_result: String = test.get_field_clone();
    assert_eq!(clone_result, "clone_field");

    // Test default getter - should return &String (default behavior for non-Option)
    let default_result: &String = test.get_field_default();
    assert_eq!(*default_result, "default_field");

    // Test reference getter for Option - should return &Option<String>
    let opt_ref_result: &Option<String> = test.get_optional_ref();
    assert_eq!(opt_ref_result, &Some("optional_ref".to_string()));

    // Test clone getter for Option - should return Option<String>
    let opt_clone_result: Option<String> = test.get_optional_clone();
    assert_eq!(opt_clone_result, Some("optional_clone".to_string()));
}

#[test]
fn test_return_type_behavior() {
    let test = TestStruct {
        field_ref: "test".to_string(),
        field_clone: "test".to_string(),
        field_default: "test".to_string(),
        optional_ref: Some("test".to_string()),
        optional_clone: Some("test".to_string()),
    };

    // Verify reference returns work correctly
    let ref_val = test.get_field_ref();
    let same_ref_val = test.get_field_ref();
    // Both should point to the same underlying data
    assert!(std::ptr::eq(ref_val, same_ref_val));

    // Verify clone returns work correctly
    let clone_val1 = test.get_field_clone();
    let clone_val2 = test.get_field_clone();
    // These should be different owned values
    assert_ne!(
        std::ptr::addr_of!(clone_val1),
        std::ptr::addr_of!(clone_val2)
    );
    assert_eq!(clone_val1, clone_val2); // But have the same content
}
