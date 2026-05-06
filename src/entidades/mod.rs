pub mod jugador;
pub mod enemigo;
pub mod disparo;

#[derive(PartialEq, Clone, Copy)]
pub enum Direccion {
    Izquierda,
    Derecha,
    Arriba,
    Abajo,
}

pub struct Posicion {
    pub x: usize,
    pub y: usize,
}

impl Posicion{
    pub fn new(x: usize, y: usize) -> Posicion {
        Self{ x, y }
    }
}