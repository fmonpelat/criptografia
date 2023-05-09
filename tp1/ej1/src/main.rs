mod finite_field_element;
use std::sync::{Arc, Mutex};

use finite_field_element::FiniteFieldElement;

mod elliptic_curve;
use elliptic_curve::EllipticCurve;

mod point;
use point::Point;

mod diffie_hellman;
use crate::diffie_hellman::DiffieHellman;

use rand::Rng;

fn main() {
    // Ejercicio 1:
    // Implementar un tipo de dato para un elemento de cuerpo finito, junto con sus operaciones aritméticas fundamentales (adición, sustracción, multiplicación y división).
    // RTA: Se utilizo el modulo finite_field_element.rs para implementar el tipo de dato para un elemento de cuerpo finito,
    // junto con sus operaciones aritméticas fundamentales (adición, sustracción, multiplicación y división).
    println!("\nEjercicio 1:");
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

    // RTA: Se utilizo el modulo point.rs para implementar el tipo de dato para puntos de una curva elíptica,
    // junto con las operaciones de grupo (suma de puntos distintos func add y duplicación de puntos mult_scalar).

    println!("\nEjercicio 2:");
    // Se crea la curva eliptica y el punto p generador de la curva
    let curve = EllipticCurve::new(-3.0, -3.0);
    let p = Point::new(
        Some(FiniteFieldElement::new(379, 1021)),
        Some(FiniteFieldElement::new(1011, 1021)),
        curve.clone()
    ).expect("Error in Point::new");

    let k_p = p.scalar_mul(655).unwrap();
    println!("kP = {}", k_p);

    // Para determinar la cantidad de puntos que tiene la curva se utiliza el teorema de Hasse
    // p + 1 - 2sqrt(p) <= #E(Zp) <= p + 1 + 2sqrt(p) donde #E(Zp) es la cantidad de puntos de la curva
    // con p = 1021 queda: 1021 + 1 - 2sqrt(1021) <= #E(Zp) <= 1021 + 1 + 2sqrt(1021)

    // Se calcula la cantidad de puntos de la curva de forma naive (por cuestion de simplicidad ya que sino se deberia usar el algoritmo de Schoof)
    let mut count = 0;
    for x in 0..1021 {
        for y in 0..1021 {
            let point = Point::new(
                Some(FiniteFieldElement::new(x, 1021)),
                Some(FiniteFieldElement::new(y, 1021)),
                curve.clone()
            );
            if point.is_ok() {
                count += 1;
            }
        }
    }
    // debemos sumar 1 por el punto en el infinito
    count += 1;
    println!("Cantidad de puntos de la curva: {}", count);

    // con hasse sabemos que debe estar entre min y max
    let min = 1021.0 + 1.0 - 2.0 * 1021.0_f64.sqrt();
    let max = 1021.0 + 1.0 + 2.0 * 1021.0_f64.sqrt();
    println!("El orden calculado chequea Hasse? {} <= #E(Zp) <= {} = {}", min, max, min <= count as f64 && count as f64 <= max);
    
    
    // Ejercicio 3:
    // Implementar un esquema básico de acuerdo de clave de Diffie-Hellman usando curvas elípticas.
    // Usar la curva con p=43, y2=x3+6 y como generador g=(13,15). ¿Qué sucede si se emplea el punto g=(9,2)?

    // RTA: creamos dos threads para simular dos usuarios con un channel cada uno para comunicarse
    // Vemos que las claves generadas son distintas, por lo que se debe acordar el punto generador

    println!("\nEjercicio 3:");
    
    let (tx_alice, rx_bob) = std::sync::mpsc::channel::<Point>();
    let (tx_bob, rx_alice) = std::sync::mpsc::channel::<Point>();

    // se crea la curva eliptica y el punto generador
    let curve = EllipticCurve::new(0.0, 6.0);
    let generator = Point::new(
        Some(FiniteFieldElement::new(13, 43)),
        Some(FiniteFieldElement::new(15, 43)),
        curve.clone()
    ).expect("Error in Point::new generator");

    let generator2 = Point::new(
        Some(FiniteFieldElement::new(9, 43)),
        Some(FiniteFieldElement::new(2, 43)),
        curve.clone()
    ).expect("Error in Point::new generator");

    let mut handles = vec!();
    // se crea el thread de alice
    {
        let tx_alice_clone = tx_alice.clone();
        let generator = generator.clone();
        let generator2 = generator2.clone();
        let rx_alice = Arc::new(Mutex::new(rx_alice));
        let _alice_thread = std::thread::spawn(move || {
            // generar un numero random entre 1 y 100
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(1..100);
            println!("Random number Alice: {}", random_number);

            {
                let rx = rx_alice.clone();
                // alice calcula su punto publico con el numero random ( A = a * g )
                let shared_key = DiffieHellman::calculate_shared_key( &generator,random_number.clone(), tx_alice_clone.clone(), rx);
                println!("Shared key from alice first generator: {}", shared_key);
            }
            {
                let rx = rx_alice.clone();
                // generamos otra clave con el otro generador
                let shared_key = DiffieHellman::calculate_shared_key( &generator2, random_number.clone(),tx_alice_clone.clone(), rx);
                println!("Shared key from alice with second generator: {}", shared_key);
            }
        });
        handles.push(_alice_thread);
    }


    // se crea el thread de bob
    {
        let tx_bob_clone = tx_bob.clone();
        let generator = generator.clone();
        let rx_bob = Arc::new(Mutex::new(rx_bob));
        let _bob_thread = std::thread::spawn(move || {
            // generar un numero random entre 1 y 100
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(1..100);
            println!("Random number Bob: {}", random_number);

            {
                let rx = rx_bob.clone();
                // bob calcula su punto publico con el numero random ( B = b * g )
                let shared_key = DiffieHellman::calculate_shared_key(&generator,random_number.clone(),tx_bob_clone.clone(), rx);
                println!("Shared key from bob first generator: {}", shared_key);
            }
            {
                let rx = rx_bob.clone();
                // generamos otra clave con el otro generador
                let shared_key = DiffieHellman::calculate_shared_key( &generator2, random_number.clone(),tx_bob_clone.clone(), rx);
                println!("Shared key from bob with second generator: {}", shared_key);
            }

        });
        handles.push(_bob_thread);
    }

    // se espera a que terminen los threads join
    for handle in handles {
        handle.join().unwrap();
    }

    // Ejercicio 4:
    // Considerar la curva y2=x3+905x+100 definida sobre el cuerpo primo de orden 1021 y el punto generador (1006,416).
    // Desarrollar alguna estrategia que permita resolver el problema del logaritmo discreto kP=(612,827)

    // RTA: Debemos encontrar el valor k sabiendo que Kp=(612, 827), podemos emplear diferentes metodos para este punto como por ejemplo:
    // - Division por fuerza bruta
    // - Pollard's rho
    // - Shank's Baby-step Giant-step

    // Elegimos el metodo de fuerza bruta ya que es el mas simple y el orden de la curva es pequeño
    println!("\nEjercicio 4:");

    let curve = EllipticCurve::new(905.0, 100.0);
    let generator = Point::new(
        Some(FiniteFieldElement::new(1006, 1021)),
        Some(FiniteFieldElement::new(416, 1021)),
            curve.clone()
    ).expect("Error creating generator point doesnt belong to curve");
    let target = Point::new(
        Some(FiniteFieldElement::new(612, 1021)), 
        Some(FiniteFieldElement::new(827, 1021)),
        curve.clone()
    ).expect("Error creating target point doesnt belong to curve");
    let factor_k = generator.naive_factor(target).expect("Error factoring point");

    if factor_k.is_none() {
        println!("No se encontro el valor k");
    } else {
        println!("El valor k sabiendo que Kp=(612, 827) es {}", factor_k.unwrap());
    }

    


}
