#[macro_use]
extern crate clap;
extern crate dbus;

use std::fmt;
use dbus::{BusType, Connection, Message};

#[derive(Debug)]
struct Player<'a> {
    name: String,
    conn: &'a Connection,
}

impl<'a> fmt::Display for Player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        ).expect("Could not create message");

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

fn get_players(conn: &Connection) -> Vec<Player> {
    let message = Message::new_method_call(
        "org.freedesktop.DBus",
        "/",
        "org.freedesktop.DBus",
        "ListNames",
    ).expect("Could not create message");

    conn.send_with_reply_and_block(message, 2_000)
        .expect("Could not send message")
        .get1::<Vec<&str>>()
        .expect("Could not get payload")
        .into_iter()
        .filter(|name| name.starts_with("org.mpris.MediaPlayer2."))
        .map(|name| {
            Player {
                name: String::from(name),
                conn: conn,
            }
        })
        .collect::<Vec<_>>()
}

fn main() {
    let conn = Connection::get_private(BusType::Session).expect("Could not get DBUS connection");

    let matches = clap_app!(mprisctl =>
        (version: env!("CARGO_PKG_VERSION")
        (author: "Christoph Rüßler <christoph.ruessler@mailbox.org>")
        (about: "Sends commands to MPRIS enabled players via DBUS")
        (@arg list_all: -l --("list-all") "List the names of players that can be controlled")
        (@arg COMMAND: "play-pause|play|pause|stop|next")
    ).get_matches();

    if matches.is_present("list_all") {
        for p in get_players(&conn) {
            println!("{}", p);
        }
    } else {
        if let Some(ref first) = get_players(&conn).first() {
            match matches.value_of("COMMAND") {
                Some("play-pause") => first.play_pause(),
                Some("play") => first.play(),
                Some("pause") => first.pause(),
                Some("stop") => first.stop(),
                Some("next") => first.next(),

                _ => println!("Unknown command"),
            }
        } else {
            println!("No player found")
        }
    }
}
