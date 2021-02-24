// advanced topics


// enfocar las ref circulares con structs normales no funciona porque
// por ejemplo si se destruye un curso, el alumno queda con una referencia
// rota
// struct Student<'a> {
//     name: String,
//     course: Vec<&'a Course<'a>>
// }

// impl<'a> Student<'a>
// {
//     fn new(name: &str) -> Student<'a> 
//     {
//         Student{
//             name: name.into(),
//             courses: Vec::new()
//         }
//     }
// }

// struct Course <'a> {
//     name: String, 
//     students: Vec<&'a Student<'a>>
// }

// impl<'a> Course<'a>
// {
//     fn new(name: &str) -> Course<'a> 
//     {
//         Course{
//             name: name.into(),
//             students: Vec::new()
//         }
//     }

//     fn add_student(&'a mut self,
//         student: &'a mut Student<'a>) 
//     {
//         // no se puede usar como mut en dos lugares a la vez
//         student.courses.push(self);
//         self.students.push(student);
//     }
// }

// Version usando refCells:

use std::rc::Rc;
use std::cell::RefCell;

struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl<'a> Student
{
    fn new(name: &str) -> Student 
    {
        Student{
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course {
    name: String,
    students:  Vec<Rc<RefCell<Student>>>
}

impl Course
{
    fn new(name: &str) -> Course
    {
        Course{
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(
        course: Rc<RefCell<Course>>,
        student: Rc<RefCell<Student>>
    ) {
        // no puedo pasarle course derecho porque tomaria un mutable
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);

    }

}

// tercer enfoque: sin refcells
// student
// course
// Vec<enrollement {course, student}>

struct Student2 {
    name: String
}

impl Student2 {
    fn courses(&self, platform: Platform) -> Vec<String>
    {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()   
    }
}

struct Course2 {
    name: String
}

struct Enrollment<'a> {
    student: &'a Student2,
    course: &'a Course2
}

impl<'a> Enrollment<'a> {
    fn new(student: &'a Student2, course: &'a Course2)
        -> Enrollment<'a>
    {
        Enrollment{student, course}
    }
}

struct Platform<'a>
{
    enrollments: Vec<Enrollment<'a>>
}

impl<'a> Platform<'a> {
    fn new()-> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self, 
        student: &'a Student2,
        course: &'a Course2)
    {
        self.enrollments.push(
            Enrollment::new(student,course));
    }
}

pub fn advanced_topics () 
{

    // let fede = Student::new("fede");

    // let curso = Course::new("Rust");

    let fede = Rc::new(
        RefCell::new(
            Student::new("Fede")
        )
    );

    let carla = Rc::new(
        RefCell::new(
            Student::new("Carla")
        )
    );

    let curso = Course::new("Rust");

    let magic_course = Rc::new(RefCell::new(curso));

    // tengo que usar el clone para poder reusarlo
    Course::add_student(magic_course.clone(), fede);
    Course::add_student(magic_course, carla);

    // con platforms

    let juan = Student2{name: "Juan".into()};
    let cur = Course2{name: "matematica".into()};

    let mut p = Platform::new();
    p.enroll(&juan, &cur);

    for c in juan.courses(p) {
        println!("{} esta en el curso {}", juan.name, c)
    }


}
