class ChessRow {
    public static readonly SIZE = 8;
    private element: HTMLElement;
    private board: ChessBoard | undefined;
    private fields: ChessField[] = [];
    index: number;
    constructor(element: HTMLElement, index: number) {
        this.element = element;
        this.index = index;
    }

    getField(index: number) {
        return this.fields[index];
    }

    addField(field: ChessField) {
        this.fields.push(field);
    }

    setBoard(board: ChessBoard) {
        this.board = board;
    }

    getBoard() {
        return this.board;
    }

    setFigures(color: COLOR, ...figures: (FIGURE|null)[]) {
        if (figures.length == 1) {
            let l = figures[0];
            figures = [l, l, l, l, l, l, l, l];
        };
        for (let i = 0; i < this.fields.length; i++) {
            let field = this.fields[i];
            let figure = figures[i];
            if(!figure){
                field.setFigure(null);
                continue;
            }
            let fig = { type: figure, color: color, field: field,alreadyMoved: false};
            field.setFigure(fig);
            this.getBoard()?.teams[color].figures.push(fig);
        }
    }
}