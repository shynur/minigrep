mod to_test;
use to_test::*;

#[test]
fn one_result() {
    let query = "duct";
    let txt = String::from("\n")
        + "Rust:\n"
        + "safe, fast, productive.\n"
        + "Pick three.\n"
        + "Duct tape.";
    assert_eq!(vec!["safe, fast, productive."], search(query, &txt));
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let txt = String::from("\n")
        + "Rust:\n"
        + "safe, fast, productive.\n"
        + "Pick three.\n"
        + "Trust me.";
    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, &txt)
    );
}
