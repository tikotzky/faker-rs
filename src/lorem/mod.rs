use super::helpers;

struct Lorem {lorem: Vec<String>}

impl Lorem {
    fn new(lorem : Vec<String>) -> Lorem {
        Lorem {lorem: lorem}
    }

    fn words(&self, num: uint) -> Vec<String> {
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

mod tests;

