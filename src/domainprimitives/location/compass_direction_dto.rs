use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum CompassDirection {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl CompassDirection {
    pub fn get_opposite_direction(&self) -> CompassDirection {
        match self {
            CompassDirection::NORTH => CompassDirection::SOUTH,
            CompassDirection::EAST => CompassDirection::WEST,
            CompassDirection::SOUTH => CompassDirection::NORTH,
            CompassDirection::WEST => CompassDirection::EAST,
        }
    }

    pub fn x_offset(&self) -> i8 {
        match self {
            CompassDirection::NORTH => 0,
            CompassDirection::EAST => 1,
            CompassDirection::SOUTH => 0,
            CompassDirection::WEST => -1,
        }
    }

    pub fn y_offset(&self) -> i8 {
        match self {
            CompassDirection::NORTH => -1,
            CompassDirection::EAST => 0,
            CompassDirection::SOUTH => 1,
            CompassDirection::WEST => 0,
        }
    }

    pub fn ninety_degrees_clockwise(&self) -> CompassDirection {
        match self {
            CompassDirection::NORTH => CompassDirection::EAST,
            CompassDirection::EAST => CompassDirection::SOUTH,
            CompassDirection::SOUTH => CompassDirection::WEST,
            CompassDirection::WEST => CompassDirection::NORTH,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_opposite_direction() {
        assert_eq!(
            CompassDirection::NORTH.get_opposite_direction(),
            CompassDirection::SOUTH
        );
        assert_eq!(
            CompassDirection::EAST.get_opposite_direction(),
            CompassDirection::WEST
        );
        assert_eq!(
            CompassDirection::SOUTH.get_opposite_direction(),
            CompassDirection::NORTH
        );
        assert_eq!(
            CompassDirection::WEST.get_opposite_direction(),
            CompassDirection::EAST
        );
    }

    #[test]
    fn test_x_offset() {
        assert_eq!(CompassDirection::NORTH.x_offset(), 0);
        assert_eq!(CompassDirection::EAST.x_offset(), 1);
        assert_eq!(CompassDirection::SOUTH.x_offset(), 0);
        assert_eq!(CompassDirection::WEST.x_offset(), -1);
    }

    #[test]
    fn test_y_offset() {
        assert_eq!(CompassDirection::NORTH.y_offset(), -1);
        assert_eq!(CompassDirection::EAST.y_offset(), 0);
        assert_eq!(CompassDirection::SOUTH.y_offset(), 1);
        assert_eq!(CompassDirection::WEST.y_offset(), 0);
    }

    #[test]
    fn test_ninety_degrees_clockwise() {
        assert_eq!(
            CompassDirection::NORTH.ninety_degrees_clockwise(),
            CompassDirection::EAST
        );
        assert_eq!(
            CompassDirection::EAST.ninety_degrees_clockwise(),
            CompassDirection::SOUTH
        );
        assert_eq!(
            CompassDirection::SOUTH.ninety_degrees_clockwise(),
            CompassDirection::WEST
        );
        assert_eq!(
            CompassDirection::WEST.ninety_degrees_clockwise(),
            CompassDirection::NORTH
        );
    }
}
