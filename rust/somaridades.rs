fn main() {
    let idades = [11, 12, 13, 15, 19, 32];
    let mut res = 0;
    for idade in idades {
        res = res + idade;
    }
    println!("{res}");
}