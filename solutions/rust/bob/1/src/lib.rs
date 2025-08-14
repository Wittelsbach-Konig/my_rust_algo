struct Reply;

impl Reply {
    pub const QUESTION: &'static str = "Sure.";
    pub const YELL: &'static str = "Whoa, chill out!";
    pub const YELL_QUESTION: &'static str = "Calm down, I know what I'm doing!";
    pub const SILENCE: &'static str = "Fine. Be that way!";
    pub const DEFAULT: &'static str = "Whatever.";
}

pub fn reply(message: &str) -> &str {
    let s = message.trim();
    if s.is_empty() {
        return Reply::SILENCE;
    }

    let is_question = s.ends_with('?');
    let mut letters = s.chars().filter(|c| c.is_alphabetic());
    let has_letters = letters.clone().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && letters.all(|c| c.is_uppercase());

    match (is_question, is_yelling) {
        (false, false) => Reply::DEFAULT,
        (true, false) => Reply::QUESTION,
        (false, true) => Reply::YELL,
        (true, true) => Reply::YELL_QUESTION,
    }
}