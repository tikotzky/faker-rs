#Faker
[![faker travis](http://img.shields.io/travis/tikotzky/faker-rs.svg?branch=master&style=flat-square)](https://travis-ci.org/tikotzky/faker-rs)

Warning! This is still very much a work in progress and not production ready at the moment.
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
faker.name.name(); // "Tyshawn Johns Sr."
faker.name.first_name(); // "Kaci"
faker.name.last_name(); // "Ernser"
faker.name.prefix(); // "Mr."
faker.name.suffix(); // "IV"
```
## Lorem
```rust
faker.lorem.word(); // "repellendus"
// returns a Vec<String>
faker.lorem.words(4); // ["culpa", "recusandae", "aut", "omnis"]
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
- `X` or `#` => random int between `0-9`
- `Z` => random int between `1-9`
- `N` => random int between `2-9`

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

## Authors
* [Seth Pollack](https://github.com/sethpollack)
* [Mordy Tikotzky](https://github.com/tikotzky)

The MIT License  
Copyright (c) 2014 [Mordy Tikotzky](https://github.com/tikotzky), [Seth Pollack](https://github.com/sethpollack)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
