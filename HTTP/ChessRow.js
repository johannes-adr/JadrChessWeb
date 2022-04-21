"use strict";
class ChessRow {
    constructor(element, index) {
        this.fields = [];
        this.element = element;
        this.index = index;
    }
    getField(index) {
        return this.fields[index];
    }
    addField(field) {
        this.fields.push(field);
    }
    setBoard(board) {
        this.board = board;
    }
    getBoard() {
        return this.board;
    }
    setFigures(color, ...figures) {
        if (figures.length == 1) {
            let l = figures[0];
            figures = [l, l, l, l, l, l, l, l];
        }
        ;
        for (let i = 0; i < this.fields.length; i++) {
            let field = this.fields[i];
            let figure = figures[i];
            if (!figure) {
                field.setFigure(null);
                continue;
            }
            let fig = { type: figure, color: color, field: field, alreadyMoved: false };
            field.setFigure(fig);
            this.getBoard()?.teams[color].figures.push(fig);
        }
    }
}
ChessRow.SIZE = 8;
