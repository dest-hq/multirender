#[cfg(feature = "softbuffer_window_renderer")]
pub use multirender_softbuffer_window_renderer::SoftbufferWindowRenderer;

#[cfg(feature = "pixels_window_renderer")]
pub use multirender_pixels_window_renderer::PixelsWindowRenderer;

#[cfg(feature = "pixels_window_renderer")]
pub type VelloCpuWindowRenderer = PixelsWindowRenderer<crate::VelloCpuImageRenderer>;
#[cfg(all(
    feature = "softbuffer_window_renderer",
    not(feature = "pixels_window_renderer")
))]
pub type VelloCpuWindowRenderer = SoftbufferWindowRenderer<crate::VelloCpuImageRenderer>;
