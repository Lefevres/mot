


pub trait Vocabulaire{
    fn suivant(&mut self) -> Option<String>;
    fn quel_numéro(&self) -> usize;
}
