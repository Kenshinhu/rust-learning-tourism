struct Student {
    name: &'static str,
    score: i32,
}

fn main() {
    let score = 59;
    let username = "ZhangSan";

    let mut student = Student {
        score,
        name: username,
    };

    student.score = 60;

    let student2 = Student {
        name: "lisa",
        ..student
    };

    println!("name: {}, score: {}", student.name, student.score);
    println!("name: {}, score: {}", student2.name, student2.score);
}
