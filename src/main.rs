use cliclack::input;

fn main() {
    let total_credits: i32 = input("Enter total credits: ")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    let num_courses: i32 = input("Enter number of courses: ")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    let courses: Vec<(String, i32, String)> =
        (0..num_courses).map(|_| get_course_details()).collect();

    if total_credits != courses.iter().map(|(_, credit, _)| credit).sum() {
        println!("Total credits do not match the sum of course credits!");
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

    println!("GPA: {:.2}", gpa);
}

fn get_course_details() -> (String, i32, String) {
    let course_code: String = input("Enter course code: ")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    let course_credit: i32 = input("Enter course credit: ")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Value is required!")
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
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap();

    return (course_code, course_credit, course_grade);
}
