fn main() {
    let numero = 5;
    let mut res = numero;
    let mut i = numero - 1;
    while i > 0 {
        res = res * i;
        i -= 1;
    }
    println!("{res}");
}