fn gimme_the_letters(s: &str) -> String 
{
    (s.chars().nth(0).unwrap()..=s.chars().last().unwrap()).collect::<String>()
}
