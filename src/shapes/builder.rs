use super::{
  triangle_list::{from_ascii_obj, from_ascii_stl},
  Geometry,
};
use quick_maths::Vec3;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Builder {
  pub to_world: crate::transform::Builder,
  pub variant: Variant,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Variant {
  Sphere {
    center: Vec3,
    radius: f32,
  },
  Plane {
    normal: Vec3,
    w: f32,
    up: Vec3,
    width: f32,
    height: f32,
  },
  Triangle(Vec3<Vec3>),
  Obj {
    file: String,
    use_mtls: Option<bool>,
    binary: Option<bool>,
  },
  Stl {
    file: String,
    binary: Option<bool>,
  },
}

impl From<Builder> for Geometry {
  fn from(b: Builder) -> Self {
    let Builder { to_world, variant } = b;
    use super::Variant as GeoVariant;
    use Variant::*;
    let variant = match variant {
      Sphere { center, radius } => GeoVariant::Sphere(super::sphere::Sphere::new(center, radius)),
      Plane {
        normal,
        w,
        up,
        width,
        height,
      } => GeoVariant::Plane(super::plane::Plane::new(&normal, w, &up, width, height)),
      Triangle(verts) => GeoVariant::Triangle(super::triangle::Triangle(verts)),
      Obj {
        file,
        use_mtls,
        binary,
      } => {
        let use_mtls = use_mtls.unwrap_or(false);
        let binary = binary.unwrap_or(false);
        let triangle_list = if binary {
          todo!("Cannot handle binary objs yet");
        } else {
          from_ascii_obj(file, use_mtls).expect("Failed to read input OBJ file")
        };
        GeoVariant::TriangleList(triangle_list)
      },
      Stl { file, binary } => {
        let binary = binary.unwrap_or(false);
        let triangle_list = if binary {
          todo!("Cannot handle binary stls yet");
        } else {
          from_ascii_stl(file).expect("Failed to read input STL file")
        };
        GeoVariant::TriangleList(triangle_list)
      },
    };
    Self {
      to_world: to_world.into(),
      variant,
    }
  }
}
