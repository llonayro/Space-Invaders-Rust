#[derive(Clone, Copy)]
enum Direccion {
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