use rand::seq::SliceRandom;

pub fn get_random_word() -> &'static str {
    let word_list = include_str!("words.txt").split_whitespace().collect::<Vec<&str>>();
    let mut rng = rand::thread_rng();
    word_list.choose(&mut rng).unwrap()
}