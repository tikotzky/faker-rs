# Faker
[![Build Status](https://travis-ci.org/tikotzky/faker-rs.svg?branch=master)](https://travis-ci.org/tikotzky/faker-rs)

##Installation
Add faker to your Cargo.toml
```toml
[dependencies]
faker = "*"
```
## Usage
```rust
let faker = Faker::new("us");
faker.name.name(); // "Christophe Bartell"
```
## Name
```rust
faker.name.full_name(); // "Tyshawn Johns Sr."
faker.name.first_name(); // "Kaci"
faker.name.last_name(); // "Ernser"
faker.name.prefix(); // "Mr."
faker.name.suffix(); // "IV"
```
## Address
```rust
faker.address.city_prefix(); // "West"
faker.address.city_suffix(); // "Mouth"
faker.address.street_suffix(); // "Landing"
faker.address.state(); // "New Jersey"
faker.address.time_zone(); // "Europe/Helsinki"
faker.address.building_number(); // "853"
faker.address.zip(); // "00531"
faker.address.secondary_address(); // "Apt. 329"
faker.address.city(); // "North Rasheedview"
faker.address.street_name(); // "Haag Station"
faker.address.street_address(); // "2814 Shannon Roads"
faker.address.state_abbr(); // "AR"
faker.address.country(); // "Cyprus"
faker.address.latitude(); // "87.099724"
faker.address.longitude(); // "-58.324116"
```
## Lorem
```rust
faker.lorem.word();  // "eligendi"
faker.lorem.words(5) // ["molestiae", "et", "non", "qui", "nisi"]
faker.lorem.sentence(4, 6); // "Qui soluta eos quia enim voluptatem rem."
faker.lorem.sentences(3);
//["Delectus magnam recusandae maxime sit et dolorem tenetur.", "Ut qui porro qui aperiam quae quos ab., Soluta beatae ut blanditiis odit amet et perferendis repellendus fugit.""]
faker.lorem.paragraph(3);   
// "Ut voluptatibus adipisci id doloremque odio nam libero distinctio vel. Beatae quos voluptas est ab cum quo nobis. Rerum occaecati rerum provident eligendi at soluta. Qui consequatur repellat voluptates nihil fugiat ea. Eius tempore voluptas enim culpa harum qui velit laboriosam omnis. Dolore est aspernatur qui a reiciendis eius culpa sunt."
faker.lorem.paragraphs(2);
//["Dolorem alias blanditiis harum sunt sit amet cum. Vitae quo nam rerum optio tenetur placeat. Cum quidem nesciunt cupiditate vel saepe voluptas dolore.", "Libero error porro quo esse quisquam beatae ex veritatis. Ut vitae voluptates impedit aliquam vel officiis porro aut amet. Dolorem quis doloribus nisi illum quia vero. Qui voluptatem repudiandae excepturi delectus earum beatae quos."]
```
## Number
```rust
faker.number.number(10); // "1968353479"
faker.number.digit(); // "1"
```
## Phone Number
```rust
//returns a random number in a random format
faker.phone_number.phone_number(); // "397.693.1309"
//takes a format and returns a number
faker.phone_number.phone_number_format("1-NXX-NXX-XXXX"); // "1-397-693-1309"
//returns a random phone format
faker.phone_number.phone_formats(); // 1-NXX-NXX-XXXX"
```
###Formating
Phone numbers may be in any of the following formats:
* "NXX-NXX-XXXX"
* "(NXX)NXX-XXXX"
* "NXX.NXX.XXXX"
* "1-NXX-NXX-XXXX"
* "NXX-NXX-XXXX xNXX"
* "(NXX)NXX-XXXX xNXX"
* "1-NXX-NXX-XXXX xNXX"
* "NXX.NXX.XXXX xNXX"
* "NXX-NXX-XXXX xNXXX"
* "(NXX)NXX-XXXX xNXXX"
* "1-NXX-NXX-XXXX xNXXX"
* "NXX.NXX.XXXX xNXXX"
* "NXX-NXX-XXXX xNXXXX"
* "(NXX)NXX-NXX xNXXXX"
* "1-NXX-NXX-XXXX xNXXXX"
* "NXX.NXX.XXXX xNXXXX"

#####This format gets replaced with the following:
- 'X' or '#' => a number 0-9
- 'Z' => a number 1-9
- 'N' => a number 2-9

## Image
```rust
faker.image.avatar("my-own-slug", "50x50", "bmp") // "http://robohash.org/my-own-slug.bmp?size=50x50"
faker.image.category(100, 100, "cats") // "http://lorempixel.com/100/100/cats"
// returns an image with a random category
faker.image.image(100, 100) // "http://lorempixel.com/100/100/business"
```
#####Avaiable categories are:
* abstract
* animals
* business
* cats
* city
* food
* nightlife
* fashion
* people
* nature
* sports
* technics
* transport

##Contributing
1. Fork the repo.
3. Add a test for your change.
4. Make the test pass. `cargo test`
5. Push to your fork and submit a pull request.

## Authors
* [Seth Pollack](https://github.com/sethpollack)
* [Mordy Tikotzky](https://github.com/tikotzky)

***

The MIT License  
Copyright (c) 2014 [Mordy Tikotzky](https://github.com/tikotzky), [Seth Pollack](https://github.com/sethpollack)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
