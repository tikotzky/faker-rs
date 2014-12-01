extern crate faker;
extern crate time;

fn main() {
    println!("Random # is: {}", faker::random::number::<int>());
    println!("Random from 2-10# is: {}", faker::random::number_in_range::<f64>(2.0, 10.0));

    let arr = ["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten"];
    println!("Random array element: {}", faker::random::array_element(&arr));

    println!("Random first name: {}", faker::name::first_name());
    println!("Random last name: {}", faker::name::last_name());
    println!("Random full name: {}", faker::name::find_name());

    let start_time = time::precise_time_ns();
    for x in range(0, 1000000i) {
        println!("{} {}", x, faker::name::find_name());
    }
    let end_time = time::precise_time_ns() ;
    let total_time= (end_time.to_f64().unwrap() - start_time.to_f64().unwrap()) / 1000000000f64;
    println!("total seconds: {}", total_time);

    return;
}