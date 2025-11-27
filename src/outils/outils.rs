use std::io;

pub fn demander(mut variable:String) -> String{
    io::stdin()
        .read_line(&mut variable)
        .expect("il y a un problème dans demander de outils");
    variable
}