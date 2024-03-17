//! [Extended colors from the CSS4 specification](https://en.wikipedia.org/wiki/Web_colors#Extended_colors),
//! Also known as X11 colors, which were standardized in HTML 4.0.

use crate::Srgba;

// The CSS4 colors are a superset of the CSS1 colors, so we can just re-export the CSS1 colors.
#[allow(unused_imports)]
pub use crate::palettes::basic::*;

/// <div style="background-color:rgb(94.1%, 97.3%, 100.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const ALICE_BLUE: Srgba = Srgba::new(0.941, 0.973, 1.0, 1.0);
/// <div style="background-color:rgb(98.0%, 92.2%, 84.3%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const ANTIQUE_WHITE: Srgba = Srgba::new(0.98, 0.922, 0.843, 1.0);
/// <div style="background-color:rgb(0.0%, 100.0%, 100.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const AQUA: Srgba = Srgba::new(0.0, 1.0, 1.0, 1.0);
/// <div style="background-color:rgb(49.8%, 100.0%, 83.1%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const AQUAMARINE: Srgba = Srgba::new(0.498, 1.0, 0.831, 1.0);
/// <div style="background-color:rgb(94.1%, 100.0%, 100.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const AZURE: Srgba = Srgba::new(0.941, 1.0, 1.0, 1.0);
/// <div style="background-color:rgb(96.1%, 96.1%, 86.3%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const BEIGE: Srgba = Srgba::new(0.961, 0.961, 0.863, 1.0);
/// <div style="background-color:rgb(100.0%, 89.4%, 76.9%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const BISQUE: Srgba = Srgba::new(1.0, 0.894, 0.769, 1.0);
/// <div style="background-color:rgb(100.0%, 92.2%, 80.4%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const BLANCHED_ALMOND: Srgba = Srgba::new(1.0, 0.922, 0.804, 1.0);
/// <div style="background-color:rgb(54.1%, 16.900000000000002%, 88.6%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const BLUE_VIOLET: Srgba = Srgba::new(0.541, 0.169, 0.886, 1.0);
/// <div style="background-color:rgb(64.7%, 16.5%, 16.5%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const BROWN: Srgba = Srgba::new(0.647, 0.165, 0.165, 1.0);
/// <div style="background-color:rgb(87.1%, 72.2%, 52.900000000000006%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const BURLYWOOD: Srgba = Srgba::new(0.871, 0.722, 0.529, 1.0);
/// <div style="background-color:rgb(37.3%, 62.0%, 62.7%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const CADET_BLUE: Srgba = Srgba::new(0.373, 0.62, 0.627, 1.0);
/// <div style="background-color:rgb(49.8%, 100.0%, 0.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const CHARTREUSE: Srgba = Srgba::new(0.498, 1.0, 0.0, 1.0);
/// <div style="background-color:rgb(82.39999999999999%, 41.199999999999996%, 11.799999999999999%);
/// width: 10px; padding: 10px; border: 1px solid;"></div>
pub const CHOCOLATE: Srgba = Srgba::new(0.824, 0.412, 0.118, 1.0);
/// <div style="background-color:rgb(100.0%, 49.8%, 31.4%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const CORAL: Srgba = Srgba::new(1.0, 0.498, 0.314, 1.0);
/// <div style="background-color:rgb(39.2%, 58.4%, 92.9%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const CORNFLOWER_BLUE: Srgba = Srgba::new(0.392, 0.584, 0.929, 1.0);
/// <div style="background-color:rgb(100.0%, 97.3%, 86.3%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const CORNSILK: Srgba = Srgba::new(1.0, 0.973, 0.863, 1.0);
/// <div style="background-color:rgb(86.3%, 7.8%, 23.5%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const CRIMSON: Srgba = Srgba::new(0.863, 0.078, 0.235, 1.0);
/// <div style="background-color:rgb(0.0%, 0.0%, 54.50000000000001%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const DARK_BLUE: Srgba = Srgba::new(0.0, 0.0, 0.545, 1.0);
/// <div style="background-color:rgb(0.0%, 54.50000000000001%, 54.50000000000001%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const DARK_CYAN: Srgba = Srgba::new(0.0, 0.545, 0.545, 1.0);
/// <div style="background-color:rgb(72.2%, 52.5%, 4.3%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DARK_GOLDENROD: Srgba = Srgba::new(0.722, 0.525, 0.043, 1.0);
/// <div style="background-color:rgb(66.3%, 66.3%, 66.3%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DARK_GRAY: Srgba = Srgba::new(0.663, 0.663, 0.663, 1.0);
/// <div style="background-color:rgb(0.0%, 39.2%, 0.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DARK_GREEN: Srgba = Srgba::new(0.0, 0.392, 0.0, 1.0);
/// <div style="background-color:rgb(66.3%, 66.3%, 66.3%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DARK_GREY: Srgba = Srgba::new(0.663, 0.663, 0.663, 1.0);
/// <div style="background-color:rgb(74.1%, 71.8%, 42.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DARK_KHAKI: Srgba = Srgba::new(0.741, 0.718, 0.42, 1.0);
/// <div style="background-color:rgb(54.50000000000001%, 0.0%, 54.50000000000001%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const DARK_MAGENTA: Srgba = Srgba::new(0.545, 0.0, 0.545, 1.0);
/// <div style="background-color:rgb(33.300000000000004%, 42.0%, 18.4%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const DARK_OLIVEGREEN: Srgba = Srgba::new(0.333, 0.42, 0.184, 1.0);
/// <div style="background-color:rgb(100.0%, 54.900000000000006%, 0.0%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const DARK_ORANGE: Srgba = Srgba::new(1.0, 0.549, 0.0, 1.0);
/// <div style="background-color:rgb(60.0%, 19.6%, 80.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DARK_ORCHID: Srgba = Srgba::new(0.6, 0.196, 0.8, 1.0);
/// <div style="background-color:rgb(54.50000000000001%, 0.0%, 0.0%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const DARK_RED: Srgba = Srgba::new(0.545, 0.0, 0.0, 1.0);
/// <div style="background-color:rgb(91.4%, 58.8%, 47.8%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DARK_SALMON: Srgba = Srgba::new(0.914, 0.588, 0.478, 1.0);
/// <div style="background-color:rgb(56.10000000000001%, 73.7%, 56.10000000000001%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const DARK_SEA_GREEN: Srgba = Srgba::new(0.561, 0.737, 0.561, 1.0);
/// <div style="background-color:rgb(28.199999999999996%, 23.9%, 54.50000000000001%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const DARK_SLATE_BLUE: Srgba = Srgba::new(0.282, 0.239, 0.545, 1.0);
/// <div style="background-color:rgb(18.4%, 31.0%, 31.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DARK_SLATE_GRAY: Srgba = Srgba::new(0.184, 0.31, 0.31, 1.0);
/// <div style="background-color:rgb(18.4%, 31.0%, 31.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DARK_SLATE_GREY: Srgba = Srgba::new(0.184, 0.31, 0.31, 1.0);
/// <div style="background-color:rgb(0.0%, 80.80000000000001%, 82.0%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const DARK_TURQUOISE: Srgba = Srgba::new(0.0, 0.808, 0.82, 1.0);
/// <div style="background-color:rgb(57.99999999999999%, 0.0%, 82.69999999999999%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const DARK_VIOLET: Srgba = Srgba::new(0.58, 0.0, 0.827, 1.0);
/// <div style="background-color:rgb(100.0%, 7.8%, 57.599999999999994%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const DEEP_PINK: Srgba = Srgba::new(1.0, 0.078, 0.576, 1.0);
/// <div style="background-color:rgb(0.0%, 74.9%, 100.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const DEEP_SKY_BLUE: Srgba = Srgba::new(0.0, 0.749, 1.0, 1.0);
/// <div style="background-color:rgb(41.199999999999996%, 41.199999999999996%, 41.199999999999996%);
/// width: 10px; padding: 10px; border: 1px solid;"></div>
pub const DIM_GRAY: Srgba = Srgba::new(0.412, 0.412, 0.412, 1.0);
/// <div style="background-color:rgb(41.199999999999996%, 41.199999999999996%, 41.199999999999996%);
/// width: 10px; padding: 10px; border: 1px solid;"></div>
pub const DIM_GREY: Srgba = Srgba::new(0.412, 0.412, 0.412, 1.0);
/// <div style="background-color:rgb(11.799999999999999%, 56.49999999999999%, 100.0%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const DODGER_BLUE: Srgba = Srgba::new(0.118, 0.565, 1.0, 1.0);
/// <div style="background-color:rgb(69.8%, 13.3%, 13.3%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const FIRE_BRICK: Srgba = Srgba::new(0.698, 0.133, 0.133, 1.0);
/// <div style="background-color:rgb(100.0%, 98.0%, 94.1%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const FLORAL_WHITE: Srgba = Srgba::new(1.0, 0.98, 0.941, 1.0);
/// <div style="background-color:rgb(13.3%, 54.50000000000001%, 13.3%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const FOREST_GREEN: Srgba = Srgba::new(0.133, 0.545, 0.133, 1.0);
/// <div style="background-color:rgb(86.3%, 86.3%, 86.3%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const GAINSBORO: Srgba = Srgba::new(0.863, 0.863, 0.863, 1.0);
/// <div style="background-color:rgb(97.3%, 97.3%, 100.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const GHOST_WHITE: Srgba = Srgba::new(0.973, 0.973, 1.0, 1.0);
/// <div style="background-color:rgb(100.0%, 84.3%, 0.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const GOLD: Srgba = Srgba::new(1.0, 0.843, 0.0, 1.0);
/// <div style="background-color:rgb(85.5%, 64.7%, 12.5%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const GOLDENROD: Srgba = Srgba::new(0.855, 0.647, 0.125, 1.0);
/// <div style="background-color:rgb(0.0%, 50.2%, 0.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const GREEN_YELLOW: Srgba = Srgba::new(0.678, 1.0, 0.184, 1.0);
/// <div style="background-color:rgb(50.2%, 50.2%, 50.2%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const GREY: Srgba = Srgba::new(0.502, 0.502, 0.502, 1.0);
/// <div style="background-color:rgb(94.1%, 100.0%, 94.1%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const HONEYDEW: Srgba = Srgba::new(0.941, 1.0, 0.941, 1.0);
/// <div style="background-color:rgb(100.0%, 41.199999999999996%, 70.6%); width: 10px; padding:
/// 10px; border: 1px solid;"></div>
pub const HOT_PINK: Srgba = Srgba::new(1.0, 0.412, 0.706, 1.0);
/// <div style="background-color:rgb(80.4%, 36.1%, 36.1%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const INDIAN_RED: Srgba = Srgba::new(0.804, 0.361, 0.361, 1.0);
/// <div style="background-color:rgb(29.4%, 0.0%, 51.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const INDIGO: Srgba = Srgba::new(0.294, 0.0, 0.51, 1.0);
/// <div style="background-color:rgb(100.0%, 100.0%, 94.1%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const IVORY: Srgba = Srgba::new(1.0, 1.0, 0.941, 1.0);
/// <div style="background-color:rgb(94.1%, 90.2%, 54.900000000000006%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const KHAKI: Srgba = Srgba::new(0.941, 0.902, 0.549, 1.0);
/// <div style="background-color:rgb(90.2%, 90.2%, 98.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LAVENDER: Srgba = Srgba::new(0.902, 0.902, 0.98, 1.0);
/// <div style="background-color:rgb(100.0%, 94.1%, 96.1%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LAVENDER_BLUSH: Srgba = Srgba::new(1.0, 0.941, 0.961, 1.0);
/// <div style="background-color:rgb(48.6%, 98.8%, 0.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LAWN_GREEN: Srgba = Srgba::new(0.486, 0.988, 0.0, 1.0);
/// <div style="background-color:rgb(100.0%, 98.0%, 80.4%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LEMON_CHIFFON: Srgba = Srgba::new(1.0, 0.98, 0.804, 1.0);
/// <div style="background-color:rgb(67.80000000000001%, 84.7%, 90.2%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const LIGHT_BLUE: Srgba = Srgba::new(0.678, 0.847, 0.902, 1.0);
/// <div style="background-color:rgb(94.1%, 50.2%, 50.2%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LIGHT_CORAL: Srgba = Srgba::new(0.941, 0.502, 0.502, 1.0);
/// <div style="background-color:rgb(87.8%, 100.0%, 100.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LIGHT_CYAN: Srgba = Srgba::new(0.878, 1.0, 1.0, 1.0);
/// <div style="background-color:rgb(98.0%, 98.0%, 82.39999999999999%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const LIGHT_GOLDENROD_YELLOW: Srgba = Srgba::new(0.98, 0.98, 0.824, 1.0);
/// <div style="background-color:rgb(82.69999999999999%, 82.69999999999999%, 82.69999999999999%);
/// width: 10px; padding: 10px; border: 1px solid;"></div>
pub const LIGHT_GRAY: Srgba = Srgba::new(0.827, 0.827, 0.827, 1.0);
/// <div style="background-color:rgb(56.49999999999999%, 93.30000000000001%, 56.49999999999999%);
/// width: 10px; padding: 10px; border: 1px solid;"></div>
pub const LIGHT_GREEN: Srgba = Srgba::new(0.565, 0.933, 0.565, 1.0);
/// <div style="background-color:rgb(82.69999999999999%, 82.69999999999999%, 82.69999999999999%);
/// width: 10px; padding: 10px; border: 1px solid;"></div>
pub const LIGHT_GREY: Srgba = Srgba::new(0.827, 0.827, 0.827, 1.0);
/// <div style="background-color:rgb(100.0%, 71.39999999999999%, 75.7%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const LIGHT_PINK: Srgba = Srgba::new(1.0, 0.714, 0.757, 1.0);
/// <div style="background-color:rgb(100.0%, 62.7%, 47.8%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LIGHT_SALMON: Srgba = Srgba::new(1.0, 0.627, 0.478, 1.0);
/// <div style="background-color:rgb(12.5%, 69.8%, 66.7%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LIGHT_SEA_GREEN: Srgba = Srgba::new(0.125, 0.698, 0.667, 1.0);
/// <div style="background-color:rgb(52.900000000000006%, 80.80000000000001%, 98.0%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const LIGHT_SKY_BLUE: Srgba = Srgba::new(0.529, 0.808, 0.98, 1.0);
/// <div style="background-color:rgb(46.7%, 53.300000000000004%, 60.0%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const LIGHT_SLATE_GRAY: Srgba = Srgba::new(0.467, 0.533, 0.6, 1.0);
/// <div style="background-color:rgb(46.7%, 53.300000000000004%, 60.0%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const LIGHT_SLATE_GREY: Srgba = Srgba::new(0.467, 0.533, 0.6, 1.0);
/// <div style="background-color:rgb(69.0%, 76.9%, 87.1%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LIGHT_STEEL_BLUE: Srgba = Srgba::new(0.69, 0.769, 0.871, 1.0);
/// <div style="background-color:rgb(100.0%, 100.0%, 87.8%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LIGHT_YELLOW: Srgba = Srgba::new(1.0, 1.0, 0.878, 1.0);
/// <div style="background-color:rgb(19.6%, 80.4%, 19.6%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LIMEGREEN: Srgba = Srgba::new(0.196, 0.804, 0.196, 1.0);
/// <div style="background-color:rgb(98.0%, 94.1%, 90.2%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const LINEN: Srgba = Srgba::new(0.98, 0.941, 0.902, 1.0);
/// <div style="background-color:rgb(100.0%, 0.0%, 100.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const MAGENTA: Srgba = Srgba::new(1.0, 0.0, 1.0, 1.0);
/// <div style="background-color:rgb(40.0%, 80.4%, 66.7%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const MEDIUM_AQUAMARINE: Srgba = Srgba::new(0.4, 0.804, 0.667, 1.0);
/// <div style="background-color:rgb(0.0%, 0.0%, 80.4%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const MEDIUM_BLUE: Srgba = Srgba::new(0.0, 0.0, 0.804, 1.0);
/// <div style="background-color:rgb(72.89999999999999%, 33.300000000000004%, 82.69999999999999%);
/// width: 10px; padding: 10px; border: 1px solid;"></div>
pub const MEDIUM_ORCHID: Srgba = Srgba::new(0.729, 0.333, 0.827, 1.0);
/// <div style="background-color:rgb(57.599999999999994%, 43.9%, 85.9%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const MEDIUM_PURPLE: Srgba = Srgba::new(0.576, 0.439, 0.859, 1.0);
/// <div style="background-color:rgb(23.5%, 70.19999999999999%, 44.3%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const MEDIUM_SEA_GREEN: Srgba = Srgba::new(0.235, 0.702, 0.443, 1.0);
/// <div style="background-color:rgb(48.199999999999996%, 40.8%, 93.30000000000001%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const MEDIUM_SLATE_BLUE: Srgba = Srgba::new(0.482, 0.408, 0.933, 1.0);
/// <div style="background-color:rgb(0.0%, 98.0%, 60.4%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const MEDIUM_SPRING_GREEN: Srgba = Srgba::new(0.0, 0.98, 0.604, 1.0);
/// <div style="background-color:rgb(28.199999999999996%, 82.0%, 80.0%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const MEDIUM_TURQUOISE: Srgba = Srgba::new(0.282, 0.82, 0.8, 1.0);
/// <div style="background-color:rgb(78.0%, 8.200000000000001%, 52.2%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const MEDIUM_VIOLET_RED: Srgba = Srgba::new(0.78, 0.082, 0.522, 1.0);
/// <div style="background-color:rgb(9.8%, 9.8%, 43.9%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const MIDNIGHT_BLUE: Srgba = Srgba::new(0.098, 0.098, 0.439, 1.0);
/// <div style="background-color:rgb(96.1%, 100.0%, 98.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const MINT_CREAM: Srgba = Srgba::new(0.961, 1.0, 0.98, 1.0);
/// <div style="background-color:rgb(100.0%, 89.4%, 88.2%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const MISTY_ROSE: Srgba = Srgba::new(1.0, 0.894, 0.882, 1.0);
/// <div style="background-color:rgb(100.0%, 89.4%, 71.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const MOCCASIN: Srgba = Srgba::new(1.0, 0.894, 0.71, 1.0);
/// <div style="background-color:rgb(100.0%, 87.1%, 67.80000000000001%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const NAVAJO_WHITE: Srgba = Srgba::new(1.0, 0.871, 0.678, 1.0);
/// <div style="background-color:rgb(99.2%, 96.1%, 90.2%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const OLD_LACE: Srgba = Srgba::new(0.992, 0.961, 0.902, 1.0);
/// <div style="background-color:rgb(42.0%, 55.7%, 13.700000000000001%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const OLIVE_DRAB: Srgba = Srgba::new(0.42, 0.557, 0.137, 1.0);
/// <div style="background-color:rgb(100.0%, 64.7%, 0.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const ORANGE: Srgba = Srgba::new(1.0, 0.647, 0.0, 1.0);
/// <div style="background-color:rgb(100.0%, 27.1%, 0.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const ORANGE_RED: Srgba = Srgba::new(1.0, 0.271, 0.0, 1.0);
/// <div style="background-color:rgb(85.5%, 43.9%, 83.89999999999999%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const ORCHID: Srgba = Srgba::new(0.855, 0.439, 0.839, 1.0);
/// <div style="background-color:rgb(93.30000000000001%, 91.0%, 66.7%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const PALE_GOLDENROD: Srgba = Srgba::new(0.933, 0.91, 0.667, 1.0);
/// <div style="background-color:rgb(59.599999999999994%, 98.4%, 59.599999999999994%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const PALE_GREEN: Srgba = Srgba::new(0.596, 0.984, 0.596, 1.0);
/// <div style="background-color:rgb(68.60000000000001%, 93.30000000000001%, 93.30000000000001%);
/// width: 10px; padding: 10px; border: 1px solid;"></div>
pub const PALE_TURQUOISE: Srgba = Srgba::new(0.686, 0.933, 0.933, 1.0);
/// <div style="background-color:rgb(85.9%, 43.9%, 57.599999999999994%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const PALE_VIOLETRED: Srgba = Srgba::new(0.859, 0.439, 0.576, 1.0);
/// <div style="background-color:rgb(100.0%, 93.7%, 83.5%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const PAPAYA_WHIP: Srgba = Srgba::new(1.0, 0.937, 0.835, 1.0);
/// <div style="background-color:rgb(100.0%, 85.5%, 72.5%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const PEACHPUFF: Srgba = Srgba::new(1.0, 0.855, 0.725, 1.0);
/// <div style="background-color:rgb(80.4%, 52.2%, 24.7%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const PERU: Srgba = Srgba::new(0.804, 0.522, 0.247, 1.0);
/// <div style="background-color:rgb(100.0%, 75.3%, 79.60000000000001%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const PINK: Srgba = Srgba::new(1.0, 0.753, 0.796, 1.0);
/// <div style="background-color:rgb(86.7%, 62.7%, 86.7%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const PLUM: Srgba = Srgba::new(0.867, 0.627, 0.867, 1.0);
/// <div style="background-color:rgb(69.0%, 87.8%, 90.2%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const POWDER_BLUE: Srgba = Srgba::new(0.69, 0.878, 0.902, 1.0);
/// <div style="background-color:rgb(40.0%, 20.0%, 60.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const REBECCA_PURPLE: Srgba = Srgba::new(0.4, 0.2, 0.6, 1.0);
/// <div style="background-color:rgb(73.7%, 56.10000000000001%, 56.10000000000001%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const ROSY_BROWN: Srgba = Srgba::new(0.737, 0.561, 0.561, 1.0);
/// <div style="background-color:rgb(25.5%, 41.199999999999996%, 88.2%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const ROYAL_BLUE: Srgba = Srgba::new(0.255, 0.412, 0.882, 1.0);
/// <div style="background-color:rgb(54.50000000000001%, 27.1%, 7.5%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const SADDLE_BROWN: Srgba = Srgba::new(0.545, 0.271, 0.075, 1.0);
/// <div style="background-color:rgb(98.0%, 50.2%, 44.7%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const SALMON: Srgba = Srgba::new(0.98, 0.502, 0.447, 1.0);
/// <div style="background-color:rgb(95.7%, 64.3%, 37.6%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const SANDY_BROWN: Srgba = Srgba::new(0.957, 0.643, 0.376, 1.0);
/// <div style="background-color:rgb(18.0%, 54.50000000000001%, 34.1%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const SEA_GREEN: Srgba = Srgba::new(0.18, 0.545, 0.341, 1.0);
/// <div style="background-color:rgb(100.0%, 96.1%, 93.30000000000001%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const SEASHELL: Srgba = Srgba::new(1.0, 0.961, 0.933, 1.0);
/// <div style="background-color:rgb(62.7%, 32.2%, 17.599999999999998%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const SIENNA: Srgba = Srgba::new(0.627, 0.322, 0.176, 1.0);
/// <div style="background-color:rgb(52.900000000000006%, 80.80000000000001%, 92.2%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const SKY_BLUE: Srgba = Srgba::new(0.529, 0.808, 0.922, 1.0);
/// <div style="background-color:rgb(41.6%, 35.3%, 80.4%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const SLATE_BLUE: Srgba = Srgba::new(0.416, 0.353, 0.804, 1.0);
/// <div style="background-color:rgb(43.9%, 50.2%, 56.49999999999999%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const SLATE_GRAY: Srgba = Srgba::new(0.439, 0.502, 0.565, 1.0);
/// <div style="background-color:rgb(43.9%, 50.2%, 56.49999999999999%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const SLATE_GREY: Srgba = Srgba::new(0.439, 0.502, 0.565, 1.0);
/// <div style="background-color:rgb(100.0%, 98.0%, 98.0%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const SNOW: Srgba = Srgba::new(1.0, 0.98, 0.98, 1.0);
/// <div style="background-color:rgb(0.0%, 100.0%, 49.8%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const SPRING_GREEN: Srgba = Srgba::new(0.0, 1.0, 0.498, 1.0);
/// <div style="background-color:rgb(27.500000000000004%, 51.0%, 70.6%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const STEEL_BLUE: Srgba = Srgba::new(0.275, 0.51, 0.706, 1.0);
/// <div style="background-color:rgb(82.39999999999999%, 70.6%, 54.900000000000006%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const TAN: Srgba = Srgba::new(0.824, 0.706, 0.549, 1.0);
/// <div style="background-color:rgb(84.7%, 74.9%, 84.7%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const THISTLE: Srgba = Srgba::new(0.847, 0.749, 0.847, 1.0);
/// <div style="background-color:rgb(100.0%, 38.800000000000004%, 27.800000000000004%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const TOMATO: Srgba = Srgba::new(1.0, 0.388, 0.278, 1.0);
/// <div style="background-color:rgb(25.1%, 87.8%, 81.6%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const TURQUOISE: Srgba = Srgba::new(0.251, 0.878, 0.816, 1.0);
/// <div style="background-color:rgb(93.30000000000001%, 51.0%, 93.30000000000001%); width: 10px;
/// padding: 10px; border: 1px solid;"></div>
pub const VIOLET: Srgba = Srgba::new(0.933, 0.51, 0.933, 1.0);
/// <div style="background-color:rgb(96.1%, 87.1%, 70.19999999999999%); width: 10px; padding: 10px;
/// border: 1px solid;"></div>
pub const WHEAT: Srgba = Srgba::new(0.961, 0.871, 0.702, 1.0);
/// <div style="background-color:rgb(96.1%, 96.1%, 96.1%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const WHITE_SMOKE: Srgba = Srgba::new(0.961, 0.961, 0.961, 1.0);
/// <div style="background-color:rgb(60.4%, 80.4%, 19.6%); width: 10px; padding: 10px; border: 1px
/// solid;"></div>
pub const YELLOW_GREEN: Srgba = Srgba::new(0.604, 0.804, 0.196, 1.0);
