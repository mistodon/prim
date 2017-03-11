pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];

pub type Position = Vector3;
pub type Normal = Vector3;
pub type UV = Vector2;


pub trait YesNo {}

#[derive(Default)]
pub struct Yes;

#[derive(Default)]
pub struct No;

impl YesNo for Yes {}
impl YesNo for No {}