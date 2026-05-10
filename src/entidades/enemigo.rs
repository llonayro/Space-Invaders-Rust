use crate::entidades::disparo::Disparo;

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
    pub activo: bool,
    pub disparos: Vec<Disparo>,
}

impl Enemigo {
    pub fn new(tipo: TipoEnemigo, posicion: Posicion) -> Self{
        Self {tipo, 
              posicion,
              direccion: super::Direccion::Derecha,
              activo: true,
              disparos: Vec::new(),
            }
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

    pub fn disparar(&mut self){
        let pos = Posicion::new(self.posicion.x, self.posicion.y + 1);
        let disparo = Disparo::new(pos, Direccion::Abajo);
        self.disparos.push(disparo);
    }
}