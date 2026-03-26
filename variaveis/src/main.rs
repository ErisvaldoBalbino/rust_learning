fn shadowing() {
    let y = 5;

    let y = y + 1;
    
    {
        let y = y * 2;
        println!("Valor de y no escopo interno: {y}");
    }

    println!("Valor de y: {y}");
}

fn main() {
    let mut x = 10;
    println!("Valor de x: {x}");
    x = 5;
    println!("Novo valor de x: {x}\n");
    shadowing();
}
