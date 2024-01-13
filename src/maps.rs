use combo_vec::{re_arr, ReArr};
use phf::phf_map;

#[derive(Clone, Copy, Debug, Default)]
pub struct CollisionInstance {
    pub translate: f32,
    pub scale: [f32; 3],
}

impl CollisionInstance {
    #[must_use]
    pub const fn new(translate: f32, scale: [f32; 3]) -> Self {
        Self { translate, scale }
    }
}

pub struct RLMap {
    pub upk_file_name: &'static str,
    pub out_folder_name: &'static str,
    pub collision_config: phf::Map<&'static str, ReArr<CollisionInstance, 4>>,
}

pub static MAPS: [RLMap; 2] = [
    RLMap {
        upk_file_name: "EuroStadium_Night_P.upk",
        out_folder_name: "soccar",
        collision_config: phf_map! {
            "Goal_STD_Collision_Half_Goal_STD_Collision" => re_arr![
                CollisionInstance::new(5120., [-1., 1., 1.]),
                CollisionInstance::new(5120., [1., 1., 1.]),
                CollisionInstance::new(-5120., [1., -1., 1.]),
                CollisionInstance::new(-5120., [-1., -1., 1.]),
            ],
            "Field_STD_Collision_SideBot_Half" => re_arr![
                CollisionInstance::new(0., [1., 1., 1.]),
                CollisionInstance::new(0., [1., -1., 1.]),
                CollisionInstance::new(0., [-1., 1., 1.]),
                CollisionInstance::new(0., [-1., -1., 1.]),
            ],
            "Field_STD_Collision_SideTop_Half" => re_arr![
                CollisionInstance::new(0., [1., 1., 1.]),
                CollisionInstance::new(0., [-1., -1., 1.]),
                CollisionInstance::new(0., [1., -1., 1.]),
                CollisionInstance::new(0., [-1., 1., 1.]),
            ],
            "Field_STD_Collision_Corner" => re_arr![
                CollisionInstance::new(0., [1., -1., 1.]),
                CollisionInstance::new(0., [-1., -1., 1.]),
                CollisionInstance::new(0., [1., 1., 1.]),
                CollisionInstance::new(0., [-1., 1., 1.]),
            ],
        },
    },
    RLMap {
        upk_file_name: "HoopsStadium_P.upk",
        out_folder_name: "hoops",
        collision_config: phf_map! {
            "Net_Rim" => re_arr![
                CollisionInstance::new(432., [-0.9, -0.9, 0.9]),
                CollisionInstance::new(-432., [0.9, 0.9, 0.9]);
                None,
                None,
            ],
            "Net_Collision" => re_arr![
                CollisionInstance::new(432., [-0.9, -0.9, 0.9]),
                CollisionInstance::new(-432., [0.9, 0.9, 0.9]);
                None,
                None,
            ],
            "SideRamps01" => re_arr![
                CollisionInstance::new(0., [-1., 1., 1.]),
                CollisionInstance::new(0., [1., 1., 1.]);
                None,
                None,
            ],
            "SideRamps02" => re_arr![
                CollisionInstance::new(0., [1., -1., 1.]),
                CollisionInstance::new(0., [1., 1., 1.]);
                None,
                None,
            ],
            "CornerPiece01" => re_arr![
                CollisionInstance::new(0., [1., -1., 1.]),
                CollisionInstance::new(0., [-1., -1., 1.]),
                CollisionInstance::new(0., [-1., 1., 1.]),
                CollisionInstance::new(0., [1., 1., 1.]),
            ],
        },
    },
];
