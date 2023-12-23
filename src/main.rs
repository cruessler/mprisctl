use clap::{Parser, ValueEnum};
use dbus::ffidisp::{BusType, Connection};
use dbus::Message;
use std::fmt;

#[derive(Debug)]
struct Player<'a> {
    name: String,
    conn: &'a Connection,
}

impl<'a> fmt::Display for Player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

trait Command {
    fn send_mpris_command(&self, command: &str);

    fn play_pause(&self);
    fn play(&self);
    fn pause(&self);
    fn stop(&self);
    fn next(&self);
}

impl<'a> Command for Player<'a> {
    fn send_mpris_command(&self, command: &str) {
        let message = Message::new_method_call(
            &self.name,
            "/org/mpris/MediaPlayer2",
            "org.mpris.MediaPlayer2.Player",
            command,
        )
        .expect("Could not create message");

        self.conn
            .send_with_reply_and_block(message, 2_000)
            .expect("Could not send message");
    }

    fn play_pause(&self) {
        self.send_mpris_command("PlayPause")
    }

    fn play(&self) {
        self.send_mpris_command("Play")
    }

    fn pause(&self) {
        self.send_mpris_command("Pause")
    }

    fn stop(&self) {
        self.send_mpris_command("Stop")
    }

    fn next(&self) {
        self.send_mpris_command("Next")
    }
}

fn get_players(conn: &Connection) -> Vec<Player<'_>> {
    let message = Message::new_method_call(
        "org.freedesktop.DBus",
        "/",
        "org.freedesktop.DBus",
        "ListNames",
    )
    .expect("Could not create message");

    conn.send_with_reply_and_block(message, 2_000)
        .expect("Could not send message")
        .get1::<Vec<&str>>()
        .expect("Could not get payload")
        .into_iter()
        .filter(|name| name.starts_with("org.mpris.MediaPlayer2."))
        .map(|name| Player {
            name: String::from(name),
            conn,
        })
        .collect::<Vec<_>>()
}

#[derive(ValueEnum, Clone, Debug)]
enum Action {
    /// List the names of players that can be controlled
    ListAll,
    PlayPause,
    Play,
    Pause,
    Stop,
    Next,
}

#[derive(Parser, Debug)]
/// Sends commands to MPRIS enabled players via DBUS
#[command(author, version, about)]
struct Args {
    /// The action to perform
    #[arg(required = false)]
    action: Action,
}

fn main() {
    let conn = Connection::get_private(BusType::Session).expect("Could not get DBUS connection");

    let args = Args::parse();

    if let Some(first) = get_players(&conn).first() {
        match args.action {
            Action::ListAll => {
                for p in get_players(&conn) {
                    println!("{}", p);
                }
            }
            Action::PlayPause => first.play_pause(),
            Action::Play => first.play(),
            Action::Pause => first.pause(),
            Action::Stop => first.stop(),
            Action::Next => first.next(),
        }
    } else {
        println!("No player found")
    }
}
