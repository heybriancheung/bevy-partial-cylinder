Bevy has many predefined primitives like Cuboid, Cylinder and Annulus. In Bevy's website you can find example of creating custom primitive (https://bevyengine.org/examples/math/custom-primitives/). Despite that, it is difficult to create your own custom primitive due to insufficient documentation explaining the various elements involved in creating it. So here I cordially contribute my work on partial cylinder in Bevy.

Usage:

```
use bevy::prelude::*;

#[path="partialcylinder.rs"]
mod partialcylinder;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ...
) {
    ...
    commands.spawn((
        Mesh3d(meshes.add(Extrusion {
            base_shape: partialcylinder::PartialCylinder::new(/* radius (f32) */, /* angle in degree (u32) */),
            half_depth: 1.0,
        }.mesh())),
        MeshMaterial3d(materials.add(/* the color */))
    ));
    ...
}
```
Screenshot:
<img width="1015" alt="螢幕截圖 2025-06-03 下午2 08 28" src="https://github.com/user-attachments/assets/8ad2ca0f-5e54-4b92-b48b-3c3756416f8a" />
