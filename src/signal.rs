use std::io::BufReader;
use std::process::Command;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::env;
use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use std::str;

fn main() -> std::io::Result<()>  {

    //get pid of waybar
    let pidof = Command::new("pidof")
        .arg("waybar")
        .output()
        .expect("Error: pidof didnt work!");
    
    let pidof = pidof.stdout;
    let pidof = str::from_utf8(&pidof)
        .expect("Error: utf8")
        .trim();
    let pid:i32 = pidof.parse().unwrap();
    

    //get hyprland sig
    let hyprland_sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .expect("Error: retrieving env var");
    let path = format!("/tmp/hypr/{hyprland_sig}/.socket2.sock");


    //open socket
    let stream = UnixStream::connect(&path)?;
    let mut buf_reader = BufReader::new(stream);
    
    
    loop {

        let mut vec_buffer = String::new();
        buf_reader.read_line(&mut vec_buffer)?;

        let event = vec_buffer
            .as_str()
            .split('>')
            .next()
            .expect("Error: readline couldn't parse");

    
        match event {
            "workspace" => (),
            "openwindow" => (),
            "closewindow" => (),
            _ => continue,
        }

        let signal: Signal = unsafe { std::mem::transmute(libc::SIGRTMIN() + 8) };
        kill(Pid::from_raw(pid), signal)
            .expect("Error: sending signal");
    }
}