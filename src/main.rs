use std::env;
use std::env::Args;
use std::os::fd::AsFd;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;
#[derive(Debug)]
struct Player {
    name: String,
    title: String,
    status: String,
    is_playing: bool,
}

fn pause_player(player: Player) {
    if player.is_playing {
        println!("{:?}", player);
    }

    // os.popen(f"playerctl -p {playerlist.name(index)} pause").read()
}

fn player_is_playing(status: &str) -> bool {
    match status {
        "Playing" => true,
        "Paused" => false,
        "Stopped" => false,
        &_ => todo!(),
    }
}
fn build_player(name: String, title: String, status: String, is_playing: bool) -> Player {
    Player {
        name,
        title,
        status,
        is_playing,
    }
}
fn get_playerlist() -> Vec<String> {
    let output = Command::new("playerctl")
        .arg("--list-all")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let player_list = String::from_utf8(output.stdout).unwrap();
    println!("{:?}", player_list);
    let mut v: Vec<String> = player_list.split("\n").map(|s| s.to_string()).collect();
    v.pop();

    return v;
}

fn get_players(player_list: Vec<String>) -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    for (_idx, player) in player_list.iter().enumerate() {
        let status = String::from_utf8(
            Command::new("playerctl")
                .args(["status", "-p", &player])
                .output()
                .expect("failed to execute process")
                .stdout,
        )
        .unwrap()
        .replace("\n", "");
        let is_playing = player_is_playing(status.as_str());
        let title = String::from_utf8(
            Command::new("playerctl")
                .args([
                    "-p",
                    &player,
                    "metadata",
                    "--format",
                    "'{{ artist }} - {{ title }}'",
                ])
                .output()
                .expect("failed to execute process")
                .stdout,
        )
        .unwrap()
        .replace("\n", "");

        players.push(build_player(player.to_string(), title, status, is_playing));
    }
    players
}
fn show_rofi_menu(players: &Vec<Player>, looping: bool) -> usize {
    let mut options: Vec<String> = Vec::new();
    for player in players {
        options.push(String::from(format!(
            "({}) {} {}",
            player.status, player.name, player.title
        )))
    }
    options.push(String::from("---------"));
    options.push(String::from("Pause All"));
    options.push(String::from("Next Track"));
    options.push(String::from("Prev Track"));
    if looping {
        options.push(String::from("Quit"))
    }
    // println!("options {:?}", options);
    let echo_cmd = Command::new("echo")
        .args([&options.join("\n")])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // unsafe {
    let selected = 0;
    let prompt = "Select Player";
    let rofi_cmd = Command::new("rofi")
        .args([
            "-dmenu",
            "-p",
            format!("{}", prompt).as_str(),
            "-format",
            "i",
            "-selected-row",
            format!("{}", selected).as_str(),
            "-me-select-entry",
            "",
            "-me-accept-entry",
            "MousePrimary",
        ])
        .stdin(Stdio::from(echo_cmd.stdout.unwrap()))
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    println!("{:}",rofi_cmd.status);
    if rofi_cmd.stdout.len() > 0 {
        let selected_line = String::from_utf8(rofi_cmd.stdout)
            .unwrap()
            .replace("\n", "")
            .parse::<u8>()
            .unwrap_or_else(|e| panic!("failed to read selected line: {}", e));
        // println!("selected line {:?}", selected_line);
        usize::from(selected_line)
    } else {
        0
    }
}

fn pause(player: &Player) {
    // playerctl -p {playerlist.name(index)} pause"
    let _output = Command::new("playerctl")
        .args(["-p", player.name.as_str(), "pause"])
        .output()
        .expect("failed to execute process");
}

fn play(player: &Player) {
    let _output = Command::new("playerctl")
        .args(["-p", player.name.as_str(), "play"])
        .output()
        .expect("failed to execute process");
}

fn play_pause(player: &Player) {
    let _output = Command::new("playerctl")
        .args(["-p", player.name.as_str(), "play-pause"])
        .output()
        .expect("failed to execute process");
}

fn pause_all(players: &Vec<Player>) {
    for player in players.iter() {
        if player.status == "Playing" {
            pause(&player);
        }
    }
}

fn next_track(players: &Vec<Player>) {
    for player in players.iter() {
        if player.status == "Playing" {
            let _output = Command::new("playerctl")
                .args(["-p", player.name.as_str(), "next"])
                .output()
                .expect("failed to execute process");
        }
    }
}

fn prev_track(players: &Vec<Player>) {
    for player in players.iter() {
        if player.status == "Playing" {
            let mut cmd = Command::new("playerctl");
            cmd.args(["-p", player.name.as_str(), "previous"]);
            cmd.output().expect("failed to execute process");
            cmd.output().expect("failed to execute process");
            // println!("prev track {:?}", _output);
        }
    }
}
fn show_rofi(looping: bool) {
    let player_list = get_playerlist();
    let players: Vec<Player> = get_players(player_list);
    let selected_line = show_rofi_menu(&players, looping);
    if selected_line < players.len() {
        play_pause(&players[selected_line]);
    } else {
        let diff = selected_line - players.len();
        // println!("diff {:?}", diff);
        match diff {
            0 => print!("meeeeh"),
            1 => pause_all(&players),
            2 => next_track(&players),
            3 => prev_track(&players),
            4 => exit(0),
            _ => (),
        }
    }
}
fn main() {
    let looping = match env::args()
        .last()
        .expect("No command line arguments found!")
        .as_str()
    {
        "continue" => true,
        "help" | "-h" | "--help" => {
            let programm_name = env::args()
                .next()
                .expect("No command line arguments found!");
            println!("Usage: {programm_name} [OPTIONS] \n");
            println!("Options:");
            println!("      continue        : loops, until 'QUIT' is selected");
            println!("      help            : prints this page");
            exit(0);
        }
        _ => false,
    };

    show_rofi(looping);
    while looping {
        show_rofi(looping);
    }
}
