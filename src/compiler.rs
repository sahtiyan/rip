// src/compiler.rs

use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct Compiler {
    version: String,
}

impl Compiler {
    pub fn new(version: &str) -> Compiler {
        Compiler {
            version: version.to_string(),
        }
    }

    pub fn version(&self) {
        println!("Rip compiler version: {}", self.version);
    }

    pub fn compile_file(&self, file_content: &str) -> String {
        let lexer = Lexer::new(file_content);
        let mut parser = Parser::new(lexer);

        let morse_code = parser.parse();
        let latin_code = self.convert_to_latin(&morse_code);
        let morse_code_back = self.convert_to_morse(&latin_code);

        println!("Original Morse: {}", morse_code);
        println!("Converted Latin: {}", latin_code);
        println!("Reconverted Morse: {}", morse_code_back);

        latin_code
    }

    fn convert_to_latin(&self, morse_code: &str) -> String {
        let morse_to_latin: Vec<&str> = morse_code.split(' ').map(|code| {
            match code {
                ".-" => "A",
                "-..." => "B",
                "-.-." => "C",
                "-.." => "D",
                "." => "E",
                "..-." => "F",
                "--." => "G",
                "...." => "H",
                ".." => "I",
                ".---" => "J",
                "-.-" => "K",
                ".-.." => "L",
                "--" => "M",
                "-." => "N",
                "---" => "O",
                ".--." => "P",
                "--.-" => "Q",
                ".-." => "R",
                "..." => "S",
                "-" => "T",
                "..-" => "U",
                "...-" => "V",
                ".--" => "W",
                "-..-" => "X",
                "-.--" => "Y",
                "--.." => "Z",
                _ => code,
            }
        }).collect();

        morse_to_latin.join("")
    }

    fn convert_to_morse(&self, latin_code: &str) -> String {
        let latin_to_morse: Vec<&str> = latin_code.chars().map(|c| {
            match c {
                'A' => ".-",
                'B' => "-...",
                'C' => "-.-.",
                'D' => "-..",
                'E' => ".",
                'F' => "..-.",
                'G' => "--.",
                'H' => "....",
                'I' => "..",
                'J' => ".---",
                'K' => "-.-",
                'L' => ".-..",
                'M' => "--",
                'N' => "-.",
                'O' => "---",
                'P' => ".--.",
                'Q' => "--.-",
                'R' => ".-.",
                'S' => "...",
                'T' => "-",
                'U' => "..-",
                'V' => "...-",
                'W' => ".--",
                'X' => "-..-",
                'Y' => "-.--",
                'Z' => "--..",
                _ => "",
            }
        }).collect();

        latin_to_morse.join(" ")
    }
}
