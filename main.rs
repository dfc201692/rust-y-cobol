use std::io; // Importamos la biblioteca de entrada y salida estándar

fn main() {
    println!("Calculadora de suma en Rust"); // Imprimimos un mensaje de bienvenida
    
    // Pedimos al usuario que ingrese el primer número
    println!("Ingrese el primer número:");
    let mut num1 = String::new(); // Creamos una nueva cadena vacía para almacenar la entrada del usuario
    io::stdin().read_line(&mut num1).expect("Error al leer la entrada"); // Leemos la entrada del usuario y la almacenamos en num1
    
    // Convertimos la cadena ingresada por el usuario a un número entero
    let num1: i32 = num1.trim().parse().expect("Se esperaba un número entero");
    
    // Pedimos al usuario que ingrese el segundo número
    println!("Ingrese el segundo número:");
    let mut num2 = String::new(); // Creamos una nueva cadena vacía para almacenar la entrada del usuario
    io::stdin().read_line(&mut num2).expect("Error al leer la entrada"); // Leemos la entrada del usuario y la almacenamos en num2
    
    // Convertimos la cadena ingresada por el usuario a un número entero
    let num2: i32 = num2.trim().parse().expect("Se esperaba un número entero");
    
    // Calculamos la suma de los dos números
    let suma = num1 + num2;
    
    // Imprimimos el resultado
    println!("La suma de {} y {} es: {}", num1, num2, suma);
}
