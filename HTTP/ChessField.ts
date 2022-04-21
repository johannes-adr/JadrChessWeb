interface OnlineMove {
    start: number,
    end: number
}

class ChessField {
    private element: HTMLElement;
    private row: ChessRow;
    private color: COLOR = COLOR.white;
    private figure: Figure | null = null;
    private static i = 0;
    private globalIndex = ChessField.i++;
    private localIndex: number;
    private static DRAG_DROP: { start: ChessField | null, current: ChessField | null, moves?: Move[] } = { start: null, current: null }
    private cover: HTMLElement;
    private numelement: HTMLElement;
    constructor(element: HTMLElement, numelement: HTMLElement, row: ChessRow, i: number, cover: HTMLElement) {
        let s = ChessRow.SIZE;
        let thiz = this;
        this.localIndex = i;
        this.element = element;
        this.numelement = numelement;
        this.row = row;
        this.cover = cover;
        row.getBoard()?.fields.push(this);

        function endMove(cr: ChessField | null, e: Event) {
            if (cr && cr.isHighlighted()) {
                let ms = ChessField.DRAG_DROP.moves!;
                for (let i = 0; i < ms.length; i++) {
                    let m = ms[i];
                    if (m.end == cr.getGlobalIndex()) {
                        if (!OFFLINE_DEBUG) {
                            let jsonMove = { ...m };
                            sendPacket({ type: "doMove", args: [LOBBYID, { start: m.start, end: m.end,flag: m.flag }] });
                        } else {
                            doMove(m);
                        }
                        break;
                    }
                }
            } else {
                e.preventDefault();
            }
            ChessField.unHighlightAll();
        }

        function startMove(e: Event) {
            ChessField.DRAG_DROP.start = thiz;
            let vb: VirtualBoard = new VirtualBoard(thiz.getRow().getBoard()!);
            let f = thiz.figure!;

            if (!OFFLINE_DEBUG) if (f.color != chessBoard.getCurrentSide() || chessBoard.getCurrentSide() != chessBoard.getThisInstanceColor()) {
                e.preventDefault();
                return;
            }
            let moves = vb.pieces[thiz.globalIndex]!.getAvaibleMoves(true)!;
            ChessField.DRAG_DROP.moves = moves;
            ChessField.unHighlightAll();

            for (let i = 0; i < moves.length; i++) {
                chessBoard.fields[moves[i].end]?.highlight();
            }
        }


        element.ondragstart = e => {
            startMove(e);
        }
        element.ondragover = e => {
            ChessField.DRAG_DROP.current = this;
        }

        element.onclick = e => {
            if(ChessField.highlighted.length > 0){
                console.log("ending move")
                endMove(this, e);
                if(ChessField.DRAG_DROP.start !== thiz){
                    startMove(e);
                }
            }else{
                startMove(e);
            }
            
        }

        element.ondragend = e => {
            let cr = ChessField.DRAG_DROP.current;
            endMove(cr,e);
        }
    }
    private static highlighted: ChessField[] = [];
    private hightlight = false;
    highlight() {
        ChessField.highlighted.push(this);
        this.cover.style.opacity = "0.3";
        this.hightlight = true;
    }

    getElement() {
        return this.element;
    }

    getNumElement() {
        return this.numelement;
    }

    unHighlight() {
        this.cover.style.opacity = "0";
        this.hightlight = false;
    }

    isHighlighted() {
        return this.hightlight;
    }

    static unHighlightAll() {
        ChessField.highlighted.forEach(e => e.unHighlight());
        ChessField.highlighted = [];
    }

    getGlobalIndex() {
        return this.globalIndex;
    }

    getLocalIndex() {
        return this.localIndex;
    }

    getRow() {
        return this.row;
    }

    setFigure(figure: Figure | null) {
        let ts = this.getRow().getBoard()!.teams;
        if (this.figure && figure) {
            let t = ts[this.figure.color]
            t.figures.splice(t.figures.indexOf(this.figure), 1);
        }
        this.figure = figure;
        this.element.innerHTML = "";
        if (figure) {
            let img = document.createElement("img");
            img.classList.add("figure-img");
            img.src = `assets/chessFigures/${figure.color}_${figure.type}.png`;
            this.element.appendChild(img);
            figure.field = this;
        }
    }

    getFigure() {
        return this.figure;
    }

    setColor(color: COLOR) {
        this.color = color;
    }
}