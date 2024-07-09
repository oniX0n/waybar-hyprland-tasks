use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::env;


fn main() -> std::io::Result<()> {

    //arg handling
    let mut arg_w = false;

    let args: Vec<String> = env::args().collect();
    for arg in args {
        match arg.as_str() {
            "-w" => arg_w = true,
            _ => continue
        }
    }




    let hyprland_sig = env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .expect("Error: retrieving env var");
    let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR")
        .expect("Error: retrieving env var");

    let path = format!("{xdg_runtime_dir}/hypr/{hyprland_sig}/.socket.sock");
    

    //workspace -w
    let mut workspace: i32 = 0;
    if arg_w {
        let mut stream = UnixStream::connect(&path)?;
        stream.write_all(b"j/activeworkspace")?;
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        
        let parsed = json::parse(&response)
            .expect("Error: parsing json");

        let workspace_id: i32 = parsed["id"].as_i32()
            .expect("Error: parsing json to i32");
        
        workspace = workspace_id;
    }

    
    //getting tasks
    let mut stream = UnixStream::connect(&path)?;
    stream.write_all(b"j/clients")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    let parsed = json::parse(&response)
        .expect("Error: parsing json");


    let mut clients = String::from("");

    for n in 0..=parsed.len()-1 {

        let class: &str = parsed[n]["class"].as_str().
            expect("Error: parsing json to String");

        if class.is_empty() {
            continue;
        }

        let workspace_id: i32 = parsed[n]["workspace"]["id"].as_i32().
            expect("Error: parsing json to i32");
        
        if arg_w && workspace_id != workspace {
            continue;
        }
        clients.push_str(class);
        clients.push_str(" | ");
        
    }
    
    if !clients.is_empty() {
        clients.pop();
        clients.pop();
        clients.pop();
    }
    clients.push('\n');
    print!("{}", clients);

    Ok(())
}
