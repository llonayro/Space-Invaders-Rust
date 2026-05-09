
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType},
    cursor::{MoveTo, Hide, Show},
    execute,
};
use std::io::stdout;

use std::thread;
use std::time::Duration;

mod motor;
mod entidades;
mod interfaz;

use motor::juego::Juego;

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();

    let mut j = Juego::new();
    j.preparar_pantalla();
    j.mostrar();

    let mut contador: u32 = 0;
    loop {
        
        if contador % (j.contar_enemigos() + 1) as u32 == 0 {
            j.actualizar_enemigos();
        }

        if contador % 2 == 0 {
            j.actualizar_disparos();
        }

        if event::poll(Duration::from_millis(2)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind == event::KeyEventKind::Press{
                    match key_event.code {
                        KeyCode::Left => {
                            j.mover_jug_izq();
                        },

                        KeyCode::Right => {
                            j.mover_jug_der();
                        },

                        KeyCode::Char(' ') => {
                            j.disparar();
                        },
                        
                        KeyCode::Char('q') => {
                            break;
                        }

                        _ => {}
                    }
                }
            }
        }

        if j.game_over {
            j.terminar();
            break;
        }

        if j.contar_enemigos() == 0 {
            j.ganaste();
            break;
        }

        j.preparar_pantalla();
        j.mostrar();

        contador = contador.wrapping_add(1);
        thread::sleep(Duration::from_millis(33));
    }

    crossterm::terminal::disable_raw_mode().unwrap();
}