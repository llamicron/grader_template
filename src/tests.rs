use rubric::TestData;

// Criteria tests go here

// This one will always pass
pub fn criteria1(_: &TestData) -> bool {
    true
}

// This will pass if they enter a number higher than 3
// The `data` comes from the submission struct
pub fn criteria2(data: &TestData) -> bool {
    if let Some(number) = data.get("entered_number") {
        return number.parse::<isize>().unwrap() > 3;
    }
    return false;
}
