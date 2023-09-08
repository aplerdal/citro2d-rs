//! Definitions from `<c2d/sprite.h>`

use core::ptr::null;

use crate::*;

#[inline]
pub static fn C2D_SpriteFromImage(sprite: &mut C2D_Sprite, image: C2D_Image){
    sprite.image           = image;
    sprite.params.pos.x    = 0.0_f32;
    sprite.params.pos.y    = 0.0_f32;
    sprite.params.pos.w.   = image.subtex.width;
    sprite.params.pos.h    = image.subtex.height;
    sprite.params.center.x = 0.0_f32;
    sprite.params.center.y = 0.0_f32;
    sprite.params.angle    = 0.0_f32;
    sprite.params.depth    = 0.0_f32;
}
#[inline]
pub static fn C2D_SpriteFromSheet(sprite: &mut C2D_Sprite, sheet: C2D_SpriteSheet ,index:u32) {
     C2D_SpriteFromImage(sprite, C2D_SpriteSheetGetImage(sheet, index));
}

#[inline]
pub static fn C2D_SpriteScale(sprite: &mut C2D_Sprite, x:f32, y:f32) {
    sprite.params.pos.w *= x;
    sprite.params.pos.h *= y;
    sprite.params.center.x *= x;
    sprite.params.center.y *= y;
}
#[inline]
pub static fn C2D_SpriteRotate(sprite: &mut C2D_Sprite, radians:f32) {
    sprite.params.angle += radians;
}
#[inline]
pub static fn C2D_SpriteRotateDegrees(sprite: &mut C2D_Sprite, degrees:f32) {
    C2D_SpriteRotate(sprite, C3D_AngleFromDegrees(degrees));
}

