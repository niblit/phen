mod constants;
mod csrng;
mod defaults;
mod wordlist;

pub struct Phen {
    length: usize,
    count: usize,
    generator: csrng::Csrng,
}

impl Phen {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();
        if args.contains(&"-h".to_string())
            || args.contains(&"--help".to_string())
            || args.contains(&"-u".to_string())
            || args.contains(&"--usage".to_string())
            || args.len() > 3
        {
            let name = &args[0];
            let message = format!(
                "\
Passphrase generator by niblit
usage: {name} [passphrase length] [passphrase count]"
            );
            return Err(message);
        }

        let length: usize = if let Some(s) = args.get(1) {
            let mut s = s.parse().unwrap_or(defaults::PASSPHRASE_LENGTH);
            if s == 0 {
                s = defaults::PASSPHRASE_LENGTH;
            }
            s
        } else {
            defaults::PASSPHRASE_LENGTH
        };

        let count: usize = if let Some(s) = args.get(2) {
            s.parse().unwrap_or(defaults::PASSPHRASE_COUNT)
        } else {
            defaults::PASSPHRASE_COUNT
        };

        Ok(Phen {
            length,
            count,
            generator: csrng::Csrng::new(),
        })
    }

    pub fn run(&mut self) {
        for _ in 0..self.count {
            let passphrase = self.get_passphrase(self.length);
            println!("{passphrase}");
        }
    }

    pub fn get_passphrase(&mut self, length: usize) -> String {
        let separator =
            constants::SEPARATORS[self.generator.random(constants::SEPARATORS.len())].to_string();
        let mut words: Vec<String> = Vec::new();

        let mut remaining = length;
        loop {
            if remaining == 0 {
                break;
            } else if remaining <= constants::MAXIMUM_PADDING_LENGTH {
                words.push(self.get_padding(remaining));
                break;
            }

            let word = self.get_word();
            let word_len = word.len();

            if word_len + 1 < remaining {
                words.push(word);
                remaining -= word_len + 1;
            }
        }

        let to_capitalize = self.generator.random(words.len());

        words[to_capitalize] = words[to_capitalize].to_ascii_uppercase();

        words.join(&separator)
    }
    fn get_word(&mut self) -> String {
        wordlist::EFF_LARGE_WORDLIST[self.generator.random(wordlist::EFF_LARGE_WORDLIST.len())]
            .to_string()
    }
    fn get_padding(&mut self, n: usize) -> String {
        let mut padding = Vec::new();
        for _ in 0..n {
            padding.push(
                constants::PADDING[self.generator.random(constants::PADDING.len())].to_string(),
            );
        }
        padding.join("")
    }
}
