pub enum Message{
    Partie,
    Points: Vec<(&str, usize, usize)>,
    Début,
    Fin,
}
