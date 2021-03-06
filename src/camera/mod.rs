pub mod builder;
pub mod orthographic;
pub mod perspective;
pub mod projective;

use crate::{film::Film, sampler::Samplers};
use quick_maths::{Ray3, Transform4, Vec2};
use std::fmt::Debug;

pub trait Camera: Debug {
  /// Sample a ray from the camera using the given uv in [0,1]^2.
  fn sample_ray(&self, sample_pos: Vec2) -> Ray3;
}

/// Common struct for all cameras
#[derive(Debug)]
pub struct Cameras {
  /// local space --> world space
  to_world: Transform4,

  /// world space --> local space
  from_world: Transform4,

  /// The film for this camera
  film: Film,

  /// Sampler for this camera
  sampler: Samplers,

  /// Which specific version of this camera is it
  variant: Variant,
}

/// Represents one variant of a camera
#[derive(Debug)]
pub enum Variant {
  Perspective(perspective::Perspective),
  Orthographic(orthographic::Orthographic),
  // TODO add camera instances here
}

impl Cameras {
  /// Returns the film for this camera
  pub fn film(&self) -> &Film { &self.film }
  /// Returns the sampler used
  pub fn sampler(&self) -> &Samplers { &self.sampler }
}

impl Camera for Cameras {
  fn sample_ray(&self, sample_pos: Vec2) -> Ray3 {
    use Variant::*;
    let local_ray = match &self.variant {
      Perspective(c) => c.sample_ray(sample_pos),
      Orthographic(o) => o.sample_ray(sample_pos),
    };
    self.to_world.apply_ray(&local_ray)
  }
}
