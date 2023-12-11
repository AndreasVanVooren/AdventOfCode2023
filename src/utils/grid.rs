use std::ops;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Coord {
    pub x : i64,
    pub y : i64
}

impl ops::Add<Coord> for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x : self.x + rhs.x,
            y : self.y + rhs.y
        }
    }
}

impl ops::AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub<Coord> for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x : self.x - rhs.x,
            y : self.y - rhs.y
        }
    }
}

impl ops::SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul<Coord> for Coord {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x : self.x * rhs.x,
            y : self.y * rhs.y
        }
    }
}

impl ops::MulAssign<Coord> for Coord {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::Div<Coord> for Coord {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x : self.x / rhs.x,
            y : self.y / rhs.y
        }
    }
}

impl ops::DivAssign<Coord> for Coord {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

pub type Grid<T> = Vec::<Vec::<T>>;

pub fn for_grid_on_other_grid<C, O, T>(recipient: &mut Grid::<T>, source: &Grid::<T>,location: &Coord, condition: &C, operation: &mut O)
where C: Fn(&T,&T) -> bool, O: FnMut(&mut T, &T)
{
    assert_ne!(source.len(), 0);
    assert!(source.len() as i64 + location.y <= recipient.len() as i64 );
    assert!(source[0].len() as i64 + location.x <= recipient[0].len() as i64 );
    
    for row in (0 .. source.len()) {
        for col in (0..source[row].len()) {
            let coord = Coord {
                x : row as i64 + location.y as i64,
                y : col as i64 + location.x as i64
            };
            
            if coord.y >= 0 && coord.y < recipient.len() as i64 && coord.x >= 0 && coord.x < recipient[0].len() as i64 {
                if condition(&recipient[coord.y as usize][coord.x as usize], &source[row][col]) {
                    operation(&mut recipient[coord.y as usize][coord.x as usize], &source[row][col]);
                }
            }
        }
    }
}

// TODO: This function doesn't work because Rust doesn't allow it.
// Need to find a workaround before implementing it.
//pub fn for_each_adjacent_elem<C, O, T>(recipient: &mut Grid::<T>, condition: &C, operation: &mut O)
//    where C: Fn(&T) -> bool, O: FnMut(&mut T, &mut T)
//{
//    for rec_row in 0 .. recipient.len() {
//        let mut rec_row_ref = & recipient[rec_row];
//        for rec_col in 0 .. rec_row_ref.len() {
//            let mut rec_elem_ref = &mut rec_row_ref[rec_col];
//            for row in -1 .. 2 {
//                for col in -1 .. 2 {
//                    if row == 0 && col == 0 {continue;}
//
//                    let coord = Coord {
//                        x : row as i64 + rec_row as i64,
//                        y : col as i64 + rec_col as i64
//                    };
//                    
//                    if coord.y >= 0 && coord.y < recipient.len() as i64 && coord.x >= 0 && coord.x < rec_row_ref.len() as i64 {
//                        let mut row_ref = &recipient[coord.y as usize];
//                        let mut elem_ref = &mut row_ref[coord.x as usize];
//                        if condition(&elem_ref) {
//                            operation(&mut elem_ref, &mut rec_elem_ref);
//                        }
//                    }
//                }
//            }
//        }
//    }
//}