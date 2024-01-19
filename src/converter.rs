use crate::maps::CollisionInstance;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{Cursor, Read, Result},
    str::from_utf8,
};

#[derive(Clone, Debug, Default)]
pub struct MeshBuilder {
    ids: Vec<u32>,
    verts: Vec<[f32; 3]>,
}

impl MeshBuilder {
    const PSK_FILE_HEADER: &'static [u8] = b"ACTRHEAD\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    /// Write mesh to 2 .bin files
    /// 
    /// Structure:
    /// - *_ids.bin - the ids as u32s
    /// - *_vertices.bin - the vertices as f32s
    #[cfg(feature = "bin")]
    pub fn to_bin_bytes(&self) -> Result<[Vec<u8>; 2]> {
        let mut ids = Vec::with_capacity(4 * self.ids.len());
        let mut verts = Vec::with_capacity(4 * self.verts.len() * 3);

        for id in self.ids.iter().copied() {
            ids.write_u32::<LittleEndian>(id)?;
        }

        for vert in self.verts.iter().copied() {
            verts.write_f32::<LittleEndian>(vert[0])?;
            verts.write_f32::<LittleEndian>(vert[1])?;
            verts.write_f32::<LittleEndian>(vert[2])?;
        }

        Ok([ids, verts])
    }

    /// Write mesh to a .cmf file
    ///
    /// Structure:
    /// - i32: number of triangles (ids.len() / 3)
    /// - i32: number of vertices (verts.len())
    /// - the triangles
    /// - the vertices
    pub fn to_cmf_bytes(&self, instance: &CollisionInstance) -> Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(8 + 4 * self.ids.len() + 4 * self.verts.len() * 3);

        bytes.write_i32::<LittleEndian>(self.ids.len() as i32 / 3)?;
        bytes.write_i32::<LittleEndian>(self.verts.len() as i32)?;

        // let transform = self.transform(instance.transform);

        for id in self.ids.iter().copied() {
            bytes.write_i32::<LittleEndian>(id as i32)?;
        }

        for vert in self.verts.iter().copied() {
            bytes.write_f32::<LittleEndian>(vert[0] * instance.scale[0])?;
            bytes.write_f32::<LittleEndian>(vert[1] * instance.scale[1] - instance.translate)?;
            bytes.write_f32::<LittleEndian>(vert[2] * instance.scale[2])?;
        }

        Ok(bytes)
    }

    /// Create a mesh from a Rocket League .pskx file
    pub fn from_pskx(bytes: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(bytes);

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

            let chunk_data_size =
                i32::from_le_bytes([chunk_header[24], chunk_header[25], chunk_header[26], chunk_header[27]]) as usize;
            let chunk_data_count =
                i32::from_le_bytes([chunk_header[28], chunk_header[29], chunk_header[30], chunk_header[31]]) as usize;

            if chunk_data_count == 0 {
                continue;
            }

            let mut chunk_data = vec![0; chunk_data_size * chunk_data_count];
            cursor.read_exact(&mut chunk_data)?;

            match chunk_id {
                "PNTS0000" => {
                    verts = read_vertices(&chunk_data, chunk_data_count)?;
                    assert_eq!(verts.len(), chunk_data_count);
                }
                "VTXW0000" => {
                    wedges = read_wedges(&chunk_data, chunk_data_count)?;
                    assert_eq!(wedges.len(), chunk_data_count);
                }
                "FACE0000" => {
                    ids = read_faces(&chunk_data, chunk_data_count, &wedges)?;
                    assert_eq!(ids.len() / 3, chunk_data_count);
                }
                _ => {}
            }
        }

        assert_eq!(verts.len(), *ids.iter().max().unwrap() as usize + 1);

        Ok(Self { ids, verts })
    }
}

pub fn read_vertices(chunk_data: &[u8], data_count: usize) -> Result<Vec<[f32; 3]>> {
    let mut vertices = Vec::with_capacity(data_count);

    let mut reader = Cursor::new(chunk_data);
    for _ in 0..data_count {
        vertices.push([
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
            reader.read_f32::<LittleEndian>()?,
        ]);
    }

    Ok(vertices)
}

pub fn read_wedges(chunk_data: &[u8], data_count: usize) -> Result<Vec<u32>> {
    let mut wedges = Vec::with_capacity(data_count);

    let mut reader = Cursor::new(chunk_data);
    for _ in 0..data_count {
        wedges.push(reader.read_u32::<LittleEndian>()?);

        // skip padding bytes / unused data
        reader.set_position(reader.position() + 12);
    }

    Ok(wedges)
}

pub fn read_faces(chunk_data: &[u8], data_count: usize, wedges: &[u32]) -> Result<Vec<u32>> {
    let mut faces = Vec::with_capacity(data_count * 3);

    let mut reader = Cursor::new(chunk_data);
    for _ in 0..data_count {
        let wedge_indices = [
            reader.read_u16::<LittleEndian>()?,
            reader.read_u16::<LittleEndian>()?,
            reader.read_u16::<LittleEndian>()?,
        ];

        // skip unused data
        reader.set_position(reader.position() + 6);

        faces.extend([
            wedges[wedge_indices[1] as usize],
            wedges[wedge_indices[0] as usize],
            wedges[wedge_indices[2] as usize],
        ]);
    }

    Ok(faces)
}
