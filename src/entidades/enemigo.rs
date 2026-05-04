
#[derive(Clone, Copy)]
pub enum TipoEnemigo {
    Normal,
    Rapido,
    Fuerte,
}

pub struct Enemigo {
    tipo: TipoEnemigo,
    posicion: Posicion,
    direccion: Direccion,
    activo: bool,
}

impl Enemigo {
    pub fn new(tipo: TipoEnemigo, posicion: Posicion) -> Self{
        Self {tipo, 
              posicion,
              direccion: super::Direccion::Derecha,
              activo: true }
    }
}