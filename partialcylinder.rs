use bevy::{
    prelude::*,
    render::{
        mesh::{Extrudable, PerimeterSegment},
        render_asset::RenderAssetUsages,
    }
};

#[derive(Copy, Clone)]
pub struct PartialCylinder {
    pub radius: f32,
    pub angle: u32
}

impl Primitive2d for PartialCylinder {}

impl Primitive3d for PartialCylinder {}

impl Default for PartialCylinder {
    fn default() -> Self {
        Self {
            radius: 1.0,
            angle: 45
        }
    }
}

impl PartialCylinder {
    #[inline(always)]
    pub const fn new(radius: f32, angle: u32) -> Self {
        Self {
            radius,
            angle
        }
    }
}

pub struct PartialCylinderMeshBuilder {
    partialcylinder: PartialCylinder
}

impl Default for PartialCylinderMeshBuilder {
    fn default() -> Self {
        Self {
            partialcylinder: PartialCylinder::default()
        }
    }
}

impl Meshable for PartialCylinder {
    type Output = PartialCylinderMeshBuilder;

    fn mesh(&self) -> Self::Output {
        Self::Output {
            partialcylinder: *self,
            ..Default::default()
        }
    }
}

impl MeshBuilder for PartialCylinderMeshBuilder {
    fn build(&self) -> Mesh {
        let num_vertices = self.partialcylinder.angle * 3;

        let mut positions = Vec::with_capacity(num_vertices as usize);
        let mut uvs = Vec::with_capacity(num_vertices as usize);
        let mut indices = Vec::with_capacity((6 * num_vertices - 9) as usize);

        let step_theta = core::f32::consts::TAU / 360 as f32;
        let mut sectors_produced = 0;

        positions.push([0.0; 3]);
        uvs.push([0.5, 0.5]);

        for i in 0..self.partialcylinder.angle {
            let theta = i as f32 * step_theta;
            let (sin, cos) = ops::sin_cos(theta);
            positions.push([self.partialcylinder.radius * cos, self.partialcylinder.radius * sin, 0.0]);
            uvs.push([0.5 - (cos + 1.0) / 4., 0.5 - sin / 2.]);
            sectors_produced += 1;
        }

        for i in 0..sectors_produced as u32 {
            indices.extend_from_slice(&[0, i, i+1]);
        }

        let mut result = Mesh::new(
            bevy::render::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)     
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        
        result.compute_normals();
        result
    }
}

impl Extrudable for PartialCylinderMeshBuilder {
    fn perimeter(&self) -> Vec<PerimeterSegment> {
        let resolution = self.partialcylinder.angle as u32 + 1;
        vec![
            PerimeterSegment::Smooth {
                first_normal: Vec2::X,
                last_normal: Vec2::NEG_X,
                indices: (0..resolution).chain([0]).collect(),
            }
        ]
    }
}
