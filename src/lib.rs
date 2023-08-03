mod constants;

// avoid modulo bias
fn get_random_number(below_value: usize) -> usize {
    assert!(below_value > 0);
    loop {
        let number = rand::random::<usize>();
        if number < (usize::MAX - usize::MAX % below_value) {
            return number % below_value;
        }
    }
}

pub struct Phen {
    length: usize,
    count: usize,
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
            let mut s = s.parse().unwrap_or(constants::DEFAULT_PASSPHRASE_LENGTH);
            if s == 0 {
                s = constants::DEFAULT_PASSPHRASE_LENGTH;
            }
            s
        } else {
            constants::DEFAULT_PASSPHRASE_LENGTH
        };

        let count: usize = if let Some(s) = args.get(2) {
            s.parse().unwrap_or(constants::DEFAULT_PASSPHRASE_LENGTH)
        } else {
            constants::DEFAULT_PASSPHRASE_COUNT
        };

        Ok(Phen { length, count })
    }

    pub fn run(&self) {
        for _ in 0..self.count {
            let passphrase = self.get_passphrase(self.length);
            println!("{passphrase}");
        }
    }

    pub fn get_passphrase(&self, length: usize) -> String {
        let separator =
            constants::SEPARATORS[get_random_number(constants::SEPARATORS.len())].to_string();
        let mut words: Vec<String> = Vec::new();

        let mut remaining = length;
        loop {
            if remaining == 0 {
                break;
            } else if remaining <= constants::MAX_PADDING_LENGTH {
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

        let to_capitalize = get_random_number(words.len());

        words[to_capitalize] = words[to_capitalize].to_ascii_uppercase();

        words.join(&separator)
    }
    fn get_word(&self) -> String {
        constants::EFF_LW[get_random_number(constants::EFF_LW.len())].to_string()
    }
    fn get_padding(&self, n: usize) -> String {
        let mut padding = Vec::new();
        for _ in 0..n {
            padding
                .push(constants::PADDING[get_random_number(constants::PADDING.len())].to_string());
        }
        padding.join("")
    }
}
