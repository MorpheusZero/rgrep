use std::fs;

pub fn process_file(file_name: String, pattern_to_match: String) -> String {
    let contents = fs::read_to_string(file_name.clone())
        .expect(&*("Should have been able to read the file: ".to_owned() + &file_name));

    let text_processor = TextProcessor {
        full_raw_text: contents.to_string(),
        pattern_to_match
    };

    text_processor.search()
}

struct TextProcessor {
    full_raw_text: String,
    pattern_to_match: String,
}

impl TextProcessor {
    fn search(&self) -> String {

        // Split the raw text into an array
        let arr = self.full_raw_text.split("\n");

        // The matched lines
        let mut output: String = "".to_string();

        for str in arr {
            if str.to_lowercase().contains(&self.pattern_to_match.to_lowercase()) {
                let new_line = output.to_owned() + str + "\n";
                output = new_line.to_owned();
            }
        }

        println!("{}", output);

        return output
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_processing_pokemon_data_pikachu() {
        let result = super::process_file("./test-data/pokemon.csv".to_string(), "pikachu".to_string());
        assert!(result.contains("Pikachu"));
    }

    #[test]
    fn test_processing_pokemon_data_charizard() {
        let result = super::process_file("./test-data/pokemon.csv".to_string(), "charizard".to_string());
        assert!(result.contains("Fire"));
    }
}