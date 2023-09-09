//! Definitions from `<c2d/sprite.h>`

/*TODO finish comments (coppied from sprite.h)
write in the format:

/// ### C2D_Example
/// description\
/// **params:**\
/// `param` - info

*/

use core::{ptr::null};

use ctru_sys::float24Uniform_s;

use crate::*;

pub const M_PI : f32 = 3.14159265358979323846;
pub const M_TAU : f32 = 2.0_f32*M_PI;
/// ### C3D_AngleFromDegrees
/// converts radians to degrees\
/// **params:**\
/// `_angle` - Angle in degrees
pub fn C3D_AngleFromDegrees(_angle : f32) -> f32 {
    return (_angle*M_TAU);
}

fn abs(f:f32) -> f32{
    if (f<0.0_f32){
        return -f;
    }
    return f;
}

/// ### C2D_SpriteFromImage
/// description\
/// **params:**\
/// `param` - info
#[inline]
pub unsafe fn C2D_SpriteFromImage(sprite: &mut C2D_Sprite, image: C2D_Image){
    sprite.image           = image;
    sprite.params.pos.x    = 0.0_f32;
    sprite.params.pos.y    = 0.0_f32;
    sprite.params.pos.w    = ((*image.subtex).width) as f32;
    sprite.params.pos.h    = ((*image.subtex).height) as f32;
    sprite.params.center.x = 0.0_f32;
    sprite.params.center.y = 0.0_f32;
    sprite.params.angle    = 0.0_f32;
    sprite.params.depth    = 0.0_f32;
}
/// ### C2D_SpriteFromSheet
/// description\
/// **params:**\
/// `param` - info
#[inline]
pub unsafe fn C2D_SpriteFromSheet(sprite: &mut C2D_Sprite, sheet: C2D_SpriteSheet ,index:u32) {
     C2D_SpriteFromImage(sprite, C2D_SpriteSheetGetImage(sheet, index as usize));
}
/// ### C2D_SpriteScale
/// description\
/// **params:**\
/// `sprite` - Mutable reference to sprite\
/// `x` - X scale (negative values flip the sprite vertically)
/// `y` - Y scale (negative values flip the sprite vertically)
#[inline]
pub unsafe fn C2D_SpriteScale(sprite: &mut C2D_Sprite, x:f32, y:f32) {
    sprite.params.pos.w *= x;
    sprite.params.pos.h *= y;
    sprite.params.center.x *= x;
    sprite.params.center.y *= y;
}
/// ### C2D_SpriteRotate
/// Rotate sprite (absolute)\
/// **params:**\
/// `sprite` - Mutable reference to sprite\
/// `radians` - Amount to rotate in radians
#[inline]
pub unsafe fn C2D_SpriteRotate(sprite: &mut C2D_Sprite, radians:f32) {
    sprite.params.angle += radians;
}
/// ### C2D_SpriteRotateDegrees
/// Rotate sprite (absolute)\
/// **params:**\
/// `sprite` - Mutable reference to sprite\
/// `degrees` - Amount to rotate in degrees
#[inline]
pub unsafe fn C2D_SpriteRotateDegrees(sprite: &mut C2D_Sprite, degrees:f32) {
    C2D_SpriteRotate(sprite, C3D_AngleFromDegrees(degrees));
}
#[inline]
pub unsafe fn C2D_SpriteMove(sprite:&mut C2D_Sprite,x:f32,y:f32){
    sprite.params.pos.x += x;
    sprite.params.pos.y += y;
}
#[inline]
pub unsafe fn C2D_SpriteSetScale(sprite:&mut C2D_Sprite,x:f32,y:f32){
    let oldCenterX:f32=sprite.params.center.x /sprite.params.pos.w;
    let oldCenterY:f32=sprite.params.center.y /sprite.params.pos.h;
    sprite.params.pos.w = x*((*sprite.image.subtex).width as f32);
    sprite.params.center.x = abs(oldCenterX*sprite.params.pos.w);
    sprite.params.center.y = abs(oldCenterY*sprite.params.pos.h);
}
#[inline]
pub unsafe fn C2D_SpriteSetRotation(sprite:&mut C2D_Sprite, radians:f32){
    sprite.params.angle = radians
}
#[inline]
pub unsafe fn C2D_SpriteSetRotationDegrees(sprite:&mut C2D_Sprite, degrees:f32){
    C2D_SpriteSetRotation(sprite, C3D_AngleFromDegrees(degrees))
}
/// ### C2D_SpriteSetCenter
/// Set the center of a sprite in values independent of the sprite size (absolute)\
/// **params:**\
/// `sprite` - Mutable reference to sprite\
/// `x` - X position of the center (0.0 through 1.0)
/// `y` - Y position of the center (0.0 through 1.0)
#[inline]
pub unsafe fn C2D_SpriteSetCenter(sprite: &mut C2D_Sprite, x:f32, y:f32) {
    sprite.params.center.x = x*sprite.params.pos.w;
    sprite.params.center.y = y*sprite.params.pos.h;
}
/// ### C2D_SpriteSetCenterRaw
/// Set the center of a sprite in terms of pixels (absolute)\
/// **params:**\
/// `sprite` - Mutable reference to sprite\
/// `x` - X position\
/// `y` - Y position
#[inline]
pub unsafe fn C2D_SpriteSetCenterRaw(sprite: &mut C2D_Sprite, x:f32, y:f32) {
    sprite.params.center.x = x;
    sprite.params.center.y = y;
}
/// ### C2D_SpriteSetPos
/// Move sprite (absolute)\
/// **params:**\
/// `sprite` - Mutable reference to sprite\
/// `x` - X position\
/// `y` - Y position
#[inline]
pub unsafe fn C2D_SpriteSetPos(sprite: &mut C2D_Sprite, x:f32, y:f32) {
    sprite.params.pos.x = x;
    sprite.params.pos.y = y;
}
/// ### C2D_SpriteSetDepth
/// Sets the depth level of a sprite (absolute)\
/// **params:**\
/// `sprite` - Mutable reference to sprite\
/// `depth` - Depth value
#[inline]
pub unsafe fn C2D_SpriteSetDepth(sprite: &mut C2D_Sprite, depth:f32) {
    sprite.params.depth = depth;
}
/// ### C2D_DrawSprite
/// Draw sprite\
/// **params:**\
/// `sprite` - Sprite to draw
#[inline]
pub unsafe fn C2D_DrawSprite(sprite: &mut C2D_Sprite) -> bool{
    return C2D_DrawImage(sprite.image, sprite.params, C2D_ImageTint { corners: [C2D_Tint {color:C2D_Color32(1, 1, 1, 0),blend:0.0_f32},C2D_Tint {color:C2D_Color32(1, 1, 1, 0),blend:0.0_f32},C2D_Tint {color:C2D_Color32(1, 1, 1, 0),blend:0.0_f32},C2D_Tint {color:C2D_Color32(1, 1, 1, 0),blend:0.0_f32}] });
}
/// ### C2D_DrawSpriteTinted
/// Draw sprite with color tinting\
/// **params:**\
/// `sprite` - Sprite to draw\
/// `tint` - Color tinting parameters to apply to the sprite
#[inline]
pub unsafe fn C2D_DrawSpriteTinted(sprite: &mut C2D_Sprite,tint:C2D_ImageTint) -> bool{
    return C2D_DrawImage(sprite.image, sprite.params,tint);
}