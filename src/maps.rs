use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Default)]
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
    pub collision_config: Lazy<HashMap<&'static str, Vec<CollisionInstance>>>,
}

pub static MAPS: [RLMap; 2] = [
    RLMap {
        upk_file_name: "EuroStadium_Night_P.upk",
        out_folder_name: "soccar",
        collision_config: Lazy::new(|| {
            HashMap::from([
                (
                    "Goal_STD_Collision_Half_Goal_STD_Collision",
                    vec![
                        CollisionInstance::new(5120., [-1., 1., 1.]),
                        CollisionInstance::new(5120., [1., 1., 1.]),
                        CollisionInstance::new(-5120., [1., -1., 1.]),
                        CollisionInstance::new(-5120., [-1., -1., 1.]),
                    ],
                ),
                (
                    "Field_STD_Collision_SideBot_Half",
                    vec![
                        CollisionInstance::new(0., [1., 1., 1.]),
                        CollisionInstance::new(0., [1., -1., 1.]),
                        CollisionInstance::new(0., [-1., 1., 1.]),
                        CollisionInstance::new(0., [-1., -1., 1.]),
                    ],
                ),
                (
                    "Field_STD_Collision_SideTop_Half",
                    vec![
                        CollisionInstance::new(0., [1., 1., 1.]),
                        CollisionInstance::new(0., [-1., -1., 1.]),
                        CollisionInstance::new(0., [1., -1., 1.]),
                        CollisionInstance::new(0., [-1., 1., 1.]),
                    ],
                ),
                (
                    "Field_STD_Collision_Corner",
                    vec![
                        CollisionInstance::new(0., [1., -1., 1.]),
                        CollisionInstance::new(0., [-1., -1., 1.]),
                        CollisionInstance::new(0., [1., 1., 1.]),
                        CollisionInstance::new(0., [-1., 1., 1.]),
                    ],
                ),
            ])
        }),
    },
    RLMap {
        upk_file_name: "HoopsStadium_P.upk",
        out_folder_name: "hoops",
        collision_config: Lazy::new(|| {
            HashMap::from([
                (
                    "Net_Rim",
                    vec![
                        CollisionInstance::new(432., [-0.9, -0.9, 0.9]),
                        CollisionInstance::new(-432., [0.9, 0.9, 0.9]),
                    ],
                ),
                (
                    "Net_Collision",
                    vec![
                        CollisionInstance::new(432., [-0.9, -0.9, 0.9]),
                        CollisionInstance::new(-432., [0.9, 0.9, 0.9]),
                    ],
                ),
                (
                    "SideRamps01",
                    vec![
                        CollisionInstance::new(0., [-1., 1., 1.]),
                        CollisionInstance::new(0., [1., 1., 1.]),
                    ],
                ),
                (
                    "SideRamps02",
                    vec![
                        CollisionInstance::new(0., [1., -1., 1.]),
                        CollisionInstance::new(0., [1., 1., 1.]),
                    ],
                ),
                (
                    "CornerPiece01",
                    vec![
                        CollisionInstance::new(0., [1., -1., 1.]),
                        CollisionInstance::new(0., [-1., -1., 1.]),
                        CollisionInstance::new(0., [-1., 1., 1.]),
                        CollisionInstance::new(0., [1., 1., 1.]),
                    ],
                ),
            ])
        }),
    },
];
