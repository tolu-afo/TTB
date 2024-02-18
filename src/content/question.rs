#[derive(Copy, Clone)]
pub struct Question {
    pub kind: QuestionKind,
    pub q: &'static str,
    pub a: &'static str,
}

#[derive(Copy, Clone)]
pub enum QuestionKind {
    ProgLang,
    MovieQuote,
    Scramble,
}

impl Question {
    const fn new(kind: QuestionKind, q: &'static str, a: &'static str) -> Self {
        Self { q, a, kind }
    }

    pub fn randomized() -> Self {
        let random = rand::random::<usize>();
        let index = random % QUESTIONS.len();
        QUESTIONS[index]
    }
}

#[rustfmt::skip]
const QUESTIONS: &[Question] = &[
    // Python
    Question::new(QuestionKind::ProgLang, "cant do a for loop", "python"),
    Question::new(QuestionKind::ProgLang, "named after a certain snake", "python"),
    Question::new(QuestionKind::ProgLang, "is commonly seen in machine learning", "python"),
    Question::new(QuestionKind::ProgLang, "will be killed by Mojo", "python"),
    // Rust
    Question::new(QuestionKind::ProgLang, "tokiiiiiiiiiiooooooo!", "rust"),
    Question::new(QuestionKind::ProgLang, "unsafe { /* trust me */ }", "rust"),
    Question::new(QuestionKind::ProgLang, "named as popular survival game", "rust"),
    Question::new(QuestionKind::ProgLang, "not endorsed by Rust foundation", "rust"),
    Question::new(QuestionKind::ProgLang, "is considered to be blazingly fast", "rust"),
    Question::new(QuestionKind::ProgLang, "fearless Arc<Mutex<HashMap<K, V>>>", "rust"),
    Question::new(QuestionKind::ProgLang, "cannot borrow as mutable because it is also borrowed as immutable", "rust"),
    // C
    Question::new(QuestionKind::ProgLang, "segfault", "c"),
    Question::new(QuestionKind::ProgLang, "king of undefined behavior", "c"),
    Question::new(QuestionKind::ProgLang, "main language of the linux kernel", "c"),
    // C++
    Question::new(QuestionKind::ProgLang, "was supposed to improve C", "c++"),
    Question::new(QuestionKind::ProgLang, "one of the most hated languages", "c++"),
    Question::new(QuestionKind::ProgLang, "has the most unredable standart library", "c++"),
    // Ocaml
    Question::new(QuestionKind::ProgLang, "a desert themed functional language", "ocaml"),
    // Zig
    Question::new(QuestionKind::ProgLang, "has `comptime` keyword", "zig"),
    Question::new(QuestionKind::ProgLang, "has a lizard mascot for the language", "zig"),
    // Go
    Question::new(QuestionKind::ProgLang, "if err != nil", "go"),
    Question::new(QuestionKind::ProgLang, "can go func yourself on accident", "go"),
    Question::new(QuestionKind::ProgLang, "appeals to certain blue haired individuals", "go"),
    Question::new(QuestionKind::ProgLang, "uses capital letters to denote public visibility", "go"),
    // Haskell
    Question::new(QuestionKind::ProgLang, "used by 35 people", "haskell"),
    Question::new(QuestionKind::ProgLang, "is like a burrito", "haskell"),
    Question::new(QuestionKind::ProgLang, "monad is a monoid in the category of endofunctors", "haskell"),
    // Racket 
    Question::new(QuestionKind::ProgLang, "this language is full of parenthesis", "racket "),
    // Php
    Question::new(QuestionKind::ProgLang, "can `explode`", "php"),
    Question::new(QuestionKind::ProgLang, "each developer of this language drives a lambo", "php"),
    // Jai
    Question::new(QuestionKind::ProgLang, "language created by Jonathan Blow, which will come out in the next 25 years", "jai"),

    // MovieQuotes
    Question::new(QuestionKind::MovieQuote, "May the Force be with you", "star wars"),
    Question::new(QuestionKind::MovieQuote, "I'm the king of the world!", "titanic"),
    Question::new(QuestionKind::MovieQuote, "It's alive! It's alive!", "frankenstein"),
    Question::new(QuestionKind::MovieQuote, "I'll be back", "terminator"),
    Question::new(QuestionKind::MovieQuote, "You're gonna need a bigger boat.", "jaws"),
    Question::new(QuestionKind::MovieQuote, "My precious", "lord of the rings"),
    Question::new(QuestionKind::MovieQuote, "Hey, you. Dumbass.", "walking dead"),
    Question::new(QuestionKind::MovieQuote, "Hey you're right, man. That is enough.", "walking dead"),

    // @Rework or remove? not as fun
    Question::new(QuestionKind::Scramble, "ulot", "tolu"),
    Question::new(QuestionKind::Scramble, "lopo", "pool"),
    Question::new(QuestionKind::Scramble, "chooectal", "chocolate"),
    Question::new(QuestionKind::Scramble, "ubritro", "burrito"),
    Question::new(QuestionKind::Scramble, "algansa", "lasagna"),
];
