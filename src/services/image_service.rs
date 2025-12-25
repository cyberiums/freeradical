// Image Service - SEO and Performance Optimization for Images

use std::path::Path;
use image::{imageops::FilterType, DynamicImage, ImageFormat};
use std::io::Cursor;

/// Generate responsive srcset attribute for an image
/// Example: generate_srcset("/images/photo.jpg") returns:
/// "/images/photo-320w.jpg 320w, /images/photo-768w.jpg 768w, /images/photo-1024w.jpg 1024w"
pub fn generate_srcset(base_url: &str) -> String {
    let widths = vec![320, 768, 1024, 1920];
    let path = Path::new(base_url);
    
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("jpg");
    let dir = path.parent().and_then(|p| p.to_str()).unwrap_or("");
    
    widths
        .iter()
        .map(|w| {
            if dir.is_empty() {
                format!("{}-{}w.{} {}w", stem, w, extension, w)
            } else {
                format!("{}/{}-{}w.{} {}w", dir, stem, w, extension, w)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Generate WebP alternative URL for an image
/// Example: get_webp_url("/images/photo.jpg") returns "/images/photo.webp"
pub fn get_webp_url(original_url: &str) -> String {
    let path = Path::new(original_url);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let dir = path.parent().and_then(|p| p.to_str()).unwrap_or("");
   
    if dir.is_empty() {
        format!("{}.webp", stem)
    } else {
        format!("{}/{}.webp", dir, stem)
    }
}

/// Check if image should use lazy loading
/// Returns true for images that should have loading="lazy"
pub fn should_lazy_load(index: usize) -> bool {
    // Don't lazy load the first 3 images (above the fold)
    index >= 3
}

/// Generate picture element HTML with WebP support
/// This provides modern browsers with WebP, fallback to original format
pub fn generate_picture_html(
    original_url: &str,
    alt_text: &str,
    title: Option<&str>,
    lazy: bool,
) -> String {
    let webp_url = get_webp_url(original_url);
    let srcset = generate_srcset(original_url);
    let loading = if lazy { " loading=\"lazy\"" } else { "" };
    let title_attr = title.map(|t| format!(" title=\"{}\"", t)).unwrap_or_default();
    
    format!(
        r#"<picture>
  <source type="image/webp" srcset="{}">
  <source type="image/jpeg" srcset="{}">
  <img src="{}" alt="{}"{}{}>
</picture>"#,
        webp_url, srcset, original_url, alt_text, title_attr, loading
    )
}

/// Resize image to specified width, maintaining aspect ratio
pub fn resize_image(img: &DynamicImage, target_width: u32) -> DynamicImage {
    let width = img.width();
    let height = img.height();
    
    // Don't upscale images
    if width <= target_width {
        return img.clone();
    }
    
    let aspect_ratio = height as f32 / width as f32;
    let target_height = (target_width as f32 * aspect_ratio) as u32;
    
    img.resize(target_width, target_height, FilterType::Lanczos3)
}

/// Convert image to WebP format with quality setting
pub fn convert_to_webp(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, String> {
    let mut buffer = Cursor::new(Vec::new());
    
    // WebP encoding with quality
    img.write_to(&mut buffer, ImageFormat::WebP)
        .map_err(|e| format!("WebP conversion failed: {}", e))?;
    
    Ok(buffer.into_inner())
}

/// Optimize image: resize and optionally convert to WebP
/// Returns tuple of (optimized_jpeg_bytes, webp_bytes, width, height)
pub fn optimize_image(
    image_data: &[u8],
    max_width: u32,
    generate_webp: bool,
) -> Result<(Vec<u8>, Option<Vec<u8>>, u32, u32), String> {
    // Load image
    let img = image::load_from_memory(image_data)
        .map_err(|e| format!("Failed to load image: {}", e))?;
    
    // Resize if needed
    let resized = resize_image(&img, max_width);
    let width = resized.width();
    let height = resized.height();
    
    // Convert to JPEG with quality 85
    let mut jpeg_buffer = Cursor::new(Vec::new());
    resized
        .write_to(&mut jpeg_buffer, ImageFormat::Jpeg)
        .map_err(|e| format!("JPEG encoding failed: {}", e))?;
    let jpeg_bytes = jpeg_buffer.into_inner();
    
    // Optionally generate WebP
    let webp_bytes = if generate_webp {
        Some(convert_to_webp(&resized, 85)?)
    } else {
        None
    };
    
    Ok((jpeg_bytes, webp_bytes, width, height))
}

/// Generate responsive image variants at multiple widths
/// Returns vector of (width, image_bytes) tuples
pub fn generate_responsive_variants(
    image_data: &[u8],
) -> Result<Vec<(u32, Vec<u8>)>, String> {
    let img = image::load_from_memory(image_data)
        .map_err(|e| format!("Failed to load image: {}", e))?;
    
    let widths = vec![320, 768, 1024, 1920];
    let mut variants = Vec::new();
    
    for width in widths {
        let resized = resize_image(&img, width);
        let mut buffer = Cursor::new(Vec::new());
        resized
            .write_to(&mut buffer, ImageFormat::Jpeg)
            .map_err(|e| format!("Failed to encode variant: {}", e))?;
        variants.push((width, buffer.into_inner()));
    }
    
    Ok(variants)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_srcset() {
        let result = generate_srcset("/images/photo.jpg");
        assert!(result.contains("320w"));
        assert!(result.contains("768w"));
        assert!(result.contains("1024w"));
        assert!(result.contains("1920w"));
    }

    #[test]
    fn test_get_webp_url() {
        assert_eq!(get_webp_url("/images/photo.jpg"), "/images/photo.webp");
        assert_eq!(get_webp_url("photo.png"), "photo.webp");
    }

    #[test]
    fn test_should_lazy_load() {
        assert_eq!(should_lazy_load(0), false); // First image
        assert_eq!(should_lazy_load(2), false); // Third image
        assert_eq!(should_lazy_load(3), true);  // Fourth image
        assert_eq!(should_lazy_load(10), true); // Later images
    }

    #[test]
    fn test_generate_picture_html() {
        let html = generate_picture_html("/images/test.jpg", "Test Image", Some("My Title"), true);
        assert!(html.contains("loading=\"lazy\""));
        assert!(html.contains("title=\"My Title\""));
        assert!(html.contains("alt=\"Test Image\""));
        assert!(html.contains("image/webp"));
    }
}
