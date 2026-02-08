use crate::*;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Transform {
    Move(Move),
    Rotation(Rotation),
    SliceMove(SliceMove),
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Transform::*;
        match self {
            Move(m) => m.fmt(f),
            Rotation(r) => r.fmt(f),
            SliceMove(m) => m.fmt(f),
        }
    }
}

impl TryFrom<&str> for Transform {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(m) = Move::try_from(value) {
            Ok(Self::Move(m))
        } else if let Ok(r) = Rotation::try_from(value) {
            Ok(Self::Rotation(r))
        } else if let Ok(m) = SliceMove::try_from(value) {
            Ok(Self::SliceMove(m))
        } else {
            Err(format!("Invalid transform: {}", value))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Move;
    use crate::Rotation;
    use crate::SliceMove;
    use crate::Transform;

    #[test]
    fn test_from_str() {
        let res = Transform::try_from("U'");
        assert_eq!(res, Ok(Transform::Move(Move::U3)));
        let res = Transform::try_from("y'");
        assert_eq!(res, Ok(Transform::Rotation(Rotation::Y3)));
        let res = Transform::try_from("M2");
        assert_eq!(res, Ok(Transform::SliceMove(SliceMove::M2)));
    }
}
