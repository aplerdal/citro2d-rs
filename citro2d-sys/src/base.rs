//! Definitions from `<c2d/base.h>`

use core::ptr::null;

use crate::*;


pub const M_PI : f32 = 3.14159265358979323846;
pub const M_TAU : f32 = 2.0_f32*M_PI;
pub fn C3D_AngleFromDegrees(_angle : f32) -> f32 {
    return (_angle*M_TAU);
}
pub fn C2D_FloatToU8(x:f32) -> u8{
	return (255.0_f32*x.clamp(0.0_f32, 1.0_f32)+0.5_f32) as u8;
}
pub fn C2D_Color32(r:u8,g:u8,b:u8,a:u8) -> u32{
	return ((r as u32) | ((g as u32) << 8) | ((b as u32) << 16) | ((a as u32) << 24));
}
pub fn C2D_Color32f(r:f32, g:f32, b:f32, a:f32) -> u32{
	return C2D_Color32(C2D_FloatToU8(r),C2D_FloatToU8(g),C2D_FloatToU8(b),C2D_FloatToU8(a));
}

#[derive(Clone)]
pub enum C2D_Corner {
    C2D_TopLeft,  ///< Top left corner
	C2D_TopRight, ///< Top right corner
	C2D_BotLeft,  ///< Bottom left corner
	C2D_BotRight
}

#[inline]
pub unsafe fn C2D_SetImageTint(tint : &mut C2D_ImageTint, corner:C2D_Corner, color:u32,blend:f32){
    tint.corners[corner.clone() as usize].color = color;
    tint.corners[corner as usize].blend = blend;
}

#[inline]
pub unsafe fn C2D_PlainImageTint(tint : &mut C2D_ImageTint, color:u32,blend:f32){
    C2D_SetImageTint(tint, C2D_Corner::C2D_TopLeft,  color, blend);
	C2D_SetImageTint(tint, C2D_Corner::C2D_TopRight, color, blend);
	C2D_SetImageTint(tint, C2D_Corner::C2D_BotLeft,  color, blend);
	C2D_SetImageTint(tint, C2D_Corner::C2D_BotRight, color, blend);
}

#[inline]
pub unsafe fn C2D_AlphaImageTint(tint : &mut C2D_ImageTint, alpha:f32){
    C2D_PlainImageTint(tint, C2D_Color32f(0.0_f32, 0.0_f32, 0.0_f32, alpha), 0.0_f32)
}

#[inline]
pub unsafe fn C2D_TopImageTint(tint : &mut C2D_ImageTint,color:u32,blend:f32) {
    C2D_SetImageTint(tint, C2D_Corner::C2D_TopLeft,  color, blend);
	C2D_SetImageTint(tint, C2D_Corner::C2D_TopRight, color, blend);
}

#[inline]
pub unsafe fn C2D_BottomImageTint(tint : &mut C2D_ImageTint,color:u32,blend:f32) {
    C2D_SetImageTint(tint, C2D_Corner::C2D_BotLeft,  color, blend);
	C2D_SetImageTint(tint, C2D_Corner::C2D_BotRight, color, blend);
}

#[inline]
pub unsafe fn C2D_LeftImageTint(tint : &mut C2D_ImageTint,color:u32,blend:f32) {
    C2D_SetImageTint(tint, C2D_Corner::C2D_TopLeft, color, blend);
	C2D_SetImageTint(tint, C2D_Corner::C2D_BotLeft, color, blend);
}

#[inline]
pub unsafe fn C2D_RightImageTint(tint : &mut C2D_ImageTint,color:u32,blend:f32) {
    C2D_SetImageTint(tint, C2D_Corner::C2D_TopRight, color, blend);
	C2D_SetImageTint(tint, C2D_Corner::C2D_BotRight, color, blend);
}

#[inline]
pub unsafe fn C2D_SCeneTarget(target: C3D_RenderTarget) {
    C2D_SceneSize(target.frameBuf.width as u32_, target.frameBuf.height as u32_, target.linked)
}

#[inline]
pub unsafe fn C2D_ViewRotateDegrees(rotation : f32) {
    C2D_ViewRotate(C3D_AngleFromDegrees(rotation))
}

#[inline]
pub unsafe fn C2D_SceneBegin(target: &mut C3D_RenderTarget){
    C2D_Flush();
    C3D_FrameDrawOn(target);
    C2D_SCeneTarget(*target);
}
#[inline]
pub unsafe fn C2D_DrawImageAt(img:C2D_Image,x:f32,y:f32,depth:f32,/*following params should be optional*/tint:C2D_ImageTint,scaleX:f32,scaleY:f32) -> bool{
    let text: *const Tex3DS_SubTexture = img.subtex;
    let params : C2D_DrawParams = C2D_DrawParams {
        pos: C2D_DrawParams__bindgen_ty_1 { x:x, y:y, w:scaleX*((*text).width as f32), h:scaleY*((*text).height as f32)},
        center: C2D_DrawParams__bindgen_ty_2 {x: 0.0_f32,y: 0.0_f32},
        depth: depth,
        angle: 0.0_f32
    };
    return C2D_DrawImage(img, params, tint);
}

#[inline]
pub unsafe fn C2D_DrawImageAtRotated(img:C2D_Image,x:f32,y:f32,depth:f32,angle:f32,/*following params should be optional*/tint:C2D_ImageTint,scaleX:f32,scaleY:f32)->bool{
    let text: *const Tex3DS_SubTexture = img.subtex;
    let params : C2D_DrawParams = C2D_DrawParams {
        pos: C2D_DrawParams__bindgen_ty_1 { x:x, y:y, w:scaleX*((*text).width as f32), h:scaleY*((*text).height as f32)},
        center: C2D_DrawParams__bindgen_ty_2 {x: (scaleX*((*text).width as f32))/2.0_f32,y: (scaleY*((*text).height as f32))/2.0_f32},
        depth: depth,
        angle: angle
    };
    return C2D_DrawImage(img, params, tint);
}

#[inline]
pub unsafe fn C2D_DrawRectSolid(x:f32,y:f32,z:f32,w:f32,h:f32,clr:u32) -> bool{
    return C2D_DrawRectangle(x, y, z, w, h, clr, clr, clr, clr);
}

#[inline]
pub unsafe fn C2D_DrawEllipseSolid(x:f32,y:f32,z:f32,w:f32,h:f32,clr:u32) -> bool{
    return C2D_DrawEllipse(x, y, z, w, h, clr, clr, clr, clr);
}