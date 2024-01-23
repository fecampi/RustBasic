// Struct com anotações de tempo de vida
#[allow(dead_code)]
struct MyStruct<'a> {
    field1: &'a str,
    field2: &'a str,
}
/// # Em Campos:
/// são usadas nos campos para indicar a relação entre os tempos de vida das referências e a própria struct.


impl<'a> MyStruct<'a> {
    // Método com anotação de tempo de vida
    fn get_field1(&self) -> &'a str {
        self.field1
    }
}



pub fn default_function() {
    let string1 = String::from("Rust");
    let string2 = String::from("Programming");

    // Criando uma instância
    let my_struct = MyStruct {
        field1: &string1,
        field2: &string2,
    };

    // Chamando o método 
    let field1_value = my_struct.get_field1();
    println!("Valor do field1 na struct: {}", field1_value);

 
}
