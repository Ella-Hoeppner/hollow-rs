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
macro_rules! wgsl_constant_string {
  ($x:ident : $t:ident) => {
    format!("const {}: {} = {};", stringify!($x), stringify!($t), $x)
  };
}

#[macro_export]
macro_rules! wgsl_constants_string {
  ($x:ident : $t:ident $(,)?) => {{
    use $crate::wgsl_constant_string;
    wgsl_constant_string!($x : $t)
  }};

  ($x:ident : $t:ident, $($rest:tt)+) => {{
    use $crate::wgsl_constant_string;
    format!("{}\n{}",
      wgsl_constant_string!($x : $t),
      wgsl_constants_string!($($rest)+)
    )
  }};
}
