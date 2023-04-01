use senjin::errors::ApiError;

#[test]
fn test_api_error_new() {
	let error = ApiError::new("Test error", Some(42));
	assert_eq!(error.message, "Test error");
	assert_eq!(error.code, Some(42));
}
