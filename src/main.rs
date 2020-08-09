#[macro_use] extern crate rubric;

// This is where our criteria tests live
mod tests;

// Import things from the rubric crate
use rubric::{Rubric, Submission, dropbox};

// Loads the rubric from the yaml file
fn load_rubric() -> Rubric {
    let yaml = yaml!("../rubrics/main.yml").expect("Couldn't read yaml file!");
    Rubric::from_yaml(yaml).expect("Bad yaml!")
}

// Build a submission with data contained in it
// Data will be used in grading, and sent back to you
fn build_submission() -> Submission {
    Submission::from_data(data! {
        "name" => prompt!("Name: ", String),
        "arbitrary" => "data",
        "entered_number" => prompt!("Number: ", String)
    })
}

fn main() {
    // Only open if you run with the "open_sesame" command line arg
    // Open this on one process and run the grader on another
    dropbox::open_with_arg("open_sesame", 8080);

    let mut rubric = load_rubric();
    let mut sub = build_submission();

    // Some added security
    sub.set_fingerprint("my super secret key shh don't tell");

    // Attach tests to rubric
    attach!(
        rubric,
        tests::criteria1,
        tests::criteria2
    );

    // Grade
    sub.grade_against(&mut rubric);

    // Print results to student
    println!("{}{}/{}", rubric, sub.grade, rubric.total_points());

    // URL to wherever your dropbox is hosted
    let url = "http://localhost:8080/submit";
    // Be sure to give feedback if the submission couldn't go through
    match sub.submit(url) {
        Ok(_) => println!("Submission recorded!"),
        Err(e) => {
            eprintln!("Something went wrong!");
            eprintln!("Couldn't submit. Error: {}", e);
        },
    }
}
