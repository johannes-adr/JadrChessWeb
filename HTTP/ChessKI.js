"use strict";
var FLAG;
(function (FLAG) {
    FLAG[FLAG["CASTLE"] = 0] = "CASTLE";
    FLAG[FLAG["PROMOTE"] = 1] = "PROMOTE";
})(FLAG || (FLAG = {}));
class VirtualFigure {
    constructor(type, color, alreadyMoved, chessField, startPos, virtualBoard) {
        this.color = color;
        this.type = type;
        this.alreadyMoved = alreadyMoved;
        this.worth = CHESS_FIGURES_VALUES[type] * 100;
        this.pos = startPos;
        this.virtualBoard = virtualBoard;
        this.chessField = chessField;
    }
    clone(virtualBoard) {
        let clone = new VirtualFigure(this.type, this.color, this.alreadyMoved, this.chessField, this.pos, virtualBoard ? virtualBoard : this.virtualBoard);
        return clone;
    }
    doMove(move) {
        let movedBefore = this.alreadyMoved;
        this.alreadyMoved = true;
        let t = this;
        let vb = t.virtualBoard;
        let endPiece = move.target;
        vb.pieces[move.start] = undefined;
        let dp;
        let i;
        let changed = -1;
        if (endPiece) {
            dp = vb.directPieces[endPiece.color];
            for (let j = 0; j < dp.length; j++) {
                let dpj = dp[j];
                if (dpj && dpj.pos == endPiece.pos) {
                    dp[j] = undefined;
                    changed = j;
                }
            }
        }
        t.pos = move.end;
        vb.pieces[move.end] = t;
        return {
            move: move,
            undo: () => {
                vb.pieces[move.start] = t;
                t.pos = move.start;
                vb.pieces[move.end] = endPiece ? endPiece : undefined;
                if (changed != -1)
                    vb.directPieces[endPiece.color][changed] = endPiece;
                t.alreadyMoved = movedBefore;
                if (endPiece)
                    dp[i] = endPiece;
            }
        };
    }
    getAvaibleMoves(doKingCheck) {
        let t = this;
        let i = this.pos;
        let fx = getX(i);
        let fy = getRow(i);
        let fields = this.virtualBoard.pieces;
        let moves = [];
        function addMove(end, worth, flag) {
            if (end < 0 || end >= fields.length)
                return;
            let vb2 = new VirtualBoard(t.virtualBoard);
            if (!worth)
                worth = 0;
            let endRow = getRow(end);
            if (endRow > fy)
                worth += (endRow - fy) * 5;
            let x = getX(end);
            let centerVal = x < 5 ? 7 - x : x;
            worth += 8 - centerVal / t.virtualBoard.turns * 5;
            let oField = fields[end];
            if (oField)
                worth = oField.worth;
            let move = { start: t.pos, end: end, figure: t, worth: worth };
            if (flag)
                move.flag = flag;
            if (oField)
                move.target = oField;
            if (doKingCheck) {
                let piece2 = vb2.pieces[t.pos];
                //Rochade während könig schach ist deaktiviert
                if (flag && flag.flag == FLAG.CASTLE) {
                    if (vb2.isFigureTypeUnderAttack(FIGURE.king, t.color))
                        return;
                }
                //Sollte durch den Zug der König unter schach gesetzt werden, ist dieser ungülting (Gepinnte figuren)
                let moveResult = piece2.doMove(move);
                if (vb2.isFigureTypeUnderAttack(FIGURE.king, t.color)) {
                    return;
                }
                moveResult.undo();
            }
            if (oField?.color != t.color)
                moves.push(move);
        }
        function xInBounds(x) { return !(x > 7 || x < 0); }
        function generateRookMoves() {
            //DOWN
            for (let ly = fy + 1; ly < 8; ly++) {
                let li = ly * 8 + fx;
                addMove(li);
                if (fields[li] != null)
                    break;
            }
            //UP
            for (let ly = fy - 1; ly >= 0; ly--) {
                let li = ly * 8 + fx;
                addMove(li);
                if (fields[li] != null)
                    break;
            }
            //RIGHT
            for (let lx = fx + 1; lx < 8; lx++) {
                let li = lx + fy * 8;
                addMove(li);
                if (fields[li] != null)
                    break;
            }
            //LEFT
            for (let lx = fx - 1; lx >= 0; lx--) {
                let li = lx + fy * 8;
                addMove(li);
                if (fields[li] != null)
                    break;
            }
        }
        function generateBishopMoves() {
            //RIGHT DOWN
            {
                let i = 1;
                for (let ly = fy + 1; ly < 8; ly++) {
                    let x = fx + i++;
                    if (x > 7 || x < 0)
                        break;
                    let li = ly * 8 + x;
                    addMove(li);
                    if (fields[li] != null)
                        break;
                }
            }
            //LEFT DOWN
            {
                let i = -1;
                for (let ly = fy + 1; ly < 8; ly++) {
                    let x = fx + i--;
                    if (!xInBounds(x))
                        break;
                    let li = ly * 8 + x;
                    addMove(li);
                    if (fields[li] != null)
                        break;
                }
            }
            //RIGHT UP
            {
                let i = 1;
                for (let ly = fy - 1; ly >= 0; ly--) {
                    let x = fx + i++;
                    if (!xInBounds(x))
                        break;
                    let li = ly * 8 + x;
                    addMove(li);
                    if (fields[li] != null)
                        break;
                }
            }
            //LEFT UP
            {
                let i = -1;
                for (let ly = fy - 1; ly >= 0; ly--) {
                    let x = fx + i--;
                    if (!xInBounds(x))
                        break;
                    let li = ly * 8 + x;
                    addMove(li);
                    if (fields[li] != null)
                        break;
                }
            }
        }
        function checkPawnPromote(isBlack, index) {
            if (isBlack) {
                if (index == 7)
                    return true;
            }
            else if (index == 0)
                return true;
            return false;
        }
        if (this.type == FIGURE.king) {
            for (let y = -1; y < 2; y++) {
                let r = getRow(fx + (y + fy) * 8);
                if (r < 0 || r > 7)
                    continue;
                for (let x = -1; x < 2; x++) {
                    let localI = x + fx + (y + fy) * 8;
                    if (r != getRow(localI) || localI == i)
                        continue;
                    addMove(localI);
                }
            }
            if (!this.alreadyMoved) {
                let rookRight = fields[i + 3];
                if (rookRight && rookRight.type == FIGURE.rook && !rookRight.alreadyMoved) {
                    if (!fields[i + 1] && !fields[i + 2]) {
                        addMove(i + 2, 0, { flag: FLAG.CASTLE, data: [i + 3, i + 1] });
                    }
                }
                let rookleft = fields[i - 4];
                if (rookleft && rookleft.type == FIGURE.rook && !rookleft.alreadyMoved) {
                    if (!fields[i - 1] && !fields[-2] && !fields[i - 3]) {
                        addMove(i - 2, 0, { flag: FLAG.CASTLE, data: [i - 4, i - 1] });
                    }
                }
            }
        }
        else if (this.type == FIGURE.pawn) {
            let b = this.color == COLOR.black;
            let front;
            let right;
            let left;
            let worth = 0;
            if (b) {
                front = i + 8;
                right = i + 7;
                left = i + 9;
                if (getRow(front) == 7)
                    worth += CHESS_FIGURES_VALUES.queen;
            }
            else {
                front = i - 8;
                right = i - 7;
                left = i - 9;
                if (getRow(front) == 0)
                    worth += CHESS_FIGURES_VALUES.queen;
            }
            let y = getRow(front);
            if (fields[front] == null) {
                let x = getX(front);
                let promote = checkPawnPromote(b, y) ? { flag: FLAG.PROMOTE } : undefined;
                //if(this.color == "white")console.log("Color: " + this.color + " b: " + b + " y: " + y + " promote: " + promote)
                addMove(front, worth, promote);
                //Wenn der zug vorwährts möglich ist, dann ist der Platz frei. osllte er noch nicht bewegt worden sein, ist ein 2er sprung möglich
                if (b) {
                    if (fy == 1 && fields[front + 8] == null)
                        addMove(front + 8, worth);
                }
                else {
                    if (fy == 6 && fields[front - 8] == null)
                        addMove(front - 8, worth);
                }
            }
            if (getRow(right) == y && fields[right] != null) {
                addMove(right, worth, checkPawnPromote(b, getRow(right)) ? { flag: FLAG.PROMOTE } : undefined);
            }
            if (getRow(left) == y && fields[left] != null) {
                addMove(left, worth, checkPawnPromote(b, getRow(left)) ? { flag: FLAG.PROMOTE } : undefined);
            }
        }
        else if (this.type == FIGURE.rook) {
            generateRookMoves();
        }
        else if (this.type == FIGURE.bishop) {
            generateBishopMoves();
        }
        else if (this.type == FIGURE.queen) {
            generateBishopMoves();
            generateRookMoves();
        }
        else if (this.type == FIGURE.knight) {
            let deltas = [[-2, -1], [-2, +1], [+2, -1], [+2, +1], [-1, -2], [-1, +2], [+1, -2], [+1, +2]];
            for (let i = 0; i < deltas.length; i++) {
                let d = deltas[i];
                let x = fx + d[1];
                let y = fy + d[0];
                if (!xInBounds(x))
                    continue;
                addMove(x + y * 8);
            }
        }
        return moves;
    }
}
function oppositeColor(color) {
    return color == COLOR.white ? COLOR.black : COLOR.white;
}
class VirtualBoard {
    constructor(init) {
        this.pieces = [];
        this.directPieces = { white: [], black: [] };
        this.turns = 0;
        if (init instanceof ChessBoard) {
            for (let i = 0; i < init.fields.length; i++) {
                let f = init.fields[i].getFigure();
                if (!f) {
                    this.pieces.push(undefined);
                }
                else {
                    let vf = new VirtualFigure(f.type, f.color, f.alreadyMoved, f.field, i, this);
                    this.pieces.push(vf);
                    this.directPieces[vf.color].push(vf);
                }
            }
            this.turns = init.turns;
        }
        else if (init instanceof VirtualBoard) {
            this.pieces = [];
            this.directPieces.white = [];
            this.directPieces.black = [];
            for (let i = 0; i < init.pieces.length; i++) {
                let piece = init.pieces[i]?.clone(this);
                this.pieces.push(piece);
                if (piece)
                    this.directPieces[piece.color].push(piece);
            }
            this.turns = init.turns;
        }
    }
    getPiece(figure) {
        return this.pieces[figure.pos];
    }
    isFigureUnderAttack(fig) {
        let enemyPieces = this.directPieces[oppositeColor(fig.color)];
        for (let i = 0; i < enemyPieces.length; i++) {
            let ep = enemyPieces[i];
            if (!ep)
                continue;
            let moves = ep.getAvaibleMoves(false);
            for (let j = 0; j < moves.length; j++) {
                let move = moves[j];
                let t = move.target;
                if (t == fig)
                    return move;
            }
        }
        return null;
    }
    isFigureTypeUnderAttack(fig, color) {
        let enemyPieces = this.directPieces[oppositeColor(color)];
        for (let i = 0; i < enemyPieces.length; i++) {
            let ep = enemyPieces[i];
            if (!ep)
                continue;
            let moves = ep.getAvaibleMoves(false);
            for (let j = 0; j < moves.length; j++) {
                let move = moves[j];
                let t = move.target;
                if (t?.type == fig)
                    return move;
            }
        }
        return null;
    }
    getBestMove(maxLevel, color) {
        var calc = 0;
        let t = this;
        function getBestMoveRecursive(vb, color, maxLevel, currentLevel) {
            if (maxLevel <= currentLevel)
                return;
            let figures = vb.directPieces[color];
            let bestMove = null;
            for (let i = 0; i < figures.length; i++) {
                let figure = figures[i];
                if (!figure)
                    continue;
                let underAttack = t.isFigureUnderAttack(figure);
                let moves = figure.getAvaibleMoves(true);
                for (let j = 0; j < moves.length; j++) {
                    calc++;
                    let move = moves[j];
                    let virtualBoard = new VirtualBoard(vb);
                    let enemyBestMove = getBestMoveRecursive(vb, getOtherColor(color), maxLevel, currentLevel + 1);
                    let virtualFigure = virtualBoard.getPiece(figure);
                    virtualFigure.doMove(move);
                    let counterMove = virtualBoard.isFigureUnderAttack(virtualFigure);
                    if (underAttack && !counterMove) {
                        move.worth += figure.worth * 2;
                        console.log("currently under attack and the is no counter move!", move);
                    }
                    if (bestMove == null)
                        bestMove = move;
                    else if (bestMove.worth < move.worth) {
                        if (!counterMove) {
                            bestMove = move;
                        }
                        else {
                            if (move.worth >= counterMove.worth)
                                bestMove = move;
                        }
                    }
                    else if (bestMove.worth == move.worth && Math.random() > 0.7)
                        bestMove = move;
                }
            }
            return bestMove;
        }
        return getBestMoveRecursive(this, color, maxLevel, 0);
    }
}
