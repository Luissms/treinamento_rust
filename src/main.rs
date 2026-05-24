const TIPO_DE_DADO:i8 = 2;
static UMA_VARIAVEL_STATIC:i8 = 3;


fn main() {
    println!("Constante: {}", TIPO_DE_DADO);
    println!("Variável estática: {}", UMA_VARIAVEL_STATIC);
    imprime();
}

fn imprime() {
    println!("Constante: {}", TIPO_DE_DADO)
}
