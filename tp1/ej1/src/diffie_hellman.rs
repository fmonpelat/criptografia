use std::sync::{mpsc::{Sender, Receiver}, Arc, Mutex};

use crate::{point::Point};

pub struct DiffieHellman {}

impl DiffieHellman {

    pub fn calculate_shared_key(generator: &Point, random_number: u32, sender: Sender<Point>, receiver: Arc<Mutex<Receiver<Point>>> ) -> Point {
        let receiver = receiver.lock().unwrap();
        // actor_name calcula su punto publico con el numero random ( B = b * g )
        let actor_name_public_key = generator.scalar_mul(random_number).unwrap();
    
        // actor_name envia su clave publica a other_actor
        sender.send(actor_name_public_key).unwrap();
    
        // actor_name recibe la clave publica de other_actor
        let other_actor_public_key = receiver.recv().unwrap();
    
        // actor_name calcula la clave secreta compartida con other_actor public key (aca K = b * A)
        let shared_key = other_actor_public_key.scalar_mul(random_number).unwrap();

        shared_key
    }
}