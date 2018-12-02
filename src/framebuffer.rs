use __gl;
use __gl::types::{GLenum, GLuint};

use device::Device;
use ImageView;
use Region;

///
pub enum ClearAttachment {
    ColorInt(usize, [i32; 4]),
    ColorUint(usize, [u32; 4]),
    ColorFloat(usize, [f32; 4]),
    Depth(f32),
    Stencil(i32),
    DepthStencil(f32, i32),
}

///
pub enum Attachment {
    Color(usize),
    Depth,
    Stencil,
    DepthStencil,
}

///
pub enum AttachmentView<'a> {
    Image(&'a ImageView),
    Renderbuffer(&'a Renderbuffer),
}

///
#[repr(transparent)]
pub struct Framebuffer(GLuint);

impl Framebuffer {
    pub const DEFAULT: &'static Self = &Framebuffer(0);
}

///
#[repr(transparent)]
pub struct Renderbuffer(GLuint);

impl Device {
    ///
    pub fn create_framebuffer(&self) -> Framebuffer {
        let mut framebuffer = 0;
        unsafe {
            self.0.CreateFramebuffers(1, &mut framebuffer);
        }
        self.get_error("CreateFramebuffers");

        Framebuffer(framebuffer)
    }

    /// Delete a framebuffer.
    pub fn delete_framebuffer(&self, framebuffer: Framebuffer) {
        self.delete_framebuffers(&[framebuffer])
    }

    /// Delete multiple framebuffers.
    pub fn delete_framebuffers(&self, framebuffers: &[Framebuffer]) {
        unsafe {
            self.0.DeleteFramebuffers(
                framebuffers.len() as _,
                framebuffers.as_ptr() as *const _, // newtype
            );
        }
        self.get_error("DeleteFramebuffers");
    }

    ///
    pub fn create_renderbuffer(&self) -> Renderbuffer {
        let mut renderbuffer = 0;
        unsafe {
            self.0.CreateRenderbuffers(1, &mut renderbuffer);
        }
        self.get_error("CreateRenderbuffers");

        Renderbuffer(renderbuffer)
    }

    /// Delete a renderbuffer.
    pub fn delete_renderbuffer(&self, renderbuffer: Renderbuffer) {
        self.delete_renderbuffers(&[renderbuffer])
    }

    /// Delete multiple renderbuffers.
    pub fn delete_renderbuffers(&self, renderbuffers: &[Renderbuffer]) {
        unsafe {
            self.0.DeleteRenderbuffers(
                renderbuffers.len() as _,
                renderbuffers.as_ptr() as *const _, // newtype
            );
        }
        self.get_error("DeleteRenderbuffers");
    }

    /// Clear framebuffer attachment.
    pub fn clear_attachment(&self, fb: &Framebuffer, cv: ClearAttachment) {
        unsafe {
            match cv {
                ClearAttachment::ColorInt(id, color) => {
                    self.0
                        .ClearNamedFramebufferiv(fb.0, __gl::COLOR, id as _, color.as_ptr());
                    self.get_error("ClearNamedFramebufferiv (Color)");
                }
                ClearAttachment::ColorUint(id, color) => {
                    self.0
                        .ClearNamedFramebufferuiv(fb.0, __gl::COLOR, id as _, color.as_ptr());
                    self.get_error("ClearNamedFramebufferuiv (Color)");
                }
                ClearAttachment::ColorFloat(id, color) => {
                    self.0
                        .ClearNamedFramebufferfv(fb.0, __gl::COLOR, id as _, color.as_ptr());
                    self.get_error("ClearNamedFramebufferfv (Color)");
                }
                ClearAttachment::Depth(depth) => {
                    self.0
                        .ClearNamedFramebufferfv(fb.0, __gl::DEPTH, 0, &depth as *const _);
                    self.get_error("ClearNamedFramebufferfv (Depth)");
                }
                ClearAttachment::Stencil(stencil) => {
                    self.0
                        .ClearNamedFramebufferiv(fb.0, __gl::STENCIL, 0, &stencil as *const _);
                    self.get_error("ClearNamedFramebufferiv (Stencil");
                }
                ClearAttachment::DepthStencil(depth, stencil) => {
                    self.0
                        .ClearNamedFramebufferfi(fb.0, __gl::DEPTH_STENCIL, 0, depth, stencil);
                    self.get_error("ClearNamedFramebufferfi (Depth-Stencil)");
                }
            }
        }
    }

    ///
    pub fn invalidate_attachments(
        &self,
        framebuffer: &Framebuffer,
        attachments: &[Attachment],
        region: Region,
    ) {
        let attachments = attachments
            .iter()
            .map(|att| match att {
                Attachment::Color(slot) => __gl::COLOR_ATTACHMENT0 + *slot as u32,
                Attachment::Depth => __gl::DEPTH_ATTACHMENT,
                Attachment::Stencil => __gl::STENCIL_ATTACHMENT,
                Attachment::DepthStencil => __gl::DEPTH_STENCIL_ATTACHMENT,
            }).collect::<Vec<_>>();

        unsafe {
            self.0.InvalidateNamedFramebufferSubData(
                framebuffer.0,
                attachments.len() as _,
                attachments.as_ptr(),
                region.x,
                region.y,
                region.w,
                region.h,
            )
        }
    }

    ///
    pub fn bind_framebuffer(&self, framebuffer: &Framebuffer) {
        unsafe {
            self.0
                .BindFramebuffer(__gl::DRAW_FRAMEBUFFER, framebuffer.0);
        }
        self.get_error("BindFramebuffer");
    }

    ///
    pub fn bind_attachments(
        &self,
        framebuffer: &Framebuffer,
        color_attachments: &[AttachmentView],
        depth_stencil_attachment: Option<AttachmentView>,
    ) {
        assert_ne!(
            framebuffer.0, 0,
            "The default framebuffer can't be changed."
        );

        let bind_attachment_view = |attachment: GLenum, view: &AttachmentView| unsafe {
            match view {
                &AttachmentView::Image(image) => {
                    self.0
                        .NamedFramebufferTexture(framebuffer.0, attachment, image.0, 0);
                    self.get_error("NamedFramebufferTexture");
                }
                &AttachmentView::Renderbuffer(_) => unimplemented!(),
            }
        };

        for (i, attachment) in color_attachments.iter().enumerate() {
            bind_attachment_view((__gl::COLOR_ATTACHMENT0 as usize + i) as _, attachment);
        }

        for attachment in depth_stencil_attachment {
            bind_attachment_view(__gl::DEPTH_STENCIL_ATTACHMENT, &attachment);
        }
    }

    ///
    pub fn set_color_attachments(&self, framebuffer: &Framebuffer, attachments: &[u32]) {
        assert_ne!(
            framebuffer.0, 0,
            "The default framebuffer can't be changed."
        );

        let attachments = attachments
            .iter()
            .map(|i| i + __gl::COLOR_ATTACHMENT0)
            .collect::<Vec<_>>();
        unsafe {
            self.0.NamedFramebufferDrawBuffers(
                framebuffer.0,
                attachments.len() as _,
                attachments.as_ptr(),
            );
        }
        self.get_error("NamedFramebufferDrawBuffers");
    }
}
