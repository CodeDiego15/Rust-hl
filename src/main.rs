fn main() {
   

    let mut _nombre: &str = "Diego";  
    let mut _apellido: &str = "Bermudez";
    let mut _edad: i32 = 25;
    println!("Estas son variables Tipo String Y Tipo Numero: {_nombre} {_apellido} {_edad}");
    _nombre = "Juan";
    _apellido = "Perez";
    _edad = 27;
    println!("Estas son variables Tipo String Y Tipo Numero: {_nombre} {_apellido} {_edad}");

    let mut _vscode: String = String::from("Visual Studio Code") ;

    let mut _num: i32 = 10;

    let _resultado: i32 = _num + 1;

    print!("XDD: {_vscode}");
}