trait Named {
    // 抽象方法必须实现
    fn name(&self) -> &str;
    // 有默认方法，impl时可不实现
    fn introduce(&self) -> String {
        format!("Hello, I'm {}", self.name())
    }
}
struct Student {
    name: String,
    grade: u32,
}
struct Teacher {
    name: String,
    subject: String,
}
impl Named for Student {
    fn name(&self) -> &str {
        // 注意别忘了&
        &self.name
    }
}
impl Named for Teacher {
    fn name(&self) -> &str {
        &self.name
    }
    fn introduce(&self) -> String {
        format!("Hello, I teach {}. My name is {}", self.subject, self.name)
    }
}
fn main() {
    let student = Student {
        name: String::from("black"),
        grade: 6,
    };
    let teacher = Teacher {
        name: String::from("ferris"),
        subject: String::from("rust"),
    };
    println!("{}", student.introduce());
    println!("{}", teacher.introduce());
}
