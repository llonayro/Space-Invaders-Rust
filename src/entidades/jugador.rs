use super::Posicion;
use super::disparo::Disparo;

pub struct Jugador {
    vida: u32,
    pub posicion: Posicion,
    disparos: Vec<Disparo>,
}

impl Jugador {
    pub fn new() -> Jugador {
        Jugador {
            vida: 3,
            posicion: Posicion::new(30, 18),
            disparos: Vec::new()
        }
    }
}