// CIRCULAR REFERENCES (two object that reference one another)

// student* --- *course
// Vec<Enrollment {course, student}>

use std::rc::Rc;
use std::cell::RefCell;

struct Student<'a> {
    name: String,
//    course: Vec<&'a Course<'a>> // Refcell
//    courses: Vec<Rc<RefCell<Course>> // Rc Refcell
}

impl<'a> Student<'a> {
//    fn new(name: &str) -> Student<'a> {
//        Student {
//            name: name.into(),
//            courses: Vec::new()
//        }
//    }

    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter().filter(|&e| e.student.name ==self.name)
        .map(|e| e.course.name.clone()).collect()
    }
}

struct Course<'a> {
    name: String, 
//    students: Vec<&'a Student<'a>>
//    students: Vec<Rc<RefCell<Student>>
}

struct Enrollment<'a> {
    student: &Student,
    course: &Course
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl <'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course))
    }
}

impl <'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment {student, course}
    }
}

impl<'a> Course<'a> {
    fn new(name: &str) -> Course<'a> {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }

    // Refcell
//    fn add_student(&'a mut self, student: &'a mut Student<'a>) {
//        student.courses.push(self);
//        self.students.push(student);
        
        // Rc Refcell
    fn add_student(
        course: Rc<RefCell<Course>>, 
        student: Rc<RefCell<Student>>) {
            student.borrow_mut().courses.push(course.clone());
            course.borrow_mut().students.push(student.clone());
            course.borrow_mut().students.push(student);
    }
}

fn main () {
    // Refcell
//    let john = Student::new("John");
//    let course == Course::new("Rust Course");
//    course.add_student(john);

    // Rc Refcell
//    let john = Rc::new(RefCell::new(Student::new("John")));
//    let course = Course::new("Rust Course:");
//    let magic_course Rc::new(Refcell::new(course));

//    Course::add_student(magic_course, john);

    let john = Student {
        name: "John".into()
    };

    let course = Course {
        name: "Intro to Rust".into()
    };

    let mut p = Platforl::new();
    p.enroll(&john, &course);

    for c in john.courses(p) {
        println!("John is taking {}", c);
    }
}