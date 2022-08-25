use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let myst = rand::thread_rng().gen_range(1..=100);

    println!("************ Bienvenue ************");
    loop {
        let mut supposition = String::new();
        println!("Saisissez un nombre :");
        io::stdin()
            .read_line(&mut supposition)
            .expect("Erreur lors de la lecture de la saisie");

        let supposition: i32 = match supposition.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match supposition.cmp(&myst) {
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Less => println!("C'est plus !"),
            Ordering::Equal => {
                println!("Bravo, vous avez trouvé !");
                break;
            }
        }
    }
}
