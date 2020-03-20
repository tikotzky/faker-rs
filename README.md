# Faker
[![Build Status](https://travis-ci.org/tikotzky/faker-rs.svg?branch=master)](https://travis-ci.org/tikotzky/faker-rs)

## Installation
Add faker to your Cargo.toml

```toml
[dependencies]
faker = "*"
```

## Usage

```rust
let faker = Faker::new("us");

faker.name.full_name();             // "Tyshawn Johns Sr."
faker.address.street_address();     // "2814 Shannon Roads"
faker.phone_number.phone_number();  // "397.693.1309"
```

For more detailed documentation read the [API
Documentaion](https://docs.rs/crate/faker).

## Contributing
1. Fork the repo
2. Add a test for your change
3. Make `cargo test` pass.
4. Push to your fork and submit a pull request

---

The MIT License  Copyright (c) 2014
[Mordy Tikotzky](https://github.com/tikotzky),
[Seth Pollack](https://github.com/sethpollack),
[Nathan Lilienthal](https://github.com/nixpulvis),

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
