use core_foundation::{
    array::CFArray,
    base::{CFAllocatorRef, CFType, TCFType},
    boolean::CFBoolean,
    number::CFNumber,
    string::{CFString, CFStringRef},
};

use core_graphics::display::CFDictionary;

#[derive(Debug, Clone)]
pub enum PixelFormatValue {
    Single(usize),
    Many(Vec<usize>),
}

#[derive(Debug, Clone)]
pub enum PixelBufferAttribute {
    MemoryAllocator(CFAllocatorRef),
    PixelFormat(PixelFormatValue),
    Width(usize),
    Height(usize),
    ExtendedPixelsLeft(usize),
    ExtendedPixelsTop(usize),
    ExtendedPixelsRight(usize),
    ExtendedPixelsBottom(usize),
    BytesPerRowAlignment(usize),
    CGBitmapContextCompatibility(bool),
    CGImageCompatibility(bool),
    OpenGLCompatibility(bool),
    PlaneAlignment(usize),
    IOSurfaceProperties(CFDictionary),
    OpenGLESCompatibility(bool),
    MetalCompatibility(bool),
    IOSurfaceCoreAnimationCompatibility(bool),
    IOSurfaceOpenGLFBOCompatibility(bool),
    IOSurfaceOpenGLESFBOCompatibility(bool),
    IOSurfaceOpenGLTextureCompatibility(bool),
    IOSurfaceOpenGLESTextureCompatibility(bool),
    OpenGLTextureCacheCompatibility(bool),
    OpenGLESTextureCacheCompatibility(bool),
}

#[derive(Debug, Clone)]
pub struct PixelBufferAttributes(Vec<PixelBufferAttribute>);
impl PixelBufferAttributes {
    pub fn new(attributes: &[PixelBufferAttribute]) -> Self {
        Self(attributes.to_vec())
    }
    pub fn add(&mut self, attribute: PixelBufferAttribute) {
        self.0.push(attribute);
    }
}

impl Default for PixelBufferAttributes {
    fn default() -> Self {
        Self::new(&[])
    }
}
impl From<PixelBufferAttributes> for CFDictionary<CFString, CFType> {
    fn from(val: PixelBufferAttributes) -> Self {
        let mut pairs = Vec::new();
        for attribute in val.0 {
            match attribute {
                // fill missing
                PixelBufferAttribute::MemoryAllocator(allocator) => {
                    pairs.push(unsafe {
                        (
                            CFString::wrap_under_get_rule(kCVPixelBufferMemoryAllocatorKey),
                            CFType::wrap_under_get_rule(allocator),
                        )
                    });
                }
                PixelBufferAttribute::PixelFormat(format) => pairs.push((
                    unsafe { CFString::wrap_under_get_rule(kCVPixelBufferPixelFormatTypeKey) },
                    match format {
                        PixelFormatValue::Single(val) => CFNumber::from(val as i64).into_CFType(),
                        PixelFormatValue::Many(vals) => CFArray::from_CFTypes(
                            vals.into_iter()
                                .map(|n| CFNumber::from(n as i64))
                                .collect::<Vec<CFNumber>>()
                                .as_slice(),
                        )
                        .into_CFType(),
                    },
                )),
                PixelBufferAttribute::Width(val) => {
                    pairs.push((
                        unsafe { CFString::wrap_under_get_rule(kCVPixelBufferWidthKey) },
                        CFNumber::from(val as i64).into_CFType(),
                    ));
                }
                PixelBufferAttribute::Height(val) => {
                    pairs.push((
                        unsafe { CFString::wrap_under_get_rule(kCVPixelBufferHeightKey) },
                        CFNumber::from(val as i64).into_CFType(),
                    ));
                }
                PixelBufferAttribute::ExtendedPixelsLeft(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferExtendedPixelsLeftKey)
                        },
                        CFNumber::from(val as i64).into_CFType(),
                    ));
                }
                PixelBufferAttribute::ExtendedPixelsTop(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferExtendedPixelsTopKey)
                        },
                        CFNumber::from(val as i64).into_CFType(),
                    ));
                }
                PixelBufferAttribute::ExtendedPixelsRight(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferExtendedPixelsRightKey)
                        },
                        CFNumber::from(val as i64).into_CFType(),
                    ));
                }
                PixelBufferAttribute::ExtendedPixelsBottom(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferExtendedPixelsBottomKey)
                        },
                        CFNumber::from(val as i64).into_CFType(),
                    ));
                }
                PixelBufferAttribute::BytesPerRowAlignment(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferBytesPerRowAlignmentKey)
                        },
                        CFNumber::from(val as i64).into_CFType(),
                    ));
                }
                PixelBufferAttribute::CGImageCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferCGImageCompatibilityKey)
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::CGBitmapContextCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(
                                kCVPixelBufferCGBitmapContextCompatibilityKey,
                            )
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::OpenGLCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferOpenGLCompatibilityKey)
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::PlaneAlignment(val) => {
                    pairs.push((
                        unsafe { CFString::wrap_under_get_rule(kCVPixelBufferPlaneAlignmentKey) },
                        CFNumber::from(val as i64).into_CFType(),
                    ));
                }
                PixelBufferAttribute::IOSurfaceProperties(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferIOSurfacePropertiesKey)
                        },
                        val.into_CFType(),
                    ));
                }
                PixelBufferAttribute::OpenGLESCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferOpenGLESCompatibilityKey)
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::MetalCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(kCVPixelBufferMetalCompatibilityKey)
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::IOSurfaceCoreAnimationCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(
                                kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey,
                            )
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::IOSurfaceOpenGLFBOCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(
                                kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey,
                            )
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::IOSurfaceOpenGLESFBOCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(
                                kCVPixelBufferIOSurfaceOpenGLESFBOCompatibilityKey,
                            )
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::IOSurfaceOpenGLTextureCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(
                                kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey,
                            )
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::IOSurfaceOpenGLESTextureCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(
                                kCVPixelBufferIOSurfaceOpenGLESTextureCompatibilityKey,
                            )
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::OpenGLTextureCacheCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(
                                kCVPixelBufferOpenGLTextureCacheCompatibilityKey,
                            )
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
                PixelBufferAttribute::OpenGLESTextureCacheCompatibility(val) => {
                    pairs.push((
                        unsafe {
                            CFString::wrap_under_get_rule(
                                kCVPixelBufferOpenGLESTextureCacheCompatibilityKey,
                            )
                        },
                        CFBoolean::from(val).into_CFType(),
                    ));
                }
            }
        }
        CFDictionary::from_CFType_pairs(&pairs)
    }
}

extern "C" {
    // A key to the allocator that the system uses to create the pixel buffer.
    static kCVPixelBufferMemoryAllocatorKey: CFStringRef;
    // A key to one or more pixel buffer format types.
    static kCVPixelBufferPixelFormatTypeKey: CFStringRef;
    // A key to the width of the pixel buffer.
    static kCVPixelBufferWidthKey: CFStringRef;
    // A key to the height of the pixel buffer.
    static kCVPixelBufferHeightKey: CFStringRef;
    // A key to the number of pixels padding the left of the image.
    static kCVPixelBufferExtendedPixelsLeftKey: CFStringRef;
    // A key to the number of pixels padding the top of the image.
    static kCVPixelBufferExtendedPixelsTopKey: CFStringRef;
    // A key to the number of pixels padding the right of the image.
    static kCVPixelBufferExtendedPixelsRightKey: CFStringRef;
    // A key to the number of pixels padding the bottom of the image.
    static kCVPixelBufferExtendedPixelsBottomKey: CFStringRef;
    // A key to a number that specifies the alignment of number of bytes per row in the pixel buffer.
    static kCVPixelBufferBytesPerRowAlignmentKey: CFStringRef;
    // A key to a Boolean value that indicates whether the pixel buffer is compatible with Core Graphics bitmap contexts.
    static kCVPixelBufferCGBitmapContextCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whether the pixel buffer is compatible with Core Graphics bitmap image types.
    static kCVPixelBufferCGImageCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whether the pixel buffer is compatible with OpenGL contexts.
    static kCVPixelBufferOpenGLCompatibilityKey: CFStringRef;
    // A key to a number that specifies the alignment of the planes in the pixel buffer.
    static kCVPixelBufferPlaneAlignmentKey: CFStringRef;
    // A key to the dictionary containing optional CFStringRef for the IOSurface framework.
    static kCVPixelBufferIOSurfacePropertiesKey: CFStringRef;
    // A key to a Boolean value that indicates whether the pixel buffer is compatible with OpenGL ES contexts.
    static kCVPixelBufferOpenGLESCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whetherCFStringRef pixel buffer is compatible with the Metal framework.
    static kCVPixelBufferMetalCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whether Core Animation can display the pixel buffer.
    static kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whether OpenGL can cCFStringRef a valid texture for use as a color buffer attachment.
    static kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whether OpenGL ES can create a valid texture for use as a color buffer attachment.
    static kCVPixelBufferIOSurfaceOpenGLESFBOCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whether OpenGL can crCFStringRef a valid texture object from the IOSurface-backed pixel buffer.
    static kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whether OpenGL ES can create a valid texture object from the IOSurface-backed pixel buffer.
    static kCVPixelBufferIOSurfaceOpenGLESTextureCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whether OpenGL performs format conversions of the texture-cache data in a shader.
    static kCVPixelBufferOpenGLTextureCacheCompatibilityKey: CFStringRef;
    // A key to a Boolean value that indicates whether OpenGL ES performs format conversions of the texture-cache data in a shader.
    static kCVPixelBufferOpenGLESTextureCacheCompatibilityKey: CFStringRef;
}
