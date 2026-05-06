use super::{Posicion, Direccion};

#[derive(Clone, Copy)]
pub enum TipoEnemigo {
    Normal,
    Rapido,
    Fuerte,
}

pub struct Enemigo {
    pub tipo: TipoEnemigo,
    pub posicion: Posicion,
    pub direccion: Direccion,
    activo: bool,
}

impl Enemigo {
    pub fn new(tipo: TipoEnemigo, posicion: Posicion) -> Self{
        Self {tipo, 
              posicion,
              direccion: super::Direccion::Derecha,
              activo: true }
    }

    pub fn mover(&mut self){
        match self.direccion {
            Direccion::Derecha => {
                self.posicion.x += 1
            }

            Direccion::Izquierda => {
                self.posicion.x -= 1
            }

            Direccion::Abajo => {
                self.posicion.y += 1
            }

            _ => {}
        }
    }
}