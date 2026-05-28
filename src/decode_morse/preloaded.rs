use std::collections::HashMap;

pub fn generate_map() -> HashMap<String, String> {
    let mut morse_code: HashMap<String, String> = HashMap::new();

    // Letters
    morse_code.insert(".-".to_string(),    "A".to_string());
    morse_code.insert("-...".to_string(),  "B".to_string());
    morse_code.insert("-.-.".to_string(),  "C".to_string());
    morse_code.insert("-..".to_string(),   "D".to_string());
    morse_code.insert(".".to_string(),     "E".to_string());
    morse_code.insert("..-.".to_string(),  "F".to_string());
    morse_code.insert("--.".to_string(),   "G".to_string());
    morse_code.insert("....".to_string(),  "H".to_string());
    morse_code.insert("..".to_string(),    "I".to_string());
    morse_code.insert(".---".to_string(),  "J".to_string());
    morse_code.insert("-.-".to_string(),   "K".to_string());
    morse_code.insert(".-..".to_string(),  "L".to_string());
    morse_code.insert("--".to_string(),    "M".to_string());
    morse_code.insert("-.".to_string(),    "N".to_string());
    morse_code.insert("---".to_string(),   "O".to_string());
    morse_code.insert(".--.".to_string(),  "P".to_string());
    morse_code.insert("--.-".to_string(),  "Q".to_string());
    morse_code.insert(".-.".to_string(),   "R".to_string());
    morse_code.insert("...".to_string(),   "S".to_string());
    morse_code.insert("-".to_string(),     "T".to_string());
    morse_code.insert("..-".to_string(),   "U".to_string());
    morse_code.insert("...-".to_string(),  "V".to_string());
    morse_code.insert(".--".to_string(),   "W".to_string());
    morse_code.insert("-..-".to_string(),  "X".to_string());
    morse_code.insert("-.--".to_string(),  "Y".to_string());
    morse_code.insert("--..".to_string(),  "Z".to_string());

    // Digits
    morse_code.insert(".----".to_string(), "1".to_string());
    morse_code.insert("..---".to_string(), "2".to_string());
    morse_code.insert("...--".to_string(), "3".to_string());
    morse_code.insert("....-".to_string(), "4".to_string());
    morse_code.insert(".....".to_string(), "5".to_string());
    morse_code.insert("-....".to_string(), "6".to_string());
    morse_code.insert("--...".to_string(), "7".to_string());
    morse_code.insert("---..".to_string(), "8".to_string());
    morse_code.insert("----.".to_string(), "9".to_string());
    morse_code.insert("-----".to_string(), "0".to_string());
    morse_code
}