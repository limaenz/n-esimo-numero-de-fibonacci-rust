use std::io;

fn main() {
    println!("Qual n-ésimo número de Fibonacci quer gerar?");

    let mut valor = String::new();
    io::stdin().read_line(&mut valor)
    .expect("Falha ao ler entrada");

    let numero: i32 = valor
    .trim()
    .parse()
    .expect("Entrada não é um inteiro");

    let obter_sequencia = gerar_sequencia_fibonacci(numero);
    println!("{}", obter_sequencia);
}

fn gerar_sequencia_fibonacci(n: i32) -> i32 {
    if n == 1 {
        return 0
    }

    if n == 2 {
        return 1
    }

    let mut contador = 3;

    let mut a = 0;
    let mut b = 1;

    let mut calculo = 0;

    while contador <= n {
        calculo = a + b;

        a = b;
        b = calculo;
        contador = contador + 1;
    }

    return calculo;
}

