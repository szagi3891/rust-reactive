use vertigo::{computed::Computed, VDomNode, NodeAttr, Css};
use self::config::Config;

use super::state::{Cell, Sudoku, sudoku_square::SudokuSquare, tree_box::TreeBoxIndex};

pub mod config;
pub mod render_cell_value;
pub mod render_cell_possible;

fn CssCenter() -> Css {
    Css::one("
        display: flex;
        justify-content: center;
    ")
}

fn cssWrapper() -> Css {
    let config = Config::new();
    Css::new(format!("
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        flex-shrink: 0;

        width: {}px;
        height: {}px;

        border: 2px solid blue;
    ", config.allWidth, config.allWidth))
}

fn cssItemWrapper() -> Css {
    let config = Config::new();
    Css::new(format!("
        border: {}px solid black;

        width: {}px;
        height: {}px;

        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        flex-shrink: 0;
    ", config.groupBorderSize, config.groupWidthSize, config.groupWidthSize))
}

fn cssCellWrapper() -> Css {
    let config = Config::new();
    Css::new(format!("
        border: {}px solid green;
        width: {}px;
        height: {}px;
    ", config.itemBorderSize, config.itemWidthSize, config.itemWidthSize))
}

fn render_cell(item: &Computed<Cell>) -> VDomNode {
    let value = *item.getValue().number.value.getValue();

    if let Some(value) = value {
        return render_cell_value::render_cell_value(value, item);
    }

    render_cell_possible::render_cell_possible(item)
}

fn render_group(group: &Computed<SudokuSquare<Cell>>) -> VDomNode {
    use NodeAttr::{buildNode, node, css, component};

    let getCell = |group: &Computed<SudokuSquare<Cell>>, x: TreeBoxIndex, y: TreeBoxIndex| -> Computed<Cell> {
        group.clone().map(move |state| {
            state.getValue().getFrom(x, y)
        })
    };

    buildNode("div", vec!(
        css(cssItemWrapper()),
        node("div", vec!(
            css(cssCellWrapper()),
            component(getCell(group, TreeBoxIndex::First,  TreeBoxIndex::First),  render_cell),
        )),
        node("div", vec!(
            css(cssCellWrapper()),
            component(getCell(group, TreeBoxIndex::First,  TreeBoxIndex::Middle), render_cell),
        )),
        node("div", vec!(
            css(cssCellWrapper()),
            component(getCell(group, TreeBoxIndex::First,  TreeBoxIndex::Last),   render_cell),
        )),
        node("div", vec!(
            css(cssCellWrapper()),
            component(getCell(group, TreeBoxIndex::Middle, TreeBoxIndex::First),  render_cell),
        )),
        node("div", vec!(
            css(cssCellWrapper()),
            component(getCell(group, TreeBoxIndex::Middle, TreeBoxIndex::Middle), render_cell),
        )),
        node("div", vec!(
            css(cssCellWrapper()),
            component(getCell(group, TreeBoxIndex::Middle, TreeBoxIndex::Last),   render_cell),
        )),
        node("div", vec!(
            css(cssCellWrapper()),
            component(getCell(group, TreeBoxIndex::Last,   TreeBoxIndex::First),  render_cell),
        )),
        node("div", vec!(
            css(cssCellWrapper()),
            component(getCell(group, TreeBoxIndex::Last,   TreeBoxIndex::Middle), render_cell),
        )),
        node("div", vec!(
            css(cssCellWrapper()),
            component(getCell(group, TreeBoxIndex::Last,   TreeBoxIndex::Last),   render_cell),
        ))
    ))
}

pub fn main_render(sudoku: &Computed<Sudoku>) -> VDomNode {
    use NodeAttr::{buildNode, node, css, component};

    let getGroup = |sudoku: &Computed<Sudoku>, x: TreeBoxIndex, y: TreeBoxIndex| -> Computed<SudokuSquare<Cell>> {
        sudoku.clone().map(move |state| {
            state.getValue().grid.getFrom(x, y)
        })
    };
    
    buildNode("div", vec!(
        css(CssCenter()),
        node("div", vec!(
            css(cssWrapper()),
            component(getGroup(sudoku, TreeBoxIndex::First,  TreeBoxIndex::First),  render_group),
            component(getGroup(sudoku, TreeBoxIndex::First,  TreeBoxIndex::Middle), render_group),
            component(getGroup(sudoku, TreeBoxIndex::First,  TreeBoxIndex::Last),   render_group),
            component(getGroup(sudoku, TreeBoxIndex::Middle, TreeBoxIndex::First),  render_group),
            component(getGroup(sudoku, TreeBoxIndex::Middle, TreeBoxIndex::Middle), render_group),
            component(getGroup(sudoku, TreeBoxIndex::Middle, TreeBoxIndex::Last),   render_group),
            component(getGroup(sudoku, TreeBoxIndex::Last,   TreeBoxIndex::First),  render_group),
            component(getGroup(sudoku, TreeBoxIndex::Last,   TreeBoxIndex::Middle), render_group),
            component(getGroup(sudoku, TreeBoxIndex::Last,   TreeBoxIndex::Last),   render_group),
        ))
    ))
}

fn cssSudokuExample() -> Css {
    Css::one("
        border: 1px solid black;
        padding: 10px;
        margin: 10px 0;
    ")
}

fn cssSudokuExampleButton() -> Css {
    Css::one("
        margin: 5px;
        cursor: pointer;
    ")
}
pub fn examples_render(sudoku: &Computed<Sudoku>) -> VDomNode {
    use NodeAttr::{buildNode, node, css, text, onClick};

    let sudoku = sudoku.getValue();
    buildNode("div", vec!(
        css(cssSudokuExample()),
        node("button", vec!(
            css(cssSudokuExampleButton()),
            onClick({
                let sudoku = sudoku.clone();

                move || {
                    sudoku.clear();
                }
            }),
            text("Wyczyść")
        )),
        node("button", vec!(
            css(cssSudokuExampleButton()),
            onClick({
                let sudoku = sudoku.clone();

                move || {
                    sudoku.example1();
                }
            }),
            text("Przykład 1")
        )),
        node("button", vec!(
            css(cssSudokuExampleButton()),
            onClick({
                let sudoku = sudoku.clone();

                move || {
                    sudoku.example2();
                }
            }),
            text("Przykład 2")
        )),
        node("button", vec!(
            css(cssSudokuExampleButton()),
            onClick({
                let sudoku = sudoku.clone();

                move || {
                    sudoku.example3();
                }
            }),
            text("Przykład 3")
        ))
    ))
}
