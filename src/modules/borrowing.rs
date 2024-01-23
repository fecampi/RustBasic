// "borrowing" permiti que partes do código façam referência a um valor sem tomar posse (ownership) dele.
// Isso é útil quando você deseja apenas emprestar temporariamente o acesso a um valor sem transferir sua propriedade,

// Função que aceita uma referência imutável
fn immutable_borrowing_example(x: &i32) {
    // Apenas leitura.
    println!("Valor através da referência imutável: {}", *x);
} // A referência sai de escopo, sem impacto no valor original.

// Função que aceita uma referência mutável
fn mutable_borrowing_example(y: &mut i32) {
    // Modificando o valor através da referência mutável.
    *y = 20;
    println!("Novo valor após referência mutável: {}", *y);
} // A referência sai de escopo, as alterações são refletidas no valor original.

pub fn default_function() {
    // imutável
    let a = 10;
    immutable_borrowing_example(&a);
    println!("Valor original após referência imutável: {}", a);

    // Exemplo de referência mutável
    let mut b = 15;
    mutable_borrowing_example(&mut b);
    println!("Valor original após referência mutável: {}", b);
}
