const BG_GREEN: &str = "\x1b[48;5;2m";
const BG_YELLOW: &str = "\x1b[48;5;3m";
const BG_GRAY: &str = "\x1b[48;5;8m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub enum Color {
    Green,
    Yellow,
    Gray,
}

pub struct TermleResult<'a> {
    pub guess: &'a str,
    pub colors: Vec<Color>,     
}

pub fn create_termle<'a>(guess: &'a str, ans: &'a str) -> Option<TermleResult<'a>> {
    if guess.len() != ans.len() {
        return None;
    }

    let mut colors = Vec::new();
    for i in 0..guess.len() {
        if guess.chars().nth(i) == ans.chars().nth(i) {
            colors.push(Color::Green);
        } else if ans.contains(guess.chars().nth(i).unwrap()) {
            colors.push(Color::Yellow);
        } else {
            colors.push(Color::Gray);
        }
    }

    Some(TermleResult {
        guess: guess,
        colors: colors,
    })
}

pub fn match_color(color: &Color) -> &str {
    match color {
        Color::Green => BG_GREEN,
        Color::Yellow => BG_YELLOW,
        Color::Gray => BG_GRAY,
    }
}

impl TermleResult<'_> {
    pub fn to_ansi(&self) -> String {
        let mut s = String::new();
        s.push_str(BOLD);

        for c in 0..self.colors.len() {
            s.push_str(match_color(&self.colors[c]));
            s.push(self.guess.chars().nth(c).unwrap());
        }

        s.push_str(RESET);
        s
    }

    pub fn to_ansi_hidden(&self) -> String {
        let mut s = String::new();
        s.push_str(BOLD);

        for c in 0..self.colors.len() {
            s.push_str(match_color(&self.colors[c]));
            s.push(' ');
        }

        s.push_str(RESET);
        s
    }
}
