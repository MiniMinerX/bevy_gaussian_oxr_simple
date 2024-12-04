// Gaussian setup module

use bevy::{prelude::*, render::view::RenderLayers};
use bevy_gaussian_splatting::{GaussianCloudSettings, GaussianSplattingBundle, GaussianSplattingPlugin};

#[derive(Component)]
pub struct GaussianMarker {
    pub prev_visibility: Visibility,
}

