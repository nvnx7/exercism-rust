use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    if phrase.is_empty() {
        return acronym;
    }

    acronym.push(phrase.chars().next().unwrap());
    let re = Regex::new(r"((\s|-|_)[a-zA-Z]|[^A-Z][A-Z])").unwrap();
    let matches: Vec<_> = re.find_iter(phrase).map(|m| m.start()).collect();

    let mut c: char;
    for i in matches {
        c = phrase.chars().nth(i + 1).unwrap();
        acronym.push(c);
    }

    acronym.to_uppercase()
}
