use byteorder::{LittleEndian, ReadBytesExt};
use std::{
    fs,
    io::{self, Write},
};
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct MeshBuilder {
    ids: Vec<usize>,
    verts: Vec<[f32; 3]>,
}

impl MeshBuilder {
    #[must_use]
    /// Combine different meshes all into one
    pub fn combine(other_meshes: Vec<Self>) -> Self {
        let (n_ids, n_verts) = other_meshes
            .iter()
            .fold((0, 0), |(n_ids, n_verts), m| (n_ids + m.ids.len(), n_verts + m.verts.len()));
        let mut id_offset = 0;

        let (ids, verts) = other_meshes.into_iter().fold(
            (Vec::with_capacity(n_ids), Vec::with_capacity(n_verts)),
            |(mut ids, mut verts), m| {
                ids.extend(m.ids.into_iter().map(|id| id + id_offset));
                id_offset += m.verts.len();
                verts.extend(m.verts);
                (ids, verts)
            },
        );

        Self { ids, verts }
    }
}

const GAMEMODE: [&str; 2] = ["soccar", "hoops"];

fn main() {
    for game_mode in GAMEMODE {
        let mut meshes = Vec::new();

        for file in WalkDir::new(format!("collision_meshes/{game_mode}")).into_iter().flatten() {
            if file.file_type().is_dir() {
                continue;
            }

            let file = file.path();
            let mut bytes = io::Cursor::new(fs::read(file).unwrap());

            let num_ids = bytes.read_i32::<LittleEndian>().unwrap() * 3;
            let num_verts = bytes.read_i32::<LittleEndian>().unwrap();

            let mut ids = Vec::new();
            let mut verts = Vec::new();

            ids.reserve(num_ids as usize);
            for _ in 0..num_ids {
                ids.push(bytes.read_i32::<LittleEndian>().unwrap() as usize);
            }

            verts.reserve(num_verts as usize);
            for _ in 0..num_verts {
                verts.push([
                    bytes.read_f32::<LittleEndian>().unwrap() / 50.,
                    bytes.read_f32::<LittleEndian>().unwrap() / 50.,
                    bytes.read_f32::<LittleEndian>().unwrap() / 50.,
                ]);
            }

            meshes.push(MeshBuilder { ids, verts });
        }

        let mesh = MeshBuilder::combine(meshes);

        let mut out_file = fs::File::create(format!("collision_meshes/{game_mode}.obj")).unwrap();

        for vert in mesh.verts {
            writeln!(out_file, "v {} {} {}", vert[0], -vert[2], vert[1]).unwrap();
        }

        for id in mesh.ids.chunks(3) {
            writeln!(out_file, "f {} {} {}", id[0] + 1, id[1] + 1, id[2] + 1).unwrap();
        }
    }
}
