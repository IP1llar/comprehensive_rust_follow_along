// For mocking, `Mockall` is a widely used library
// You need to refactor your code to use traits, which you can then quickly mock:
use std::time::Duration;

#[mockall::automock]
pub trait Pet {
    fn is_hungry(&self, since_last_meal: Duration) -> bool;
}

#[test]
fn test_robot_dog() {
    let mut mock_dog = MockPet::new();
    mock_dog.expect_is_hungry().return_const(true);
    assert_eq!(mock_dog.is_hungry(Duration::from_secs(10)), true);

}

// The advice here is for Android (AOSP) where Mockall is the recommended mocking library
// There are other mocking libraries available on crates.io, in particular in the area of mocking
// HTTP services. The other mocking libraries work in a similar fashion as Mockall, meaning that
// they make it easy to get a mock implementation of a given trait
//
// Note that mocking is somewhat controversial: mocks allow you to completely isolate a test from
// its dependencies. The immediate result is faster and more stable test execution. On the other
// hand, the mocks can be configured wrongly and return output different from what the real
// dependencies would do.
// If at all possible, it is recommended that you use the real dependencies. As an example, many
// databases allow you to configure an in-memory backend. This means that you get the correct
// behaviour in your tests, plus they are fast and will automatically clean up after themselves
// Similarly, many web frameworks allow you to start an in-process server which binds to a random
// port on `localhost`. Always prefer this over mocking away the framework since it helps you test
// your code in the real environment
//
// Mockall is not part of the Rust Playground, so you need to run this example in a local
// environment using `cargo add mockall` to quickly add Mockall to an existing Cargo project
//
// Mockall has a lot more functionality
// - In particular, you can set up expectations which depend on the arguments passed
// - Here we use this to mock a cat which becomes a hungry 3 hours after the last time it was fed:
#[test]
fn test_robot_cat() {
    let mut mock_cat = MockPet::new();
    mock_cat
        .expect_is_hungry()
        .with(mockall::predicate::gt(Duration::from_secs(3 * 3600)))
        .return_const(true);
    mock_cat.expect_is_hungry().return_const(false);
    assert_eq!(mock_cat.is_hungry(Duration::from_secs(1 * 3600)), false);
    assert_eq!(mock_cat.is_hungry(Duration::from_secs(5 * 3600)), true);
}

// You can use `.times(n)` to limit the number of times a mock method can be called to `n`
// - The mock will automatically panic when dropped if this isn't satisfied
