extern crate faker;
use faker::Faker;

fn main() {

    let faker = Faker::new("en");

    println!("\n\n#### BEGIN LOREM ####")
    println!("faker.lorem.word() = {}\n",       faker.lorem.word());
    println!("faker.lorem.words() = {}\n",      faker.lorem.words(5));
    println!("faker.lorem.sentence() = {}\n",   faker.lorem.sentence(4, 6));
    println!("faker.lorem.sentences() = {}\n",  faker.lorem.sentences(3));
    println!("faker.lorem.paragraph() = {}\n",  faker.lorem.paragraph(3));
    println!("faker.lorem.paragraphs() = {}\n", faker.lorem.paragraphs(2));

    println!("\n\n#### BEGIN IMAGE ####")
    println!("faker.image.image() = {}",      faker.image.image(200, 400));
    println!("faker.image.avatar() = {}",     faker.image.avatar(50, 50, "tikotzky", "bmp"));
    println!("faker.image.category() = {}",   faker.image.category(200, 400, "business"));

    println!("\n\n#### BEGIN NAME ####")
    println!("faker.name.title() = {}",       faker.name.title());
    println!("faker.name.first_name() = {}",  faker.name.first_name());
    println!("faker.name.last_name() = {}",   faker.name.last_name());
    println!("faker.name.prefix() = {}",      faker.name.prefix());
    println!("faker.name.suffix() = {}",      faker.name.suffix());
    println!("faker.name.name() = {}",        faker.name.name());

    return;
}