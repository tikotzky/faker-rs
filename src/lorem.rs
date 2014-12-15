use super::locale::Locale;
use super::helpers::Helpers;

pub struct Lorem {
    lorem: Vec<&'static str>,
    helpers: Helpers
}

impl Lorem {

    pub fn new(locale: Locale) -> Lorem {
        Lorem {
            lorem: locale.lorem,
            helpers: Helpers
        }
    }

    pub fn word(&self) -> String {
        self.words(1).connect("")
    }

    pub fn words(&self, num: uint) -> Vec<&str> {
        self.helpers.shuffle(self.lorem.as_slice()).slice(0, num).to_vec()
    }

    pub fn sentence(&self, word_count: uint, range: uint) -> String {
        self.helpers.sentence_case(self.words(word_count + self.helpers.number_in_range(0, range)).connect(" ")) + "."
    }

    pub fn sentences(&self, sentence_count: uint) -> Vec<String> {
        let mut sentences = Vec::new();
        for _ in range(0, sentence_count) {
            sentences.push(self.sentence(7, 3));
        }
        sentences
    }

    pub fn paragraph(&self, sentence_count: uint) -> String {
        self.sentences(sentence_count + self.helpers.number_in_range(0, 3)).connect("\n")
    }

    pub fn paragraphs(&self, paragraph_count: uint) -> Vec<String> {
        let mut paragraphs = Vec::new();
        for _ in range(0, paragraph_count) {
            paragraphs.push(self.paragraph(3));
        }
        paragraphs
    }    
}

