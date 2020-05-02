use lyon::lyon_algorithms::path::Path;
use ruffle_core::shape_utils::DrawCommand;
use ruffle_core::swf;
use swf::Twips;
macro_rules! create_debug_label {
    ($($arg:tt)*) => (
        if cfg!(feature = "render_debug_labels") {
            Some(format!($($arg)*))
        } else {
            None
        }
    )
}

pub fn create_buffer_with_data(
    device: &wgpu::Device,
    data: &[u8],
    usage: wgpu::BufferUsage,
    label: Option<String>,
) -> wgpu::Buffer {
    let mapped = device.create_buffer_mapped(&wgpu::BufferDescriptor {
        size: data.len() as u64,
        usage,
        label: label.as_deref(),
    });
    mapped.data.copy_from_slice(data);
    mapped.finish()
}

pub fn point(x: Twips, y: Twips) -> lyon::math::Point {
    lyon::math::Point::new(x.to_pixels() as f32, y.to_pixels() as f32)
}

pub fn ruffle_path_to_lyon_path(commands: Vec<DrawCommand>, is_closed: bool) -> Path {
    let mut builder = Path::builder();
    for cmd in commands {
        match cmd {
            DrawCommand::MoveTo { x, y } => {
                builder.move_to(point(x, y));
            }
            DrawCommand::LineTo { x, y } => {
                builder.line_to(point(x, y));
            }
            DrawCommand::CurveTo { x1, y1, x2, y2 } => {
                builder.quadratic_bezier_to(point(x1, y1), point(x2, y2));
            }
        }
    }

    if is_closed {
        builder.close();
    }

    builder.build()
}

#[allow(clippy::many_single_char_names)]
pub fn swf_to_gl_matrix(m: swf::Matrix) -> [[f32; 4]; 4] {
    let tx = m.translate_x.get() as f32;
    let ty = m.translate_y.get() as f32;
    let det = m.scale_x * m.scale_y - m.rotate_skew_1 * m.rotate_skew_0;
    let mut a = m.scale_y / det;
    let mut b = -m.rotate_skew_1 / det;
    let mut c = -(tx * m.scale_y - m.rotate_skew_1 * ty) / det;
    let mut d = -m.rotate_skew_0 / det;
    let mut e = m.scale_x / det;
    let mut f = (tx * m.rotate_skew_0 - m.scale_x * ty) / det;

    a *= 20.0 / 32768.0;
    b *= 20.0 / 32768.0;
    d *= 20.0 / 32768.0;
    e *= 20.0 / 32768.0;

    c /= 32768.0;
    f /= 32768.0;
    c += 0.5;
    f += 0.5;
    [
        [a, d, 0.0, 0.0],
        [b, e, 0., 0.0],
        [c, f, 1.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ]
}

#[allow(clippy::many_single_char_names)]
pub fn swf_bitmap_to_gl_matrix(
    m: swf::Matrix,
    bitmap_width: u32,
    bitmap_height: u32,
) -> [[f32; 4]; 4] {
    let bitmap_width = bitmap_width as f32;
    let bitmap_height = bitmap_height as f32;

    let tx = m.translate_x.get() as f32;
    let ty = m.translate_y.get() as f32;
    let det = m.scale_x * m.scale_y - m.rotate_skew_1 * m.rotate_skew_0;
    let mut a = m.scale_y / det;
    let mut b = -m.rotate_skew_1 / det;
    let mut c = -(tx * m.scale_y - m.rotate_skew_1 * ty) / det;
    let mut d = -m.rotate_skew_0 / det;
    let mut e = m.scale_x / det;
    let mut f = (tx * m.rotate_skew_0 - m.scale_x * ty) / det;

    a *= 20.0 / bitmap_width;
    b *= 20.0 / bitmap_width;
    d *= 20.0 / bitmap_height;
    e *= 20.0 / bitmap_height;

    c /= bitmap_width;
    f /= bitmap_height;

    [
        [a, d, 0.0, 0.0],
        [b, e, 0.0, 0.0],
        [c, f, 1.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ]
}
