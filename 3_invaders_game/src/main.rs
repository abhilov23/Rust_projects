mod player;

use crossterm::event;
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use rusty_audio::Audio;
use crossterm::{cursor, terminal, ExecutableCommand};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use invaders::{frame, render};
use invaders::frame::{new_frame, Drawable};
use player::Player;

fn main() -> Result <(), Box<dyn Error>> {
    
    let mut audio = Audio::new();
    audio.add("explode", "src/explode.wav"); //adding the sound of the game moments
    audio.add("lose", "src/lose.wav");
    audio.add("move", "src/move.wav");
    audio.add("pew", "src/pew.wav");
    audio.add("startup", "src/startup.wav");
    audio.add("win", "src/win.wav");
    
    //Tarminal
    let mut stdout = std::io::stdout(); 
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    //Render loop in a separate thread
    let(render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = std::io::stdout();
        let last_frame_snapshot = last_frame.clone(); // Create a separate copy or snapshot

        render::render(&mut stdout, &mut last_frame, &last_frame_snapshot, true);
        loop{
           let curr_frame =  match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &mut last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    //Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    'gameLoop: loop{
        //per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        //input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code{ //key input section
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter =>{
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                   KeyCode::Esc | KeyCode::Char('q')=>{ //exiting the game by pressing q or esc
                       audio.play("lose");
                       break 'gameLoop;
                   },
                   _ =>{}
                }
            }
        }

        //updates
        player.update(delta);


        //Draw and render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }
    //cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait(); //waiting for the sound to end
    stdout.execute(cursor::Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
