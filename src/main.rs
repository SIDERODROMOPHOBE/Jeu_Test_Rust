use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    
    let number = rand::thread_rng().gen_range(0..100);
    
    loop{
            let mut devine =String::new();
            io::stdin().read_line(&mut devine).expect("Entrée non fonctionnelle");


            let devine: u32 = match devine.trim().parse() {
                Ok(nombre) => nombre,
                Err(_) => continue,
            };
    

            match number.cmp(&devine) {
            Ordering::Equal => {
                println!("Gagné !");
                break;
            } 
            
            Ordering::Greater => println!("C'est plus grand"),
            Ordering::Less => println!("C'est plus petit"),
        }    
    }
}
