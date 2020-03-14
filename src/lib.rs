pub mod cell_state;
pub mod game;

extern crate ndarray;

use cell_state::CellState;
use ndarray::{Array, ArrayBase, Dim, OwnedRepr, ShapeBuilder};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

pub type Field = ArrayBase<OwnedRepr<CellState>, Dim<[usize; 2]>>;

pub fn random_field<Sh>(size: Sh) -> Field
where
    Sh: ShapeBuilder<Dim = Dim<[usize; 2]>>,
{
    Array::random(size, Uniform::new_inclusive(0, 1)).map(CellState::from)
}

pub fn next_generation(current_generation: &Field) -> Field {
    Array::from_shape_vec(
        current_generation.raw_dim(),
        current_generation
            .indexed_iter()
            .map(|(index, &value)| {
                let neighbour_count: u8 = neighbour_indices(index)
                    .iter()
                    .filter_map(|&index| {
                        current_generation
                            .get(index)
                            .map(|&cell_state| cell_state as u8)
                    })
                    .sum();
                match neighbour_count {
                    2 => value,
                    3 => CellState::Live,
                    _ => CellState::Dead,
                }
            })
            .collect::<Vec<_>>(),
    )
    .expect("panic")
}

fn neighbour_indices(index: (usize, usize)) -> Vec<(usize, usize)> {
    let index = (index.0 as i32, index.1 as i32);
    [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
    .iter()
    .filter_map(|&offset| {
        let neighbour = (index.0 + offset.0, index.1 + offset.1);
        if neighbour.0 < 0 || neighbour.1 < 0 {
            None
        } else {
            Some((neighbour.0 as usize, neighbour.1 as usize))
        }
    })
    .collect::<Vec<_>>()
}

#[cfg(test)]
mod next_generation_tests {
    use super::CellState::{Dead, Live};
    use super::{next_generation, Field};
    use ndarray::array;
    #[test]
    fn tub() {
        let field: Field = array![[Live, Dead, Live], [Dead, Live, Dead], [Live, Dead, Live]];

        let one_step: Field = array![[Dead, Live, Dead], [Live, Dead, Live], [Dead, Live, Dead]];

        assert_eq!(next_generation(&field), one_step);
        assert_eq!(next_generation(&one_step), one_step);
    }

    #[test]
    fn blinker() {
        let horizontal: Field = array![
            [Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead],
            [Dead, Live, Live, Live, Dead],
            [Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead]
        ];

        let vertical: Field = array![
            [Dead, Dead, Dead, Dead, Dead],
            [Dead, Dead, Live, Dead, Dead],
            [Dead, Dead, Live, Dead, Dead],
            [Dead, Dead, Live, Dead, Dead],
            [Dead, Dead, Dead, Dead, Dead]
        ];

        assert_eq!(next_generation(&horizontal), vertical);
        assert_eq!(next_generation(&vertical), horizontal);
        assert_eq!(next_generation(&next_generation(&horizontal)), horizontal);
    }
}

#[test]
fn neighbour_indices_test() {
    let i1: (usize, usize) = (4, 5);
    let e1 = [
        (4, 6),
        (5, 5),
        (4, 4),
        (3, 5),
        (5, 6),
        (5, 4),
        (3, 6),
        (3, 4),
    ];

    assert_eq!(neighbour_indices(i1), e1);

    let i2: (usize, usize) = (0, 0);
    let e2 = [(0, 1), (1, 0), (1, 1)];
    assert_eq!(neighbour_indices(i2), e2);
}
