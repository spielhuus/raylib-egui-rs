pub use mint;
pub type Vector2 = mint::Vector2<f32>;
pub type Vector3 = mint::Vector3<f32>;
pub type Vector4 = mint::Vector4<f32>;
pub type Matrix = mint::RowMatrix4<f32>;
pub type Quaternion = mint::Vector4<f32>; // raylib does this same alias so we match it

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Check collision between two rectangles
    #[inline]
    #[must_use]
    pub fn check_collision_recs(&self, other: Rectangle) -> bool {
        unsafe { crate::ffi::CheckCollisionRecs(*self, other) }
    }

    /// Checks collision between circle and rectangle.
    #[inline]
    #[must_use]
    pub fn check_collision_circle_rec(&self, center: impl Into<Vector2>, radius: f32) -> bool {
        unsafe { crate::ffi::CheckCollisionCircleRec(center.into(), radius, *self) }
    }

    /// Gets the overlap between two colliding rectangles.
    /// ```rust
    /// use raylib_sys::Rectangle;
    ///
    /// let r1 = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    /// let r2 = Rectangle::new(20.0, 20.0, 10.0, 10.0);
    /// assert_eq!(None, r1.get_collision_rec(r2));
    /// assert_eq!(Some(r1), r1.get_collision_rec(r1));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_collision_rec(&self, other: Rectangle) -> Option<Rectangle> {
        self.check_collision_recs(other)
            .then(|| unsafe { crate::ffi::GetCollisionRec(*self, other) })
    }

    /// Checks if point is inside rectangle.
    #[inline]
    #[must_use]
    pub fn check_collision_point_rec(&self, point: impl Into<Vector2>) -> bool {
        unsafe { crate::ffi::CheckCollisionPointRec(point.into(), *self) }
    }
}
