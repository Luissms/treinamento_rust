const TIPO_DE_DADO:i8 = 2;
static UMA_VARIAVEL_STATIC:i8 = 3;


fn main() {
    println!("Constante: {}", TIPO_DE_DADO);
    imprime();
}

fn imprime() {
    println!("Constante: {}", TIPO_DE_DADO)
}
