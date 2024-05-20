#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoly 4.0",
    ];

    // // menim hell yolum --start--
    // let mut good_students =  vec![];
    // for student in students{
    //     let v: Vec<&str> =student.split(' ').collect();
    //     //println!("{:?}", v[0]);
    //     let name = v[0].to_string();
    //     let score = v[1].parse::<f32>().unwrap();
    //     if score > 3.0 {
    //         good_students.push(Student{name, gpa:score});
    //     }
    // }
    // println!("{:?}", good_students);
    // // menim hell yolum --end--

    // // videodaki hell yolu combinatorsuz --start--
    // let mut good_students = vec![];
    // for student in students {
    //     let mut student = student.split(' ');
    //     let name = student.next();
    //     let score = student.next();
    //     if let (Some(n), Some(s)) = (name, score) {
    //         let na = n.to_string();
    //         let sc = s.parse::<f32>();
    //         if let Ok(scf) = sc {
    //             if scf >= 3.5 {
    //                 good_students.push(Student { name: na, gpa: scf });
    //             }
    //         }
    //     }
    // }
    // println!("{:?}", good_students);
    // // videodaki hell yolu combinatorsuz --end--

    // // MENIM HELL YOLUM COMBINATORLAR ILE --START--
    // let good_students = students.iter().map(|s|{
    //     let v: Vec<&str> =s.split(' ').collect();
    //          let name = v[0].to_string();
    //          let score = v[1].parse::<f32>().unwrap();
    //          (name, score)
    // }).filter(|d| d.1 > 3.5).map(|k|{
    //     Student{
    //         gpa:k.1,
    //         name:k.0
    //     }
    // }).collect::<Vec<Student>>();

    // // println!("{:?}", good_students);
    // // // MENIM HELL YOLUM COMBINATORLAR ILE --END--


    // // VIDEODAKI HELL YOLU COMBINATOR ILE --START--
    // let good_students: Vec<Student> = students.iter()
    // .map(|s|{
    //     let mut s = s.split(' ');
    //     let name = s.next()?.to_owned();
    //     let gpa = s.next()?.parse::<f32>().ok()?;
    //     Some(Student {name, gpa})
    // })
    // .flatten()
    // .filter(|s| s.gpa > 3.5)
    // .collect();

    // println!("{:?}", good_students)
    // // VIDEODAKI HELL YOLU COMBINATOR ILE --START--


    // VIDEODAKI HELL YOLU COMBINATOR ILE --START--
    let good_students: Vec<Student> = students.iter()
    .filter_map(|s|{
        let mut s = s.split(' ');
        let name = s.next()?.to_owned();
        let gpa = s.next()?.parse::<f32>().ok()?;
        if gpa>3.5{
            return None
        }
        Some(Student {name, gpa})
    })
    .collect();

    println!("{:?}", good_students)
    // VIDEODAKI HELL YOLU COMBINATOR ILE --START--


}
