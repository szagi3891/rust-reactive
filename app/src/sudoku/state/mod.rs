use virtualdom::computed::Dependencies::Dependencies;

use self::{
    number_item::{NumberItem, create_number_item},
    possible_values::{PossibleValues, possible_values},
    possible_values_last::{PossibleValuesLast, possible_values_last},
    sudoku_square::SudokuSquare
};

mod tree_box;
mod sudoku_square;
mod number_item;
mod possible_values;
mod possible_values_last;


fn createGrid(deps: &Dependencies,) -> SudokuSquare<SudokuSquare<NumberItem>> {
    SudokuSquare::createWithIterator(move |_level0x, _level0y| {
        SudokuSquare::createWithIterator(move |_level1x, _level1y| {
            create_number_item(deps, None)
        })
    })
}

fn createGridPossible(deps: &Dependencies, gridNumber: &SudokuSquare<SudokuSquare<NumberItem>>) -> SudokuSquare<SudokuSquare<PossibleValues>> {
    SudokuSquare::createWithIterator(|level0x, level0y| {
        SudokuSquare::createWithIterator(|level1x, level1y| {
            possible_values(deps, gridNumber, level0x, level0y, level1x, level1y)
        })
    })
}

fn createGridPossibleLast(
    deps: &Dependencies,
    gridNumber: &SudokuSquare<SudokuSquare<NumberItem>>,
    gridPossible: &SudokuSquare<SudokuSquare<PossibleValues>>
) -> SudokuSquare<SudokuSquare<PossibleValuesLast>> {
    SudokuSquare::createWithIterator(|level0x, level0y| {
        SudokuSquare::createWithIterator(|level1x, level1y| {
            possible_values_last(deps, gridNumber, gridPossible, level0x, level0y, level1x, level1y)
        })
    })
}
struct Cell {
    number: NumberItem,
    possible: PossibleValues,
    possibleLast: PossibleValuesLast,
}

fn creatergidView(
    deps: &Dependencies,
    gridNumber: SudokuSquare<SudokuSquare<NumberItem>>,
    gridPossible: SudokuSquare<SudokuSquare<PossibleValues>>,
    gridPossibleLast: SudokuSquare<SudokuSquare<PossibleValuesLast>>,
) -> SudokuSquare<SudokuSquare<Cell>> {

    return SudokuSquare::createWithIterator(|level0x, level0y| {
        return SudokuSquare::createWithIterator(|level1x, level1y| {
            let number = (*gridNumber.getFrom(level0x, level0y).getFrom(level1x, level1y)).clone();
            let possible = (*gridPossible.getFrom(level0x, level0y).getFrom(level1x, level1y)).clone();
            let possibleLast = (*gridPossibleLast.getFrom(level0x, level0y).getFrom(level1x, level1y)).clone();

            Cell {
                number,
                possible,
                possibleLast,
            }
        });
    });
}

pub struct Sudoku {
    grid: SudokuSquare<SudokuSquare<Cell>>,
}

impl Sudoku {
    pub fn new(deps: &Dependencies) -> Sudoku {
        let gridNumber = createGrid(deps);
        let gridPossible = createGridPossible(deps, &gridNumber);
        let gridPossibleLast = createGridPossibleLast(deps, &gridNumber, &gridPossible);

        Sudoku {
            grid: creatergidView(deps, gridNumber, gridPossible, gridPossibleLast),
        }
    }
}

