#![forbid(unsafe_code)]
#![warn(clippy::all)]

mod converter;
mod maps;

use converter::MeshBuilder;
use walkdir::WalkDir;
use maps::{RLMap, MAPS};
use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

const OUT_DIR: &str = "./assets/";
const UMODEL: &str = if cfg!(windows) { "umodel.exe" } else { "./umodel" };

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

    let input_dir = if let Some(input_dir) = get_input_dir() {
        input_dir
    } else {
        println!("PLEASE ENTER the absolute path to your 'rocketleague/TAGame/CookedPCConsole' folder:");
        let mut assets_dir = String::new();
        io::stdin().read_line(&mut assets_dir)?;
        assets_dir.pop();

        let assets_path = Path::new(&assets_dir);
        assert!(
            assets_path.is_dir() && assets_path.exists(),
            "Couldn't find the directory to Rocket League specified!"
        );

        println!("Saving assets path to 'assets.path' file...");
        fs::write("assets.path", &assets_dir)?;
        assets_dir
    };

    let umodel = Path::new(UMODEL);
    assert!(umodel.exists(), "Couldn't find umodel executable!");

    for map in &MAPS {
        print!("Processing {} from Rocket League...", map.upk_file_name);
        io::stdout().flush()?;

        // call umodel to uncook all the map files
        Command::new(umodel)
            .args([
                &format!("-path={input_dir}"),
                &format!("-out={OUT_DIR}"),
                "-game=rocketleague",
                "-export",
                "-nooverwrite",
                "-nolightmap",
                "-notex",
                "-uncook",
                map.upk_file_name,
            ])
            .stdout(Stdio::null())
            .spawn()?
            .wait()?;

        println!(" done.");
    }

    Ok(())
}

fn format_meshes(map: &RLMap, meshes: &Path) -> io::Result<()> {
    let out_folder = Path::new("collision_meshes").join(map.out_folder_name);

    // create the output folder if it doesn't exist
    if !out_folder.exists() {
        fs::create_dir_all(&out_folder)?;
    }

    // get the uncooked pskx files from Rocket League
    let file_paths = WalkDir::new(meshes.display().to_string()).into_iter().flatten();

    let mut i = 0;
    for path in file_paths {
        if path.file_type().is_dir() {
            continue;
        }

        let path = path.path();
        let name = path.file_stem().unwrap().to_string_lossy().to_string();

        let Some(collisions) = map.collision_config.get(name.as_str()) else {
            continue;
        };

        let builder = MeshBuilder::from_pskx(&fs::read(path)?)?;

        for instance in collisions.iter() {
            let bytes = builder.to_cmf_bytes(instance)?;

            let file_name = out_folder.join(format!("mesh_{i}.cmf"));
            fs::write(file_name, bytes)?;
            i += 1;
        }
    }

    Ok(())
}

fn format_maps() -> io::Result<()> {
    let meshes = Path::new(OUT_DIR);

    if meshes.exists() {
        print!("Formatting collision meshes for RocketSim...");
        io::stdout().flush()?;
    } else {
        panic!("Couldn't find collision meshes!")
    }

    for map in &MAPS {
        format_meshes(map, meshes)?;
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
    format_maps()?;
    remove_extra_files()?;

    Ok(())
}
