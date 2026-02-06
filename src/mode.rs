pub trait Mode{
    fn taille() -> Option<usize>;
    fn indice() -> Option<&'static str>;
    fn est_terminer() -> bool;
    fn soumettre();
    fn initialiser(detail:Option<usize>);
    fn pré_initialiser() -> Option<&'static str>;
    fn règle() -> &'static str;
}
