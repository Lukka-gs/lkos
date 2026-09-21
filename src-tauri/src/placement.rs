//! Pure physical-coordinate placement. No monitor names or OS state in tests.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub fn initial_bounds(work: Rect, scale: f64) -> Result<Rect, &'static str> {
    if !scale.is_finite() || scale <= 0.0 || scale > 16.0 || work.width == 0 || work.height == 0 {
        return Err("Invalid monitor work area or scale");
    }
    let width = ((360.0 * scale).round() as u32).max(1).min(work.width);
    let height = ((76.0 * scale).round() as u32).max(1).min(work.height);
    let min_x = i64::from(work.x);
    let min_y = i64::from(work.y);
    let max_x = min_x + i64::from(work.width - width);
    let max_y = min_y + i64::from(work.height - height);
    let x = (max_x - (160.0 * scale).round() as i64).clamp(min_x, max_x);
    let y = (min_y + (48.0 * scale).round() as i64).clamp(min_y, max_y);
    Ok(Rect {
        x: x.try_into().map_err(|_| "Coordinate overflow")?,
        y: y.try_into().map_err(|_| "Coordinate overflow")?,
        width,
        height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stays_in_work_area_at_supported_fixture_scales() {
        for scale in [1.0, 1.25, 1.5, 2.0] {
            for work in [
                Rect {
                    x: 0,
                    y: 0,
                    width: 1920,
                    height: 1040,
                },
                Rect {
                    x: -2560,
                    y: -400,
                    width: 2560,
                    height: 1400,
                },
                Rect {
                    x: 48,
                    y: 80,
                    width: 1280,
                    height: 680,
                },
            ] {
                let b = initial_bounds(work, scale).unwrap();
                assert!(b.x >= work.x && b.y >= work.y);
                assert!(
                    i64::from(b.x) + i64::from(b.width)
                        <= i64::from(work.x) + i64::from(work.width)
                );
                assert!(
                    i64::from(b.y) + i64::from(b.height)
                        <= i64::from(work.y) + i64::from(work.height)
                );
                assert_eq!(b.width, (360.0 * scale).round() as u32);
            }
        }
    }
    #[test]
    fn clamps_vertical_offset_and_oversize_overlay() {
        let work = Rect {
            x: -300,
            y: 20,
            width: 200,
            height: 90,
        };
        assert_eq!(initial_bounds(work, 2.0).unwrap(), work);
        let b = initial_bounds(
            Rect {
                width: 1920,
                height: 100,
                x: 0,
                y: 0,
            },
            1.0,
        )
        .unwrap();
        assert_eq!(b.y, 24); // old 48px offset placed bottom outside this work area
    }
    #[test]
    fn refuses_invalid_display_data() {
        let work = Rect {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };
        for scale in [0.0, -1.0, f64::NAN, f64::INFINITY, 17.0] {
            assert!(initial_bounds(work, scale).is_err());
        }
        assert!(initial_bounds(Rect { width: 0, ..work }, 1.0).is_err());
    }
}
