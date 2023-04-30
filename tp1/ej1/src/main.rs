mod finite_field_element;
use finite_field_element::FiniteFieldElement;

mod elliptic_curve;
mod point;

mod f64_element;

fn main() {
    // Ejercicio 1:
    // Implementar un tipo de dato para un elemento de cuerpo finito, junto con sus operaciones aritméticas fundamentales (adición, sustracción, multiplicación y división).
    // RTA: Se utilizo el modulo finite_field_element.rs para implementar el tipo de dato para un elemento de cuerpo finito,
    // junto con sus operaciones aritméticas fundamentales (adición, sustracción, multiplicación y división).
    let a = FiniteFieldElement::new(5, 7);
    let b = FiniteFieldElement::new(3, 7);
    // Ejemplo Suma:
    match a.add(&b) {
        Ok(sum) => println!("{} + {} = {}", a, b, sum),
        Err(e) => println!("{}", e),
    }
    // Ejemplo de Resta:
    match a.sub(&b) {
        Ok(diff) => println!("{} - {} = {}", a, b, diff),
        Err(e) => println!("{}", e),
    }
    // Ejemplo de Multiplicacion:
    match a.mul(&b) {
        Ok(mul) => println!("{} * {} = {}", a, b, mul),
        Err(e) => println!("{}", e),
    }
    // Ejemplo de Division:
    match a.div(&b) {
        Ok(div) => println!("{} / {} = {}", a, b, div),
        Err(e) => println!("{}", e),
    }

    // Ejercicio 2:
    // Implementar un tipo de dato para puntos de una curva elíptica, junto con las operaciones de grupo (suma de puntos distintos y duplicación de puntos),
    //  utilizando la forma de Weierstrass. Hacer pruebas con la curva y2=x3-3x-3 y p=1021, determinando la cantidad de puntos que tiene la curva.
    //  Usando P=(379,1011), obtener kP, siendo k=655.
}
