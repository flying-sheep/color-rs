// Copyright 2013 The color-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::num::{self, Int, Float};

use {Color, FloatColor, Color3};
use {Channel, FloatChannel};
use {Hsv, ToHsv};

#[derive(Clone, Copy, PartialEq, Eq, Show)]
pub struct Rgb<T> { pub r: T, pub g: T, pub b: T }

fn cast<T: num::NumCast, U: num::NumCast>(n: T) -> U {
    num::cast(n).unwrap()
}

impl<T:Channel> Rgb<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T) -> Rgb<T> {
        Rgb { r: r, g: g, b: b }
    }
}

impl<T:Channel> Color<T> for Rgb<T> {
    /// Clamps the components of the color to the range `(lo,hi)`.
    #[inline]
    fn clamp_s(self, lo: T, hi: T) -> Rgb<T> {
        Rgb::new(self.r.clamp(lo, hi),
                 self.g.clamp(lo, hi),
                 self.b.clamp(lo, hi))
    }

    /// Clamps the components of the color component-wise between `lo` and `hi`.
    #[inline]
    fn clamp_c(self, lo: Rgb<T>, hi: Rgb<T>) -> Rgb<T> {
        Rgb::new(self.r.clamp(lo.r, hi.r),
                 self.g.clamp(lo.g, hi.g),
                 self.b.clamp(lo.b, hi.b))
    }

    /// Inverts the color.
    #[inline]
    fn inverse(self) -> Rgb<T> {
        Rgb::new(self.r.invert_channel(),
                 self.g.invert_channel(),
                 self.b.invert_channel())
    }
}

impl<T:FloatChannel> FloatColor<T> for Rgb<T> {
    /// Normalizes the components of the color by clamping them to the range `(0,1)`.
    #[inline]
    fn normalize(self) -> Rgb<T> {
        Rgb::new(self.r.normalize_channel(),
                 self.g.normalize_channel(),
                 self.b.normalize_channel())
    }
}

impl<T: Channel> Color3<T> for Rgb<T> {
    fn into_fixed(self) -> [T; 3] {
        match self {
            Rgb { r, g, b } => [r, g, b],
        }
    }
}

pub trait ToRgb {
    fn to_rgb<U:Channel>(&self) -> Rgb<U>;
}

impl ToRgb for u32 {
    #[inline]
    fn to_rgb<U:Channel>(&self) -> Rgb<U> {
        panic!("Not yet implemented")
    }
}

impl ToRgb for u64 {
    #[inline]
    fn to_rgb<U:Channel>(&self) -> Rgb<U> {
        panic!("Not yet implemented")
    }
}

impl<T:Clone + Channel> ToRgb for Rgb<T> {
    #[inline]
    fn to_rgb<U:Channel>(&self) -> Rgb<U> {
        Rgb::new(self.r.to_channel(),
                 self.g.to_channel(),
                 self.b.to_channel())
    }
}

impl<T:Clone + Channel + Int> ToHsv for Rgb<T> {
    #[inline]
    fn to_hsv<U:FloatChannel>(&self) -> Hsv<U> {
        // Algorithm taken from the Wikipedia article on HSL and Hsv:
        // http://en.wikipedia.org/wiki/HSL_and_Hsv#From_Hsv

        let rgb_u = self.to_rgb::<U>();

        let mx = rgb_u.r.max(rgb_u.g).max(rgb_u.b);
        let mn = rgb_u.r.min(rgb_u.g).min(rgb_u.b);
        let chr = mx - mn;

        if chr != Float::zero() {
            let h =
                if      rgb_u.r == mx       { ((rgb_u.g - rgb_u.b) / chr) % cast(6u8) }
                else if rgb_u.g == mx       { ((rgb_u.b - rgb_u.r) / chr) + cast(2u8) }
                else      /* rgb_u.b == mx */ { ((rgb_u.r - rgb_u.g) / chr) + cast(4u8) }
            * cast(60u8);

            let s = chr / mx;

            Hsv::new(h, s, mx)

        } else {
            Hsv::new(Float::zero(), Float::zero(), mx)
        }
    }
}

/// SVG 1.0 color constants: http://www.w3.org/TR/SVG/types.html#ColorKeywords
pub mod consts {
    use Rgb;

    pub static ALICEBLUE:               Rgb<u8> = Rgb { r: 0xF0, g: 0xF8, b: 0xFF };
    pub static ANTIQUEWHITE:            Rgb<u8> = Rgb { r: 0xFA, g: 0xEB, b: 0xD7 };
    pub static AQUA:                    Rgb<u8> = Rgb { r: 0x00, g: 0xFF, b: 0xFF };
    pub static AQUAMARINE:              Rgb<u8> = Rgb { r: 0x7F, g: 0xFF, b: 0xD4 };
    pub static AZURE:                   Rgb<u8> = Rgb { r: 0xF0, g: 0xFF, b: 0xFF };
    pub static BEIGE:                   Rgb<u8> = Rgb { r: 0xF5, g: 0xF5, b: 0xDC };
    pub static BISQUE:                  Rgb<u8> = Rgb { r: 0xFF, g: 0xE4, b: 0xC4 };
    pub static BLACK:                   Rgb<u8> = Rgb { r: 0x00, g: 0x00, b: 0x00 };
    pub static BLANCHEDALMOND:          Rgb<u8> = Rgb { r: 0xFF, g: 0xEB, b: 0xCD };
    pub static BLUE:                    Rgb<u8> = Rgb { r: 0x00, g: 0x00, b: 0xFF };
    pub static BLUEVIOLET:              Rgb<u8> = Rgb { r: 0x8A, g: 0x2B, b: 0xE2 };
    pub static BROWN:                   Rgb<u8> = Rgb { r: 0xA5, g: 0x2A, b: 0x2A };
    pub static BURLYWOOD:               Rgb<u8> = Rgb { r: 0xDE, g: 0xB8, b: 0x87 };
    pub static CADETBLUE:               Rgb<u8> = Rgb { r: 0x5F, g: 0x9E, b: 0xA0 };
    pub static CHARTREUSE:              Rgb<u8> = Rgb { r: 0x7F, g: 0xFF, b: 0x00 };
    pub static CHOCOLATE:               Rgb<u8> = Rgb { r: 0xD2, g: 0x69, b: 0x1E };
    pub static CORAL:                   Rgb<u8> = Rgb { r: 0xFF, g: 0x7F, b: 0x50 };
    pub static CORNFLOWERBLUE:          Rgb<u8> = Rgb { r: 0x64, g: 0x95, b: 0xED };
    pub static CORNSILK:                Rgb<u8> = Rgb { r: 0xFF, g: 0xF8, b: 0xDC };
    pub static CRIMSON:                 Rgb<u8> = Rgb { r: 0xDC, g: 0x14, b: 0x3C };
    pub static CYAN:                    Rgb<u8> = Rgb { r: 0x00, g: 0xFF, b: 0xFF };
    pub static DARKBLUE:                Rgb<u8> = Rgb { r: 0x00, g: 0x00, b: 0x8B };
    pub static DARKCYAN:                Rgb<u8> = Rgb { r: 0x00, g: 0x8B, b: 0x8B };
    pub static DARKGOLDENROD:           Rgb<u8> = Rgb { r: 0xB8, g: 0x86, b: 0x0B };
    pub static DARKGRAY:                Rgb<u8> = Rgb { r: 0xA9, g: 0xA9, b: 0xA9 };
    pub static DARKGREEN:               Rgb<u8> = Rgb { r: 0x00, g: 0x64, b: 0x00 };
    pub static DARKKHAKI:               Rgb<u8> = Rgb { r: 0xBD, g: 0xB7, b: 0x6B };
    pub static DARKMAGENTA:             Rgb<u8> = Rgb { r: 0x8B, g: 0x00, b: 0x8B };
    pub static DARKOLIVEGREEN:          Rgb<u8> = Rgb { r: 0x55, g: 0x6B, b: 0x2F };
    pub static DARKORANGE:              Rgb<u8> = Rgb { r: 0xFF, g: 0x8C, b: 0x00 };
    pub static DARKORCHID:              Rgb<u8> = Rgb { r: 0x99, g: 0x32, b: 0xCC };
    pub static DARKRED:                 Rgb<u8> = Rgb { r: 0x8B, g: 0x00, b: 0x00 };
    pub static DARKSALMON:              Rgb<u8> = Rgb { r: 0xE9, g: 0x96, b: 0x7A };
    pub static DARKSEAGREEN:            Rgb<u8> = Rgb { r: 0x8F, g: 0xBC, b: 0x8F };
    pub static DARKSLATEBLUE:           Rgb<u8> = Rgb { r: 0x48, g: 0x3D, b: 0x8B };
    pub static DARKSLATEGRAY:           Rgb<u8> = Rgb { r: 0x2F, g: 0x4F, b: 0x4F };
    pub static DARKTURQUOISE:           Rgb<u8> = Rgb { r: 0x00, g: 0xCE, b: 0xD1 };
    pub static DARKVIOLET:              Rgb<u8> = Rgb { r: 0x94, g: 0x00, b: 0xD3 };
    pub static DEEPPINK:                Rgb<u8> = Rgb { r: 0xFF, g: 0x14, b: 0x93 };
    pub static DEEPSKYBLUE:             Rgb<u8> = Rgb { r: 0x00, g: 0xBF, b: 0xFF };
    pub static DIMGRAY:                 Rgb<u8> = Rgb { r: 0x69, g: 0x69, b: 0x69 };
    pub static DODGERBLUE:              Rgb<u8> = Rgb { r: 0x1E, g: 0x90, b: 0xFF };
    pub static FIREBRICK:               Rgb<u8> = Rgb { r: 0xB2, g: 0x22, b: 0x22 };
    pub static FLORALWHITE:             Rgb<u8> = Rgb { r: 0xFF, g: 0xFA, b: 0xF0 };
    pub static FORESTGREEN:             Rgb<u8> = Rgb { r: 0x22, g: 0x8B, b: 0x22 };
    pub static FUCHSIA:                 Rgb<u8> = Rgb { r: 0xFF, g: 0x00, b: 0xFF };
    pub static GAINSBORO:               Rgb<u8> = Rgb { r: 0xDC, g: 0xDC, b: 0xDC };
    pub static GHOSTWHITE:              Rgb<u8> = Rgb { r: 0xF8, g: 0xF8, b: 0xFF };
    pub static GOLD:                    Rgb<u8> = Rgb { r: 0xFF, g: 0xD7, b: 0x00 };
    pub static GOLDENROD:               Rgb<u8> = Rgb { r: 0xDA, g: 0xA5, b: 0x20 };
    pub static GRAY:                    Rgb<u8> = Rgb { r: 0x80, g: 0x80, b: 0x80 };
    pub static GREEN:                   Rgb<u8> = Rgb { r: 0x00, g: 0x80, b: 0x00 };
    pub static GREENYELLOW:             Rgb<u8> = Rgb { r: 0xAD, g: 0xFF, b: 0x2F };
    pub static HONEYDEW:                Rgb<u8> = Rgb { r: 0xF0, g: 0xFF, b: 0xF0 };
    pub static HOTPINK:                 Rgb<u8> = Rgb { r: 0xFF, g: 0x69, b: 0xB4 };
    pub static INDIANRED:               Rgb<u8> = Rgb { r: 0xCD, g: 0x5C, b: 0x5C };
    pub static INDIGO:                  Rgb<u8> = Rgb { r: 0x4B, g: 0x00, b: 0x82 };
    pub static IVORY:                   Rgb<u8> = Rgb { r: 0xFF, g: 0xFF, b: 0xF0 };
    pub static KHAKI:                   Rgb<u8> = Rgb { r: 0xF0, g: 0xE6, b: 0x8C };
    pub static LAVENDER:                Rgb<u8> = Rgb { r: 0xE6, g: 0xE6, b: 0xFA };
    pub static LAVENDERBLUSH:           Rgb<u8> = Rgb { r: 0xFF, g: 0xF0, b: 0xF5 };
    pub static LAWNGREEN:               Rgb<u8> = Rgb { r: 0x7C, g: 0xFC, b: 0x00 };
    pub static LEMONCHIFFON:            Rgb<u8> = Rgb { r: 0xFF, g: 0xFA, b: 0xCD };
    pub static LIGHTBLUE:               Rgb<u8> = Rgb { r: 0xAD, g: 0xD8, b: 0xE6 };
    pub static LIGHTCORAL:              Rgb<u8> = Rgb { r: 0xF0, g: 0x80, b: 0x80 };
    pub static LIGHTCYAN:               Rgb<u8> = Rgb { r: 0xE0, g: 0xFF, b: 0xFF };
    pub static LIGHTGOLDENRODYELLOW:    Rgb<u8> = Rgb { r: 0xFA, g: 0xFA, b: 0xD2 };
    pub static LIGHTGREEN:              Rgb<u8> = Rgb { r: 0x90, g: 0xEE, b: 0x90 };
    pub static LIGHTGREY:               Rgb<u8> = Rgb { r: 0xD3, g: 0xD3, b: 0xD3 };
    pub static LIGHTPINK:               Rgb<u8> = Rgb { r: 0xFF, g: 0xB6, b: 0xC1 };
    pub static LIGHTSALMON:             Rgb<u8> = Rgb { r: 0xFF, g: 0xA0, b: 0x7A };
    pub static LIGHTSEAGREEN:           Rgb<u8> = Rgb { r: 0x20, g: 0xB2, b: 0xAA };
    pub static LIGHTSKYBLUE:            Rgb<u8> = Rgb { r: 0x87, g: 0xCE, b: 0xFA };
    pub static LIGHTSLATEGRAY:          Rgb<u8> = Rgb { r: 0x77, g: 0x88, b: 0x99 };
    pub static LIGHTSTEELBLUE:          Rgb<u8> = Rgb { r: 0xB0, g: 0xC4, b: 0xDE };
    pub static LIGHTYELLOW:             Rgb<u8> = Rgb { r: 0xFF, g: 0xFF, b: 0xE0 };
    pub static LIME:                    Rgb<u8> = Rgb { r: 0x00, g: 0xFF, b: 0x00 };
    pub static LIMEGREEN:               Rgb<u8> = Rgb { r: 0x32, g: 0xCD, b: 0x32 };
    pub static LINEN:                   Rgb<u8> = Rgb { r: 0xFA, g: 0xF0, b: 0xE6 };
    pub static MAGENTA:                 Rgb<u8> = Rgb { r: 0xFF, g: 0x00, b: 0xFF };
    pub static MAROON:                  Rgb<u8> = Rgb { r: 0x80, g: 0x00, b: 0x00 };
    pub static MEDIUMAQUAMARINE:        Rgb<u8> = Rgb { r: 0x66, g: 0xCD, b: 0xAA };
    pub static MEDIUMBLUE:              Rgb<u8> = Rgb { r: 0x00, g: 0x00, b: 0xCD };
    pub static MEDIUMORCHID:            Rgb<u8> = Rgb { r: 0xBA, g: 0x55, b: 0xD3 };
    pub static MEDIUMPURPLE:            Rgb<u8> = Rgb { r: 0x93, g: 0x70, b: 0xDB };
    pub static MEDIUMSEAGREEN:          Rgb<u8> = Rgb { r: 0x3C, g: 0xB3, b: 0x71 };
    pub static MEDIUMSLATEBLUE:         Rgb<u8> = Rgb { r: 0x7B, g: 0x68, b: 0xEE };
    pub static MEDIUMSPRINGGREEN:       Rgb<u8> = Rgb { r: 0x00, g: 0xFA, b: 0x9A };
    pub static MEDIUMTURQUOISE:         Rgb<u8> = Rgb { r: 0x48, g: 0xD1, b: 0xCC };
    pub static MEDIUMVIOLETRED:         Rgb<u8> = Rgb { r: 0xC7, g: 0x15, b: 0x85 };
    pub static MIDNIGHTBLUE:            Rgb<u8> = Rgb { r: 0x19, g: 0x19, b: 0x70 };
    pub static MINTCREAM:               Rgb<u8> = Rgb { r: 0xF5, g: 0xFF, b: 0xFA };
    pub static MISTYROSE:               Rgb<u8> = Rgb { r: 0xFF, g: 0xE4, b: 0xE1 };
    pub static MOCCASIN:                Rgb<u8> = Rgb { r: 0xFF, g: 0xE4, b: 0xB5 };
    pub static NAVAJOWHITE:             Rgb<u8> = Rgb { r: 0xFF, g: 0xDE, b: 0xAD };
    pub static NAVY:                    Rgb<u8> = Rgb { r: 0x00, g: 0x00, b: 0x80 };
    pub static OLDLACE:                 Rgb<u8> = Rgb { r: 0xFD, g: 0xF5, b: 0xE6 };
    pub static OLIVE:                   Rgb<u8> = Rgb { r: 0x80, g: 0x80, b: 0x00 };
    pub static OLIVEDRAB:               Rgb<u8> = Rgb { r: 0x6B, g: 0x8E, b: 0x23 };
    pub static ORANGE:                  Rgb<u8> = Rgb { r: 0xFF, g: 0xA5, b: 0x00 };
    pub static ORANGERED:               Rgb<u8> = Rgb { r: 0xFF, g: 0x45, b: 0x00 };
    pub static ORCHID:                  Rgb<u8> = Rgb { r: 0xDA, g: 0x70, b: 0xD6 };
    pub static PALEGOLDENROD:           Rgb<u8> = Rgb { r: 0xEE, g: 0xE8, b: 0xAA };
    pub static PALEGREEN:               Rgb<u8> = Rgb { r: 0x98, g: 0xFB, b: 0x98 };
    pub static PALEVIOLETRED:           Rgb<u8> = Rgb { r: 0xDB, g: 0x70, b: 0x93 };
    pub static PAPAYAWHIP:              Rgb<u8> = Rgb { r: 0xFF, g: 0xEF, b: 0xD5 };
    pub static PEACHPUFF:               Rgb<u8> = Rgb { r: 0xFF, g: 0xDA, b: 0xB9 };
    pub static PERU:                    Rgb<u8> = Rgb { r: 0xCD, g: 0x85, b: 0x3F };
    pub static PINK:                    Rgb<u8> = Rgb { r: 0xFF, g: 0xC0, b: 0xCB };
    pub static PLUM:                    Rgb<u8> = Rgb { r: 0xDD, g: 0xA0, b: 0xDD };
    pub static POWDERBLUE:              Rgb<u8> = Rgb { r: 0xB0, g: 0xE0, b: 0xE6 };
    pub static PURPLE:                  Rgb<u8> = Rgb { r: 0x80, g: 0x00, b: 0x80 };
    pub static RED:                     Rgb<u8> = Rgb { r: 0xFF, g: 0x00, b: 0x00 };
    pub static ROSYBROWN:               Rgb<u8> = Rgb { r: 0xBC, g: 0x8F, b: 0x8F };
    pub static ROYALBLUE:               Rgb<u8> = Rgb { r: 0x41, g: 0x69, b: 0xE1 };
    pub static SADDLEBROWN:             Rgb<u8> = Rgb { r: 0x8B, g: 0x45, b: 0x13 };
    pub static SALMON:                  Rgb<u8> = Rgb { r: 0xFA, g: 0x80, b: 0x72 };
    pub static SANDYBROWN:              Rgb<u8> = Rgb { r: 0xFA, g: 0xA4, b: 0x60 };
    pub static SEAGREEN:                Rgb<u8> = Rgb { r: 0x2E, g: 0x8B, b: 0x57 };
    pub static SEASHELL:                Rgb<u8> = Rgb { r: 0xFF, g: 0xF5, b: 0xEE };
    pub static SIENNA:                  Rgb<u8> = Rgb { r: 0xA0, g: 0x52, b: 0x2D };
    pub static SILVER:                  Rgb<u8> = Rgb { r: 0xC0, g: 0xC0, b: 0xC0 };
    pub static SKYBLUE:                 Rgb<u8> = Rgb { r: 0x87, g: 0xCE, b: 0xEB };
    pub static SLATEBLUE:               Rgb<u8> = Rgb { r: 0x6A, g: 0x5A, b: 0xCD };
    pub static SLATEGRAY:               Rgb<u8> = Rgb { r: 0x70, g: 0x80, b: 0x90 };
    pub static SNOW:                    Rgb<u8> = Rgb { r: 0xFF, g: 0xFA, b: 0xFA };
    pub static SPRINGGREEN:             Rgb<u8> = Rgb { r: 0x00, g: 0xFF, b: 0x7F };
    pub static STEELBLUE:               Rgb<u8> = Rgb { r: 0x46, g: 0x82, b: 0xB4 };
    pub static TAN:                     Rgb<u8> = Rgb { r: 0xD2, g: 0xB4, b: 0x8C };
    pub static TEAL:                    Rgb<u8> = Rgb { r: 0x00, g: 0x80, b: 0x80 };
    pub static THISTLE:                 Rgb<u8> = Rgb { r: 0xD8, g: 0xBF, b: 0xD8 };
    pub static TOMATO:                  Rgb<u8> = Rgb { r: 0xFF, g: 0x63, b: 0x47 };
    pub static TURQUOISE:               Rgb<u8> = Rgb { r: 0x40, g: 0xE0, b: 0xD0 };
    pub static VIOLET:                  Rgb<u8> = Rgb { r: 0xEE, g: 0x82, b: 0xEE };
    pub static WHEAT:                   Rgb<u8> = Rgb { r: 0xF5, g: 0xDE, b: 0xB3 };
    pub static WHITE:                   Rgb<u8> = Rgb { r: 0xFF, g: 0xFF, b: 0xFF };
    pub static WHITESMOKE:              Rgb<u8> = Rgb { r: 0xF5, g: 0xF5, b: 0xF5 };
    pub static YELLOW:                  Rgb<u8> = Rgb { r: 0xFF, g: 0xFF, b: 0x00 };
    pub static YELLOWGREEN:             Rgb<u8> = Rgb { r: 0x9A, g: 0xCD, b: 0x32 };
}

#[cfg(test)]
mod tests {
    use {Hsv, ToHsv};
    use {Rgb, ToRgb};

    #[test]
    fn test_rgb_to_rgb() {
        assert_eq!(Rgb::<u8>::new(0xA0, 0xA0, 0xA0).to_rgb::<u8>(), Rgb::<u8>::new(0xA0, 0xA0, 0xA0));
        assert_eq!(Rgb::<u8>::new(0xA0, 0xA0, 0xA0).to_rgb::<u16>(), Rgb::<u16>::new(0xA0A0, 0xA0A0, 0xA0A0));
    }

    #[test]
    fn test_rgb_to_hsv() {
        assert_eq!(Rgb::<u8>::new(0xFF, 0xFF, 0xFF).to_hsv::<f32>(), Hsv::<f32>::new(0.0, 0.0, 1.0));
        assert_eq!(Rgb::<u8>::new(0x99, 0x00, 0x00).to_hsv::<f32>(), Hsv::<f32>::new(0.0, 1.0, 0.6));
        assert_eq!(Rgb::<u8>::new(0x00, 0x99, 0x00).to_hsv::<f32>(), Hsv::<f32>::new(120.0, 1.0, 0.6));
        assert_eq!(Rgb::<u8>::new(0x00, 0x00, 0x99).to_hsv::<f32>(), Hsv::<f32>::new(240.0, 1.0, 0.6));
    }
}
