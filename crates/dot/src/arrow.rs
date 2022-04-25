
/// This structure holds all information that can describe an arrow connected to
/// either start or end of an edge.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Arrow {
    pub arrows: Vec<ArrowShape>,
}

use self::ArrowShape::*;

impl Arrow {
    /// Return `true` if this is a default arrow.
    pub fn is_default(&self) -> bool {
        self.arrows.is_empty()
    }

    /// Arrow constructor which returns a default arrow
    pub fn default() -> Arrow {
        Arrow {
            arrows: vec![],
        }
    }

    /// Arrow constructor which returns an empty arrow
    pub fn none() -> Arrow {
        Arrow {
            arrows: vec![NoArrow],
        }
    }

    /// Arrow constructor which returns a regular triangle arrow, without modifiers
    pub fn normal() -> Arrow {
        Arrow {
            arrows: vec![ArrowShape::normal()]
        }
    }

    /// Arrow constructor which returns an arrow created by a given ArrowShape.
    pub fn from_arrow(arrow: ArrowShape) -> Arrow {
        Arrow {
            arrows: vec![arrow],
        }
    }

    /// Function which converts given arrow into a renderable form.
    pub fn to_dot_string(&self) -> String {
        let mut cow = String::new();
        for arrow in &self.arrows {
            cow.push_str(&arrow.to_dot_string());
        };
        cow
    }
}


impl Into<Arrow> for [ArrowShape; 2] {
    fn into(self) -> Arrow {
        Arrow {
            arrows: vec![self[0], self[1]],
        }
    }
}
impl Into<Arrow> for [ArrowShape; 3] {
    fn into(self) -> Arrow {
        Arrow {
            arrows: vec![self[0], self[1], self[2]],
        }
    }
}
impl Into<Arrow> for [ArrowShape; 4] {
    fn into(self) -> Arrow {
        Arrow {
            arrows: vec![self[0], self[1], self[2], self[3]],
        }
    }
}

/// Arrow modifier that determines if the shape is empty or filled.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Fill {
    Open,
    Filled,
}

impl Fill {
    pub fn as_slice(self) -> &'static str {
        match self {
            Fill::Open => "o",
            Fill::Filled => "",
        }
    }
}

/// Arrow modifier that determines if the shape is clipped.
/// For example `Side::Left` means only left side is visible.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
    Both,
}

impl Side {
    pub fn as_slice(self) -> &'static str {
        match self {
            Side::Left  => "l",
            Side::Right => "r",
            Side::Both  => "",
        }
    }
}


/// This enumeration represents all possible arrow edge
/// as defined in [grapviz documentation](http://www.graphviz.org/content/arrow-shapes).
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum ArrowShape {
    /// No arrow will be displayed
    NoArrow,
    /// Arrow that ends in a triangle. Basically a normal arrow.
    /// NOTE: there is error in official documentation, this supports both fill and side clipping
    Normal(Fill, Side),
    /// Arrow ending in a small square box
    Box(Fill, Side),
    /// Arrow ending in a three branching lines also called crow's foot
    Crow(Side),
    /// Arrow ending in a curve
    Curve(Side),
    /// Arrow ending in an inverted curve
    ICurve(Fill, Side),
    /// Arrow ending in an diamond shaped rectangular shape.
    Diamond(Fill, Side),
    /// Arrow ending in a circle.
    Dot(Fill),
    /// Arrow ending in an inverted triangle.
    Inv(Fill, Side),
    /// Arrow ending with a T shaped arrow.
    Tee(Side),
    /// Arrow ending with a V shaped arrow.
    Vee(Side),
}
impl ArrowShape {
    /// Constructor which returns no arrow.
    pub fn none() -> ArrowShape {
        ArrowShape::NoArrow
    }

    /// Constructor which returns normal arrow.
    pub fn normal() -> ArrowShape {
        ArrowShape::Normal(Fill::Filled, Side::Both)
    }

    /// Constructor which returns a regular box arrow.
    pub fn boxed() -> ArrowShape {
        ArrowShape::Box(Fill::Filled, Side::Both)
    }

    /// Constructor which returns a regular crow arrow.
    pub fn crow() -> ArrowShape {
        ArrowShape::Crow(Side::Both)
    }

    /// Constructor which returns a regular curve arrow.
    pub fn curve() -> ArrowShape {
        ArrowShape::Curve(Side::Both)
    }

    /// Constructor which returns an inverted curve arrow.
    pub fn icurve() -> ArrowShape {
        ArrowShape::ICurve(Fill::Filled, Side::Both)
    }

    /// Constructor which returns a diamond arrow.
    pub fn diamond() -> ArrowShape {
        ArrowShape::Diamond(Fill::Filled, Side::Both)
    }

    /// Constructor which returns a circle shaped arrow.
    pub fn dot() -> ArrowShape {
        ArrowShape::Diamond(Fill::Filled, Side::Both)
    }

    /// Constructor which returns an inverted triangle arrow.
    pub fn inv() -> ArrowShape {
        ArrowShape::Inv(Fill::Filled, Side::Both)
    }

    /// Constructor which returns a T shaped arrow.
    pub fn tee() -> ArrowShape {
        ArrowShape::Tee(Side::Both)
    }

    /// Constructor which returns a V shaped arrow.
    pub fn vee() -> ArrowShape {
        ArrowShape::Vee(Side::Both)
    }

    /// Function which renders given ArrowShape into a String for displaying.
    pub fn to_dot_string(&self) -> String {
        let mut res = String::new();
        match *self {
            Box(fill, side) | ICurve(fill, side)| Diamond(fill, side) |
            Inv(fill, side) | Normal(fill, side)=> {
                res.push_str(fill.as_slice());
                match side {
                    Side::Left | Side::Right => res.push_str(side.as_slice()),
                    Side::Both => {},
                };
            },
            Dot(fill)       => res.push_str(fill.as_slice()),
            Crow(side) | Curve(side) | Tee(side)
            | Vee(side) => {
                match side {
                    Side::Left | Side::Right => res.push_str(side.as_slice()),
                    Side::Both => {},
                }
            }
            NoArrow => {},
        };
        match *self {
            NoArrow         => res.push_str("none"),
            Normal(_, _)    => res.push_str("normal"),
            Box(_, _)       => res.push_str("box"),
            Crow(_)         => res.push_str("crow"),
            Curve(_)        => res.push_str("curve"),
            ICurve(_, _)    => res.push_str("icurve"),
            Diamond(_, _)   => res.push_str("diamond"),
            Dot(_)          => res.push_str("dot"),
            Inv(_, _)       => res.push_str("inv"),
            Tee(_)          => res.push_str("tee"),
            Vee(_)          => res.push_str("vee"),
        };
        res
    }
}