#[macro_export]
macro_rules! include_prefixed_wgsl {
  ($source_path:expr, $prefix:expr) => {{
    use wgpu::{include_wgsl, ShaderModuleDescriptor, ShaderSource};
    ShaderModuleDescriptor {
      label: None,
      source: wgpu::ShaderSource::Wgsl(
        if let ShaderSource::Wgsl(source) = include_wgsl!($source_path).source {
          ($prefix + &source).into()
        } else {
          unreachable!()
        },
      ),
    }
  }};
}

#[macro_export]
macro_rules! wgsl_const_string {
  ($x:ident : $t:ident) => {
    format!("const {}: {} = {};", stringify!($x), stringify!($t), $x)
  };
}

#[macro_export]
macro_rules! wgsl_const_strings {
  ($x:ident : $t:ident $(,)?) => {{
    use crate::wgsl_const_string;
    wgsl_const_string!($x : $t)
  }};

  ($x:ident : $t:ident, $($rest:tt)+) => {{
    use crate::wgsl_const_string;
    format!("{}\n{}",
      wgsl_const_string!($x : $t),
      wgsl_const_strings!($($rest)+)
    )
  }};
}
