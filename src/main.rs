use cliclack::{input, outro_cancel, outro_note};

fn main() {
    let total_credits: u32 = input("Enter total credits: ")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else if input == "0" {
                Err("Value must be greater than 0")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    let num_courses: u32 = input("Enter number of courses: ")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else if input == "0" {
                Err("Value must be greater than 0")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    let courses: Vec<(String, u32, String)> =
        (0..num_courses).map(|_| get_course_details()).collect();

    if total_credits != courses.iter().map(|(_, credit, _)| credit).sum() {
        outro_cancel("Total credits do not match the sum of course credits!").unwrap();
        return;
    }

    let gpa: f64 = courses
        .iter()
        .map(|(_, credit, grade)| {
            let grade_point = match grade.as_str() {
                "S" => 10.0,
                "A" => 9.0,
                "B" => 8.0,
                "C" => 7.0,
                "D" => 6.0,
                "E" => 5.0,
                "F" => 0.0,
                _ => 0.0,
            };
            grade_point * *credit as f64
        })
        .sum::<f64>()
        / total_credits as f64;

    outro_note("YOUR GPA", format!("GPA: {:.2}", gpa)).unwrap();
}

fn get_course_details() -> (String, u32, String) {
    let course_code: String = input("Enter course code: ")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else if input.len() != 6
                || !input.chars().take(4).all(|c| c.is_alphabetic())
                || !input.chars().skip(4).all(|c| c.is_numeric())
            {
                Err("Inavlid course code!")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    let course_credit: u32 = input("Enter course credit: ")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else if input == "0" {
                Err("Value must be greater than 0")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    let course_grade: String = input("Enter course grade: ")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else if input.chars().count() != 1
                || !input.chars().all(|c| c.is_alphabetic())
                || !matches!(
                    input.to_uppercase().as_str(),
                    "S" | "A" | "B" | "C" | "D" | "E" | "F"
                )
            {
                Err("Invalid grade!")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    (
        course_code.to_uppercase(),
        course_credit,
        course_grade.to_uppercase(),
    )
}
