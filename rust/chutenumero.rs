use rand::Rng;

fn main() {
    let numero = 5;
    let mut rng = rand::thread_rng();
    let randomnumero = rng.gen_range(0..10);
    if numero == randomnumero {
        return println!("Acertou!");
    }
    if numero > randomnumero {
        println!("Chutou Alto!");
    } else {
        println!("Chutou Baixo!");
    }
}