use std::io;

fn main() {
    /* Exercise one: Print vars */
    //print_vars();
    
    /* Exercise two  Receive inputs*/
    //get_user_input();

    // /* Exercise tree: conditions */
    conditions_print();

}

fn print_vars() {
    //print two vars
    let edad: u8 = 25;
    let nombre: &str = "John";
    println!("Soy {nombre}, tengo {edad} años");
}

fn get_user_input(){
    //get user inputs and print
    let mut edad = String::new();
    let mut nombre = String::new();

    println!("Ingresa tu edad");
    //get user input
    io::stdin().read_line(&mut edad).expect("Error");
    //convert string to number
    let edad_int: u8 = edad.trim().parse().expect("Error");
    
    println!("Ingresa tu nombre");
    io::stdin().read_line(&mut nombre).expect("Error");
    //Create var with user inputs
    //let mut frase: String = String::new();
    let frase = format!("Hola {}, tienes {} años", nombre, edad_int);
    println!("{}", frase);
}

fn conditions_print() {
    //get user inputs and print
    let mut edad = String::new();
    let mut nombre = String::new();

    println!("Ingresa tu edad");
    io::stdin().read_line(&mut edad).expect("Error");
    //convert string to number
    let edad_int: u8 = edad.trim().parse().expect("Error");
    
    println!("Ingresa tu nombre");
    io::stdin().read_line(&mut nombre).expect("Error");
    //Create var with user inputs
    //let mut frase: String = String::new();
    let frase = format!("Hola {}, tienes {} años", nombre, edad_int);
    if edad_int >= 18 {
        //print string with vars
        println!("{}, Eres mayor de edad", frase);
    } else {
        println!("{}, Eres menor de edad", frase);
    }
}