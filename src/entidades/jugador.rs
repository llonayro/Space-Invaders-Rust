use super::Posicion;
use super::disparo::Disparo;

pub struct Jugador {
    pub vida: u32,
    pub posicion: Posicion,
    pub disparos: Vec<Disparo>,
}

impl Jugador {
    pub fn new() -> Jugador {
        Jugador {
            vida: 3,
            posicion: Posicion::new(30, 18),
            disparos: Vec::new()
        }
    }

    pub fn disparar(&mut self) {
        let nueva_posicion = Posicion::new(self.posicion.x, self.posicion.y - 1);
        let disparo = Disparo::new(nueva_posicion, super::Direccion::Arriba);
        self.disparos.push(disparo);
    }

    pub fn moverse_der(&mut self) {
        self.posicion.x += 1
    }

    pub fn moverse_izq(&mut self) {
        self.posicion.x -= 1
    }
}