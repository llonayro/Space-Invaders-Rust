use crate::entidades::{Direccion, Posicion};

use crate::entidades::jugador::Jugador;
use crate::entidades::enemigo::{Enemigo, TipoEnemigo};
use crate::interfaz::pantalla::Pantalla;

use my_random::MyRandom;



pub struct Juego {
    jugador: Jugador,
    enemigos: Vec<Enemigo>,
    puntuacion: u32,
    pub game_over: bool,
    pantalla: Pantalla
}

impl Juego {
    pub fn new() -> Self {
        let mut lista_enemigos = Vec::new();
        for y in 1..4 {
            for x in 28..33 {
                let tipo = match y {
                              1 => {
                                TipoEnemigo::Fuerte
                              }

                              2 => {
                                TipoEnemigo::Rapido
                              }

                              3 => {
                                TipoEnemigo::Normal
                              }

                              _ => {
                                TipoEnemigo::Normal
                              }
                           };
                lista_enemigos.push(Enemigo::new(tipo, Posicion::new(x, y)))
            }
        }
        Self{
            jugador: Jugador::new(),
            enemigos: lista_enemigos,
            puntuacion: 0,
            game_over: false,
            pantalla: Pantalla::new(),
        }

    }

    pub fn preparar_pantalla(&mut self){
        self.pantalla.limpiar_medio();

        for alien in self.enemigos.iter() {
            if alien.activo {
                let c = match alien.tipo {
                            TipoEnemigo::Fuerte => {
                                'Ѫ'
                            }

                            TipoEnemigo::Normal => {
                                '¤'
                            }

                            TipoEnemigo::Rapido => {
                                'Ѧ'
                            }

                        };
                
                self.pantalla.dibujar_punto(alien.posicion.x, alien.posicion.y, c)
            }
        }

        for disparo in self.jugador.disparos.iter() {
            self.pantalla.dibujar_punto(disparo.posicion.x, disparo.posicion.y, '⁞');
        }

        let mut disparos_enemigos = Vec::new();
        
        for alien in self.enemigos.iter().filter(|a| a.activo) {
            disparos_enemigos.extend(alien.disparos.iter());
        }
        

        for alien in self.enemigos.iter().filter(|a| a.activo){
            for d in disparos_enemigos.iter().filter(|d| d.posicion != alien.posicion){
                self.pantalla.dibujar_punto(d.posicion.x, d.posicion.y, '↓');
            }
        }

        self.pantalla.dibujar_punto(self.jugador.posicion.x, self.jugador.posicion.y, '▲')
    }

    pub fn mostrar(&self) {
        let mut vidas = String::new();
        for _ in 0..self.jugador.vida {
            vidas.push_str(" ❤️");
        }
        self.pantalla.renderizar(vidas, self.puntuacion)
    }

    pub fn contar_enemigos(&self) -> usize {
        self.enemigos.iter().filter(|a| a.activo).count()
    }

    pub fn actualizar_enemigos(&mut self){
        let x_max = self.enemigos.iter().map(|a| a.posicion.x).max().unwrap_or(0);
        let x_min = self.enemigos.iter().map(|a| a.posicion.x).min().unwrap_or(0);

        let en_borde_der = x_max == self.pantalla.pixeles[0].len() - 2;
        let en_borde_izq = x_min == 1;
        
        let direccion_actual = self.enemigos[0].direccion;

        let nueva_direccion = match(en_borde_der, en_borde_izq, direccion_actual){
                                        (true, _, Direccion::Derecha) => {
                                                                            Direccion::Abajo       
                                                                        }
                                        (true, _, Direccion::Abajo) => {
                                                                            Direccion::Izquierda
                                                                        }                                     
                                        
                                        (_, true, Direccion::Izquierda) => {
                                                                                Direccion::Abajo
                                                                            }
                                        (_, true, Direccion::Abajo) => {
                                                                            Direccion::Derecha        
                                                                        }

                                        _ => {
                                                direccion_actual
                                            }                                   
                                    };

        for alien in self.enemigos.iter_mut() {
            alien.direccion = nueva_direccion;
            alien.mover();
        }

        if self.enemigos.last().is_some_and(|e| e.posicion.y == 18) {
            self.game_over = true
        }

    }

    pub fn actualizar_disparos(&mut self) {
        
        for bala in self.jugador.disparos.iter_mut() {
            bala.mover();
            for alien in self.enemigos.iter_mut().filter(|a| a.activo && a.posicion == bala.posicion) {
                self.puntuacion += match alien.tipo {
                    TipoEnemigo::Fuerte => { 300 },
                    TipoEnemigo::Rapido => { 200 },
                    TipoEnemigo::Normal => { 100 },
                };
                alien.activo = false;
                bala.activo = false;
            }
            if bala.posicion.y == 0 {
                bala.activo = false;
            }
        }
        self.jugador.disparos.retain(|d| d.activo);


        let mut disparos_enemigos = Vec::new();
        for alien in self.enemigos.iter_mut().filter(|a| a.activo) {
            disparos_enemigos.extend(alien.disparos.iter_mut());
        }

        for d in disparos_enemigos.iter_mut() {
            d.mover();
        }
        for disparo in self.jugador.disparos.iter_mut() {
            for disp_ene in disparos_enemigos.iter_mut().filter(|de| de.posicion == disparo.posicion) {
                disparo.activo = false;
                disp_ene.activo = false;
            }
        }

        for disp in disparos_enemigos.iter() {

            if disp.posicion == self.jugador.posicion {
                self.jugador.vida -= 1;
                self.jugador.posicion.x = 30;
                self.jugador.posicion.y = 18;
            }

            if self.jugador.vida == 0 {
                self.game_over = true;
            }
        }

        disparos_enemigos.retain(|de| de.activo);
        


    }

    pub fn mover_jug_der(&mut self) {
        if self.jugador.posicion.x < self.pantalla.pixeles[0].len() - 2 {
            self.jugador.moverse_der();
        }
    }

    pub fn disparar(&mut self) {
        self.jugador.disparar();
    }

    pub fn disparar_enemigo(&mut self) {
        let mut rng = MyRandom::new();
        for alien in self.enemigos.iter_mut() {
            if rng.rand_range(1, 100) == 27 {
                alien.disparar();
            }

            if alien.posicion.x == self.jugador.posicion.x {
                if rng.rand_range(1, 5) == 2 {
                    alien.disparar();
                }
            }
        }
        
    }

    pub fn mover_jug_izq(&mut self) {
        if self.jugador.posicion.x > 1 {
            self.jugador.moverse_izq();
        }
    }

    pub fn terminar(&mut self) {
        self.pantalla.limpiar_medio();
        let texto = ['G', 'A', 'M', 'E', ' ', 'O', 'V', 'E', 'R'];
        let mut j = 0;

        for i in 25..34 {
            self.pantalla.pixeles[1][i] = texto[j];
            j += 1
        }

        self.rellenar();

        self.pantalla.renderizar_sin_cabecera();
    }

    pub fn ganaste(&mut self) {
        self.pantalla.limpiar_medio();

        let texto = ['G', 'A', 'N', 'A', 'S', 'T', 'E'];
        let mut j = 0;
        for i in 27..34 {
            self.pantalla.pixeles[1][i] = texto[j];
            j += 1
        }

        self.rellenar();

        self.pantalla.renderizar_sin_cabecera();
    }

    fn rellenar(&mut self) {
        let titulo = ['S', 'A', 'L', 'O', 'N', ' ', 'D', 'E', ' ', 'L', 'A', ' ', 'F', 'A', 'M', 'A'];
        let mut j = 0;
        for i in 22..38 {
            self.pantalla.pixeles[3][i] = titulo[j];
            j += 1
        }

        let primer_lugar = ['1', '.', ' ', 'B', 'i', 'l', 'l', 'y', ' ', 'M', 'i', 't', 'c', 'h', 'e', 'l', 'l', ' ', '.', '.', '.', '.', ' ', 'l', '0', '0', '0', '0'];
        let mut j = 0;
        for i in 16..44 {
            self.pantalla.pixeles[5][i] = primer_lugar[j];
            j += 1
        }

        let segundo_lugar = ['2', '.', ' ', 'L', 'i', 'n', 'u', 's', ' ', 'T', 'o', 'r', 'v', 'a', 'l', 'd', 's', ' ', '.', '.', '.', '.', ' ', '3', '0', '0', '0'];
        let mut j2 = 0;
        for i in 16..43 { 
            self.pantalla.pixeles[6][i] = segundo_lugar[j2];
            j2 += 1;
        }

        let tercer_lugar = ['3', '.', ' ', 'B', 'r', 'e', 'n', 'd', 'a', 'n', ' ', 'E', 'i', 'c', 'h', ' ', '.', '.', '.', '.', '.', '.', ' ', '3', '0', '0', '0'];
        let mut j3 = 0;
        for i in 16..43 { 
            self.pantalla.pixeles[7][i] = tercer_lugar[j3];
            j3 += 1;
        }

        for i in 8..10 {
            self.pantalla.pixeles[i][16] = '.';
        }

        let puntaje_str = format!("{:0>4}", self.puntuacion);
        let p_chars: Vec<char> = puntaje_str.chars().collect();
        let puesto = ['5', '1', '.', ' ', 'Y', 'o', 'u', ' ', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', ' ', p_chars[0], p_chars[1], p_chars[2], p_chars[3]];
        let mut jp = 0;
        for i in 16..43 {
            self.pantalla.pixeles[10][i] = puesto[jp];
            jp += 1;
        }

        for i in 11..13 {
            self.pantalla.pixeles[i][16] = '.';
        }

        let posicion_100 = ['1', '0', '0', '.', ' ', 'H', 'u', 'g', 'o', ' ', 'N', 'e', 'x', ' ', '.', '.', '.', '.', '.', '.', '.', '.', ' ', '0', '1', '0', '0'];
        let mut j100 = 0;
        for i in 16..43 { 
            self.pantalla.pixeles[13][i] = posicion_100[j100];
            j100 += 1;
        }
    }
}