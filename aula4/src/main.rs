fn main() {
    let x = 10; // `x` é o dono do valor 10
    let y = &x; // `y` é uma referência imutável para `x` 1jhbjh
    println!("Valor de x: {}, via referência: {}", x, y);
} 