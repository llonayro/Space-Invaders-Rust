use super::{Direccion, Posicion};

pub struct Disparo {
    pub posicion: Posicion,
    pub direccion: Direccion,
}

impl Disparo {
    pub fn new (posicion: Posicion, direccion: Direccion) -> Self {
        Self {posicion, 
              direccion , 
            }
    }

    pub fn mover(&mut self){
        match self.direccion {
            Direccion::Arriba => {
                self.posicion.y -= 1
            }
            Direccion::Abajo => {
                self.posicion.y += 1
            }
            _ => {}
        }
    }
}