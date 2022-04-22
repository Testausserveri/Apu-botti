//! Turn Finnish sentences into English sentences using a list of words.
//!
//! Should panic if the rule file is bad, but doesn't for some reason.
//! Doesn't panic if the message is bad.
//!
//! # Rule file format
//! [key] = [value] (newline)
//! where key is the Finnish word and value is the English word.
//!
//! # Example
//! "tervehdys" = "hello" (\n)
//!
//! # Note
//! - There should only be lower-case characters. Saving a few nanosenconds at run-time.
//! - The rule file shouldn't contain any mistakes.
//!   (Writing a linter for that later if this project comes back alive.)
//! - This thing doesn't understand contexts
//!   Example of that would be "terve".
//!   "terve" could mean "healthy" or "hello" depending on the context.

struct Translator {
    english_sentence: Vec<String>,
    sentence: Vec<String>,
    rules: String,
}

/// Use a thirdparty translator to translate the message.
/// Not ready. Maybe later.
pub fn translate_online(_input: Vec<String>) -> Result<Vec<String>, String> {
/*let client = reqwest::Client::new();
    let body = client.post("https://translate.yandex.net/api/v1/tr.json/translate")
        .body(r#"text=help&options=4"#)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{}",body);*/
    Ok(Vec::new())
}

/// Translate the text from Finnish to English according to the rule file.
/// The error should be sent by the Discord bot.
///
/// Note: Won't work as intended
/// if the one who wrote the message didn't add spaces
/// or the spaces are in weird places.
pub fn translate_rule_based(input: Vec<String>, rule_file: String) -> Result<Vec<String>, String> {
    // Add error handling here
    let r = std::fs::read_to_string(&rule_file).unwrap();

    let mut d = Vec::new();

    // Remove unwanted characters.
    for i in 0..input.len() {
        let mut temp = input[i].clone();

        if input[i].contains(".") {
            temp = input[i].replace(".", "")
        }

        if input[i].contains("?") {
            temp = input[i].replace("?", "")
        }

        if input[i].contains("!") {
            temp = input[i].replace("!", "")
        }

        d.push(temp.to_lowercase())
    }

    let mut t = Translator {
        english_sentence: Vec::new(),
        sentence: d,
        rules: r
    };

    let s = t.sentence.clone();

    for word in s {
        match t.translate(word.to_string()) {
            // Discord bot should send this message below in some form.
            Err(e) => return Err(format!("An error '{}' occured when attempted to translate() '{}'.", e, word)),
            Ok(_) => {}
        }
    }

    Ok(t.english_sentence)
}

impl Translator {
    /// Attempts to find `key` from rule file
    /// and returns the index of last character of the word
    /// or possibly an error.
    fn find(&self, key: &String) -> Result<usize, ()> {
        if key.is_empty() {
            return Err(())
        }

        for (i, c) in self.rules.chars().enumerate() {
            if c.to_string() == &key[0..1] {
                if i + key.len() > self.rules.len() {
                    // No more file left
                    // which means that `key` was not found.
                    return Err(())
                }

                if key.as_str() == &self.rules[i..i + key.len()] {
                    return Ok(i + key.len())
                }
            }
        }
        // `key` was not found
        return Err(())
    }
    /// Parse a value.
    // I tried to do this with a macro, but it didn't work :(
    fn parse_value(&mut self, index: usize) {
        let mut temp = String::new();
        let mut value_parsed = false;

        for i in index..self.rules.len() {
            match &self.rules[i..i + 1] {
                " " => {}
                "\n" => if value_parsed { break } else {
                    panic!("New line before value? Index: {}", index);
                }
                // Probably not a good idea to add this all-covering thing
                // when only looking for characters, but it's fine.
                c => {
                    temp.push_str(c);
                    value_parsed = true
                }
            }
        }

        self.english_sentence.push(temp);
    }

    /// Reads a `key` from the config file and returns the English equvalient of it
    /// or if an error occured, returns Err(()).
    /// If [key] = "Hello", returns "Hello".
    fn translate(&mut self, key: String) -> Result<(), String> {
        if key.is_empty() {
            return Err(String::from("`key` was empty"))
        }

        let index = match self.find(&key) {
            /*Err(e) => return Err(
                format!(
                "An error occured when attempted to find() '{0}'. Most likely '{0}' was not found in the rule file.",
                key
            )),*/
            Err(_) => {
                self.english_sentence.push(key);
                return Ok(())
            }
            Ok(v) => v
        };

        for i in index..self.rules.len() {
            // Is there a better way to do this than this slice thing?
            match &self.rules[i..i + 1] {
                "=" => { self.parse_value(i + 1); break }
                _   => {}
            }
        }

        Ok(())
    }
}

