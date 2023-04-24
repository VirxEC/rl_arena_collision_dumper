mod converter;

use std::{
    collections::HashMap,
    fs::{self, read_dir},
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

use converter::MeshBuilder;

const OUT_DIR: &str = "./assets/";

fn get_input_dir() -> Option<String> {
    let Ok(input_file) = fs::read_to_string("assets.path") else {
        println!("Couldn't find 'assets.path' file in your base folder!");
        return None;
    };

    let Some(assets_dir) = input_file.lines().next() else {
        println!("Your 'assets.path' file is empty!");
        return None;
    };

    let assets_path = Path::new(assets_dir);
    if assets_path.is_dir() && assets_path.exists() {
        Some(assets_dir.to_string())
    } else {
        println!("Couldn't find the directory to Rocket League specified!");
        None
    }
}

fn uncook() -> io::Result<()> {
    if Path::new(OUT_DIR).exists() {
        println!("Found existing assets");
        return Ok(());
    }

    let input_dir = match get_input_dir() {
        Some(input_dir) => input_dir,
        None => {
            println!("PLEASE ENTER the absolute path to your 'rocketleague/TAGame/CookedPCConsole' folder:");
            let mut assets_dir = String::new();
            io::stdin().read_line(&mut assets_dir)?;
            assets_dir.pop();

            let assets_path = Path::new(&assets_dir);
            if !assets_path.is_dir() || !assets_path.exists() {
                panic!("Couldn't find the directory to Rocket League specified!");
            }

            println!("Saving assets path to 'assets.path' file...");
            fs::write("assets.path", &assets_dir)?;
            assets_dir
        }
    };

    print!("Processing Stadium_P.upk from Rocket League...");
    io::stdout().flush()?;

    // call umodel to uncook all the map files
    Command::new(if cfg!(windows) { "umodel.exe" } else { "./umodel" })
        .args([
            &format!("-path={input_dir}"),
            &format!("-out={OUT_DIR}"),
            "-game=rocketleague",
            "-export",
            "-nooverwrite",
            "-nolightmap",
            "-notex",
            "-uncook",
            "Stadium_P.upk",
        ])
        .stdout(Stdio::null())
        .spawn()?
        .wait()?;

    println!(" done.");

    Ok(())
}

type CollisionInstances = Vec<([f32; 3], f32)>;
fn read_collision_cfg() -> io::Result<HashMap<String, CollisionInstances>> {
    print!("Reading collision config...");
    io::stdout().flush()?;

    let mut collision_cfg = HashMap::new();

    let file = fs::read_to_string("collision.cfg")?;
    for line in file.lines() {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap().to_string();
        let x = split.next().unwrap().parse::<f32>().unwrap();
        let y = split.next().unwrap().parse::<f32>().unwrap();
        let z = split.next().unwrap().parse::<f32>().unwrap();
        let y_offset = split.next().map(|s| s.parse::<f32>().unwrap()).unwrap_or_default();

        collision_cfg.entry(name).or_insert_with(Vec::new).push(([x, y, z], y_offset));
    }

    println!(" done.");

    Ok(collision_cfg)
}

fn format_collision_meshes() -> io::Result<()> {
    let collision_cfg = read_collision_cfg()?;

    let meshes = Path::new(OUT_DIR).join("FieldCollision_Standard").join("StaticMesh3");

    if meshes.exists() {
        print!("Formatting collision meshes for RocketSim...");
        io::stdout().flush()?;
    } else {
        panic!("Couldn't find collision meshes!")
    }

    let out_folder = Path::new("collision_meshes").join("soccar");

    // create the output folder if it doesn't exist
    if !out_folder.exists() {
        fs::create_dir_all(&out_folder)?;
    }

    // get the uncooked pskx files from Rocket League
    let file_paths = read_dir(meshes)?
        .flatten()
        .filter(|entry| entry.path().extension().unwrap_or_default() == "pskx")
        .map(|entry| entry.path());

    let mut i = 0;
    for path in file_paths {
        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let builder = MeshBuilder::from_pskx(&fs::read(path)?)?;

        let Some(collisions) = collision_cfg.get(&name) else {
            continue;
        };

        for (scale, y_offset) in collisions {
            let bytes = builder.to_cmf_bytes(scale, *y_offset);

            let file_name = out_folder.join(format!("mesh_{i}.cmf"));
            fs::write(file_name, bytes)?;
            i += 1;
        }
    }

    println!(" done.");

    Ok(())
}

fn remove_extra_files() -> io::Result<()> {
    print!("Removing extra files...");
    io::stdout().flush()?;
    fs::remove_dir_all(OUT_DIR)?;
    println!(" done.");

    Ok(())
}

fn main() -> io::Result<()> {
    uncook()?;
    format_collision_meshes()?;
    remove_extra_files()?;

    Ok(())
}
