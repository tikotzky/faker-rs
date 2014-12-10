#[cfg(not(test))]
extern crate faker;
#[cfg(not(test))]
use faker::Faker;

#[cfg(not(test))]
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
    println!("faker.image.image() = {}",        faker.image.image(200, 400));
    println!("faker.image.avatar() = {}",       faker.image.avatar(50, 50, "tikotzky", "bmp"));
    println!("faker.image.category() = {}",     faker.image.category(200, 400, "business"));

    println!("\n\n#### BEGIN NAME ####")
    println!("faker.name.title() = {}",         faker.name.title());
    println!("faker.name.first_name() = {}",    faker.name.first_name());
    println!("faker.name.last_name() = {}",     faker.name.last_name());
    println!("faker.name.prefix() = {}",        faker.name.prefix());
    println!("faker.name.suffix() = {}",        faker.name.suffix());
    println!("faker.name.name() = {}",          faker.name.full_name());

    println!("\n\n#### BEGIN NUMBER ####")
    println!("faker.number.digit() = {}",       faker.number.digit());
    println!("faker.number.number() = {}",      faker.number.number(5));

    println!("\n\n#### BEGIN PHONE NUMBER ####")
    println!("faker.phone_number.phone_number() = {}",  faker.phone_number.phone_number());
    println!("faker.number.phone_number_format() = {}", faker.phone_number.phone_number_format("A###-B###-C###"));

    println!("\n\n#### BEGIN HELPERS ####")
    println!("faker.helpers.number() = {}",             faker.helpers.number::<int>());
    println!("faker.helpers.number_in_range() = {}",    faker.helpers.number_in_range(0i, 10));

    println!("\n\n#### BEGIN ADDRESS ####")
    println!("faker.address.city_prefix() = {}",      faker.address.city_prefix());
    println!("faker.address.city_suffix() = {}",      faker.address.city_suffix());
    println!("faker.address.street_suffix() = {}",    faker.address.street_suffix());
    println!("faker.address.state() = {}",            faker.address.state());
    println!("faker.address.time_zone() = {}",        faker.address.time_zone());
    println!("faker.address.building_number() = {}",  faker.address.building_number());
    println!("faker.address.zip() = {}",              faker.address.zip());
    println!("faker.address.secondary_address() = {}",faker.address.secondary_address());
    println!("faker.address.city() = {}",             faker.address.city());
    println!("faker.address.street_name() = {}",      faker.address.street_name());
    println!("faker.address.street_address() = {}",   faker.address.street_address());
    println!("faker.address.state_abbr() = {}",       faker.address.state_abbr());
    println!("faker.address.country() = {}",          faker.address.country());
    println!("faker.address.latitude() = {}",         faker.address.latitude());
    println!("faker.address.longitude() = {}",        faker.address.longitude());




    return;
}