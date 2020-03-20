use super::helpers;
use super::locale::Locale;

/// ## Lorem
///
/// ```
/// use faker::Faker;
/// let faker = Faker::new("en");
///
/// faker.lorem.word();         // => "eligendi"
/// faker.lorem.words(5);       // => ["molestiae", "et", "non", "qui", "nisi"]
/// faker.lorem.sentence(4, 6); // => "Qui soluta eos quia enim voluptatem rem."
/// faker.lorem.sentences(3);
/// // => ["Delectus magnam recusandae maxime sit et dolorem tenetur.", "Ut qui porro qui aperiam quae quos ab., Soluta beatae ut blanditiis odit amet et perferendis repellendus fugit.""]
/// faker.lorem.paragraph(3);
/// // => "Ut voluptatibus adipisci id doloremque odio nam libero distinctio vel. Beatae quos voluptas est ab cum quo nobis. Rerum occaecati rerum provident eligendi at soluta. Qui consequatur repellat voluptates nihil fugiat ea. Eius tempore voluptas enim culpa harum qui velit laboriosam omnis. Dolore est aspernatur qui a reiciendis eius culpa sunt."
/// faker.lorem.paragraphs(2);
/// // => ["Dolorem alias blanditiis harum sunt sit amet cum. Vitae quo nam rerum optio tenetur placeat. Cum quidem nesciunt cupiditate vel saepe voluptas dolore.", "Libero error porro quo esse quisquam beatae ex veritatis. Ut vitae voluptates impedit aliquam vel officiis porro aut amet. Dolorem quis doloribus nisi illum quia vero. Qui voluptatem repudiandae excepturi delectus earum beatae quos."]
/// ```
pub struct Lorem {
    lorem: Vec<&'static str>,
}

impl Lorem {
    pub fn new(locale: Locale) -> Lorem {
        Lorem {
            lorem: locale.lorem,
        }
    }

    pub fn word(&self) -> String {
        self.words(1).join("")
    }

    pub fn words(&self, num: usize) -> Vec<&str> {
        helpers::shuffle(&self.lorem)[0..num].to_vec()
    }

    pub fn sentence(&self, word_count: usize, range: usize) -> String {
        helpers::sentence_case(
            self.words(word_count + helpers::number_in_range(0, range))
                .join(" "),
        ) + "."
    }

    pub fn sentences(&self, sentence_count: u32) -> Vec<String> {
        let mut sentences = Vec::new();
        for _ in 0..sentence_count {
            sentences.push(self.sentence(7, 3));
        }
        sentences
    }

    pub fn paragraph(&self, sentence_count: u32) -> String {
        self.sentences(sentence_count + helpers::number_in_range(0, 3))
            .join("\n")
    }

    pub fn paragraphs(&self, paragraph_count: u32) -> Vec<String> {
        let mut paragraphs = Vec::new();
        for _ in 0..paragraph_count {
            paragraphs.push(self.paragraph(3));
        }
        paragraphs
    }
}

#[cfg(test)]
mod tests {
    use faker::Faker;

    #[test]
    fn test_words() {
        let faker = Faker::new("en");
        let words = faker.lorem.words(5);
        assert!(words.len() == 5);
    }

    #[test]
    fn test_sentence() {
        let faker = Faker::new("en");
        let sentence = faker.lorem.sentence(2, 5);
        let count = sentence.split(' ').count();
        assert!(count >= 2 && count <= 7);
    }
}
