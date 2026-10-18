const WHEEL: Wheel = Wheel {
    level_0: Ring([2, 15, 23, 19, 3, 2, 3, 27, 20, 11, 27, 10, 19, 10, 13, 10]),
    level_1: (
        Ring([22, 9, 5, 10, 5, 1, 24, 2, 10, 9, 7, 3, 12, 24, 10, 9]),
        HalfRing::init([2, 10, 15, 6, 9, 16, 17, 2]),
    ),
    level_2: (
        Ring([8, 24, 8, 3, 6, 15, 22, 6, 1, 1, 11, 27, 14, 5, 5, 7]),
        HalfRing::init([2, 17, 15, 14, 5, 10, 2, 22]),
    ),
    level_3: (
        Ring([6, 10, 4, 1, 5, 5, 4, 8, 6, 3, 1, 6, 10, 6, 10, 2]),
        HalfRing::init([10, 6, 13, 3, 3, 6, 10, 10]),
    ),
};

struct Wheel {
    level_0: Ring,
    level_1: (Ring, HalfRing),
    level_2: (Ring, HalfRing),
    level_3: (Ring, HalfRing),
}

impl Wheel {
    fn solve(&self) {
        // level_1 ring is in fixed position the whole time
        // level_1 half_ring can rotate in 16 different positions
        for offset_level_1_half_ring in 0..16 {
            // level_2 ring has same offset as level_1 half_ring
            // level_2 half_ring can rotate in 16 different positions
            for offset_level_2_half_ring in 0..16 {
                // level_3 ring has same offset as level_2 half_ring
                // level_3 half_ring can rotate in 16 different positions
                for offset_level_3_half_ring in 0..16 {
                    let ring_0 = &self.level_0;

                    let mut half_ring_1 = self.level_1.1.clone();
                    half_ring_1.apply_offset(offset_level_1_half_ring);
                    let mut ring_1 = self.level_1.0.clone();
                    ring_1.overlay_half_ring(&half_ring_1);

                    let mut half_ring_2 = self.level_2.1.clone();
                    half_ring_2.apply_offset(offset_level_2_half_ring);
                    let mut ring_2 = self.level_2.0.clone();
                    ring_2.apply_offset(offset_level_1_half_ring);
                    ring_2.overlay_half_ring(&half_ring_2);

                    let mut half_ring_3 = self.level_3.1.clone();
                    half_ring_3.apply_offset(offset_level_3_half_ring);
                    let mut ring_3 = self.level_3.0.clone();
                    ring_3.apply_offset(offset_level_2_half_ring);
                    ring_3.overlay_half_ring(&half_ring_3);

                    if Self::validate_sum(ring_0, &ring_1, &ring_2, &ring_3) {
                        println!(
                            "Solution found - Sum is {}

Level 0 (outmost)
Ring: {ring_0:?}

Level 1
Ring: {ring_1:?}

Level 2
Ring: {ring_2:?}

Level 3 (inmost)
Ring: {ring_3:?}",
                            ring_0.0[0] + ring_1.0[0] + ring_2.0[0] + ring_3.0[0]
                        );
                    }
                }
            }
        }
    }

    fn validate_sum(ring_0: &Ring, ring_1: &Ring, ring_2: &Ring, ring_3: &Ring) -> bool {
        let sum = ring_0.0[0] + ring_1.0[0] + ring_2.0[0] + ring_3.0[0];
        for index in 0..15 {
            if sum != ring_0.0[index] + ring_1.0[index] + ring_2.0[index] + ring_3.0[index] {
                return false;
            }
        }

        true
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Ring([u8; 16]);

impl Ring {
    fn apply_offset(&mut self, offset: usize) {
        assert!(offset < 16, "Offset needs to be smaller than 16.");

        self.0.rotate_right(offset);
    }

    fn overlay_half_ring(&mut self, half_ring: &HalfRing) {
        self.0
            .iter_mut()
            .zip(half_ring.0.iter())
            .for_each(|(dst, src)| {
                if let Some(number) = src {
                    *dst = *number;
                }
            })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct HalfRing([Option<u8>; 16]);

impl HalfRing {
    const fn init(numbers: [u8; 8]) -> Self {
        Self([
            Some(numbers[0]),
            None,
            Some(numbers[1]),
            None,
            Some(numbers[2]),
            None,
            Some(numbers[3]),
            None,
            Some(numbers[4]),
            None,
            Some(numbers[5]),
            None,
            Some(numbers[6]),
            None,
            Some(numbers[7]),
            None,
        ])
    }

    fn apply_offset(&mut self, offset: usize) {
        assert!(offset < 16, "Offset needs to be smaller than 16.");

        self.0.rotate_right(offset);
    }
}

fn main() {
    WHEEL.solve();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_offset_to_ring() {
        let mut ring = Ring([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

        let result_ring = Ring([14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);

        ring.apply_offset(3);
        assert_eq!(ring, result_ring);
    }

    #[test]
    #[should_panic(expected = "Offset needs to be smaller than 16.")]
    fn apply_too_high_offset_to_ring() {
        let mut ring = Ring([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

        ring.apply_offset(16);
    }

    #[test]
    fn apply_even_offset_to_half_ring() {
        let mut half_ring = HalfRing::init([1, 2, 3, 4, 5, 6, 7, 8]);

        let result_half_ring = HalfRing([
            Some(7),
            None,
            Some(8),
            None,
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
            None,
        ]);

        half_ring.apply_offset(4);
        assert_eq!(half_ring, result_half_ring);
    }

    #[test]
    fn apply_uneven_offset_to_half_ring() {
        let mut half_ring = HalfRing::init([1, 2, 3, 4, 5, 6, 7, 8]);

        let result_half_ring = HalfRing([
            None,
            Some(7),
            None,
            Some(8),
            None,
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);

        half_ring.apply_offset(5);
        assert_eq!(half_ring, result_half_ring);
    }

    #[test]
    #[should_panic(expected = "Offset needs to be smaller than 16.")]
    fn apply_too_high_offset_to_half_ring() {
        let mut half_ring = HalfRing::init([1, 2, 3, 4, 5, 6, 7, 8]);

        half_ring.apply_offset(16);
    }

    #[test]
    fn overlay_ring_with_half_ring() {
        let mut ring = Ring([0; 16]);
        let half_ring = HalfRing::init([1; 8]);

        let result_ring = Ring([1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]);

        ring.overlay_half_ring(&half_ring);
        assert_eq!(ring, result_ring);
    }

    #[test]
    fn overlay_ring_with_random_half_ring() {
        let mut ring = Ring([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        let half_ring = HalfRing::init([2, 4, 6, 8, 10, 12, 14, 16]);

        let result_ring = Ring([2, 2, 4, 4, 6, 6, 8, 8, 10, 10, 12, 12, 14, 14, 16, 16]);

        ring.overlay_half_ring(&half_ring);
        assert_eq!(ring, result_ring);
    }

    #[test]
    fn overlay_ring_1_with_no_offset_half_ring() {
        let mut ring = Ring([22, 9, 5, 10, 5, 1, 24, 2, 10, 9, 7, 3, 12, 24, 10, 9]);
        let half_ring = HalfRing::init([2, 10, 15, 6, 9, 16, 17, 2]);

        let result_ring = Ring([2, 9, 10, 10, 15, 1, 6, 2, 9, 9, 16, 3, 17, 24, 2, 9]);

        ring.overlay_half_ring(&half_ring);
        assert_eq!(ring, result_ring);
    }

    #[test]
    fn overlay_ring_1_with_offset_half_ring() {
        let mut ring = Ring([22, 9, 5, 10, 5, 1, 24, 2, 10, 9, 7, 3, 12, 24, 10, 9]);
        let mut half_ring = HalfRing::init([2, 10, 15, 6, 9, 16, 17, 2]);
        half_ring.apply_offset(3);

        let result_ring = Ring([22, 2, 5, 2, 5, 10, 24, 15, 10, 6, 7, 9, 12, 16, 10, 17]);

        ring.overlay_half_ring(&half_ring);
        assert_eq!(ring, result_ring);
    }

    #[test]
    fn sum_with_zero() {
        let ring_0 = Ring([0; 16]);
        let ring_1 = Ring([0; 16]);
        let ring_2 = Ring([0; 16]);
        let ring_3 = Ring([0; 16]);

        assert!(Wheel::validate_sum(&ring_0, &ring_1, &ring_2, &ring_3));
    }

    #[test]
    fn invalid_sum() {
        let ring_0 = Ring([0; 16]);
        let ring_1 = Ring([0; 16]);
        let ring_2 = Ring([0; 16]);
        let mut ring_3 = Ring([0; 16]);
        ring_3.0[5] = 3;

        assert!(!Wheel::validate_sum(&ring_0, &ring_1, &ring_2, &ring_3));
    }
}
