use std::{
    io::{self, Read},
    str::from_utf8,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Clone, Debug, Default)]
pub struct MeshBuilder {
    ids: Vec<i32>,
    verts: Vec<f32>,
}

impl MeshBuilder {
    const PSK_FILE_HEADER: &[u8] = b"ACTRHEAD\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    /// Write mesh to a .cmf file
    ///
    /// Structure:
    /// - i32: number of triangles (ids.len() / 3)
    /// - i32: number of vertices (verts.len() / 3)
    /// - the triangles
    /// - the vertices
    pub fn to_cmf_bytes(&self, scale: &[f32; 3], y_offset: f32) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + 4 * self.ids.len() + 4 * self.verts.len());

        bytes.write_i32::<LittleEndian>(self.ids.len() as i32 / 3).unwrap();
        bytes.write_i32::<LittleEndian>(self.verts.len() as i32 / 3).unwrap();

        for id in &self.ids {
            bytes.write_i32::<LittleEndian>(*id).unwrap();
        }

        for vert in self.verts.chunks_exact(3) {
            bytes.write_f32::<LittleEndian>(vert[0] / 50. * scale[0]).unwrap();
            bytes.write_f32::<LittleEndian>(vert[1] / 50. * scale[1] - y_offset).unwrap();
            bytes.write_f32::<LittleEndian>(vert[2] / 50. * scale[2]).unwrap();
        }

        bytes
    }

    /// Create a mesh from a Rocket League .pskx file
    pub fn from_pskx(bytes: &[u8]) -> io::Result<Self> {
        let mut cursor = io::Cursor::new(bytes);

        // ensure file header matches PSK_FILE_HEADER
        let mut file_header = [0; 32];
        cursor.read_exact(&mut file_header)?;
        assert_eq!(&file_header[..Self::PSK_FILE_HEADER.len()], Self::PSK_FILE_HEADER);

        let mut ids = Vec::new();
        let mut verts = Vec::new();

        let mut wedges = Vec::new();

        // read chunks
        loop {
            let mut chunk_header = [0; 32];
            if cursor.read_exact(&mut chunk_header).is_err() {
                break;
            }

            let chunk_id = from_utf8(&chunk_header[0..8]).unwrap();
            // let chunk_type = i32::from_le_bytes([chunk_header[20], chunk_header[21], chunk_header[22], chunk_header[23]]);
            let chunk_data_size = i32::from_le_bytes([chunk_header[24], chunk_header[25], chunk_header[26], chunk_header[27]]) as usize;
            let chunk_data_count = i32::from_le_bytes([chunk_header[28], chunk_header[29], chunk_header[30], chunk_header[31]]) as usize;

            if chunk_data_count == 0 {
                continue;
            }

            let mut chunk_data = vec![0; chunk_data_size * chunk_data_count];
            cursor.read_exact(&mut chunk_data)?;

            match chunk_id {
                "PNTS0000" => {
                    verts = read_vertices(&chunk_data, chunk_data_count);
                    assert_eq!(verts.len() / 3, chunk_data_count);
                    assert_eq!(verts.len() % 3, 0);
                }
                "VTXW0000" => {
                    wedges = read_wedges(&chunk_data, chunk_data_count);
                    assert_eq!(wedges.len(), chunk_data_count);
                }
                "FACE0000" => {
                    ids.extend(read_faces(&chunk_data, chunk_data_count, &wedges).into_iter().flatten().map(|(id, _, _)| id as i32));
                    assert_eq!(ids.len() / 3, chunk_data_count);
                }
                _ => {}
            }
        }

        assert_eq!(verts.len() / 3, *ids.iter().max().unwrap() as usize + 1);

        Ok(Self { ids, verts })
    }
}

pub fn read_vertices(chunk_data: &[u8], data_count: usize) -> Vec<f32> {
    let mut vertices = Vec::with_capacity(data_count);

    let mut reader = io::Cursor::new(chunk_data);
    for _ in 0..data_count * 3 {
        vertices.push(reader.read_f32::<LittleEndian>().unwrap());
    }

    vertices
}

#[derive(Clone, Copy, Debug)]
pub struct Wedge {
    pub vertex_id: u32,
    pub uv: [f32; 2],
    pub material_index: usize,
}

pub fn read_wedges(chunk_data: &[u8], data_count: usize) -> Vec<Wedge> {
    let mut wedges = Vec::with_capacity(data_count);

    let mut reader = io::Cursor::new(chunk_data);
    for _ in 0..data_count {
        let vertex_id = reader.read_u32::<LittleEndian>().unwrap();
        let u = reader.read_f32::<LittleEndian>().unwrap();
        let v = reader.read_f32::<LittleEndian>().unwrap();
        let material_index = reader.read_u8().unwrap() as usize;
        wedges.push(Wedge {
            vertex_id,
            uv: [u, v],
            material_index,
        });

        // read padding bytes
        reader.read_u8().unwrap();
        reader.read_u8().unwrap();
        reader.read_u8().unwrap();
    }

    wedges
}

pub fn read_faces(chunk_data: &[u8], data_count: usize, wedges: &[Wedge]) -> Vec<[(u32, [f32; 2], usize); 3]> {
    let mut faces = Vec::with_capacity(data_count * 3);

    let mut reader = io::Cursor::new(chunk_data);
    for _ in 0..data_count {
        let wdg_idx_1 = reader.read_u16::<LittleEndian>().unwrap() as usize;
        let wdg_idx_2 = reader.read_u16::<LittleEndian>().unwrap() as usize;
        let wdg_idx_3 = reader.read_u16::<LittleEndian>().unwrap() as usize;
        let _mat_index = reader.read_u8().unwrap();
        let _aux_mat_index = reader.read_u8().unwrap();
        let _smoothing_group = reader.read_u32::<LittleEndian>().unwrap();

        let verts = [wedges[wdg_idx_1], wedges[wdg_idx_2], wedges[wdg_idx_3]];

        faces.push([
            (verts[1].vertex_id, verts[1].uv, verts[1].material_index),
            (verts[0].vertex_id, verts[0].uv, verts[0].material_index),
            (verts[2].vertex_id, verts[2].uv, verts[2].material_index),
        ]);
    }

    faces
}
