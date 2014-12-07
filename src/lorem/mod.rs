use super::helpers;
use super::locale::Locale;

pub struct Lorem {
    lorem: Vec<&'static str>
}

impl Lorem {
    pub fn new(locale: Locale) -> Lorem {
        Lorem {lorem: locale.lorem}
    }

    fn word(&self) -> String {
        self.words(1).connect("")
    }

    fn words(&self, num: uint) -> Vec<&str> {
        helpers::shuffle(self.lorem.as_slice()).slice(0, num).to_vec()
    }

    fn sentence(&self, word_count: uint, range: uint) -> String {
        self.words(word_count + helpers::number_in_range(0, range)).connect(" ")
    }

    fn sentences(&self, sentence_count: uint) -> String {
        let mut sentences = Vec::new();
        for _ in range(0, sentence_count) {
            sentences.push(self.sentence(7, 3));
        }
        sentences.connect("\n")
    }

    fn paragraph(&self, sentence_count: uint) -> String {
        self.sentences(sentence_count + helpers::number_in_range(0, 3))
    }

    fn paragraphs(&self, paragraph_count: uint) -> String {
        let mut paragraphs = Vec::new();
        for _ in range(0, paragraph_count) {
            paragraphs.push(self.paragraph(3));
        }
        paragraphs.connect("\n \r\t")
    }
}

#[cfg(test)]
mod tests;

