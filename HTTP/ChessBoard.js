"use strict";
class ChessBoard {
    constructor(element) {
        this.rows = [];
        this.onTurn = COLOR.white;
        this.rotation = 0;
        this.thisInstanceColor = COLOR.white;
        this.turns = 1;
        this.fields = [];
        this.teams = { white: new Team(COLOR.white), black: new Team(COLOR.black) };
        this.minusSize = 0;
        this.element = element;
    }
    rotate(r) {
        if (!r)
            r = this.rotation += 180;
        function rot(e) {
            e.style.transform = `rotate(${r}deg)`;
        }
        rot(this.element);
        this.fields.forEach(e => {
            if (e) {
                rot(e.getElement());
                rot(e.getNumElement());
            }
        });
    }
    smaller() {
        if (this.minusSize > 500)
            return;
        this.minusSize += 50;
        this.setCSSBoardSize(this.minusSize);
    }
    bigger() {
        this.minusSize -= 50;
        this.setCSSBoardSize(this.minusSize);
    }
    setCSSBoardSize(size) {
        setCSSRule("#board", "--cw", `${size}px`);
    }
    addToMoveContainer(move, startFig, f) {
        let moves = document.getElementById("moves");
        let moveturn = document.getElementById("moveturn-" + this.turns);
        if (!moveturn) {
            moveturn = document.createElement("div");
            moveturn.classList.add("hstack", "moveturn");
            moveturn.id = "moveturn-" + this.turns;
            let turncount = document.createElement("span");
            turncount.innerText = this.turns + "";
            moveturn.appendChild(turncount);
            moves.appendChild(moveturn);
        }
        {
            console.log("Current move childs: " + moves.childNodes.length);
            if (chessBoard.turns == 1 && chessBoard.getCurrentSide() == COLOR.black && moves.childNodes.length == 4) {
                moves.appendChild(document.createElement("div"));
            }
            let div = document.createElement("div");
            div.classList.add("turnInfo");
            let attackimg = document.createElement("img");
            attackimg.classList.add("turnInfo-img");
            attackimg.src = `assets/chessFigures/${startFig.color}_${startFig.type}.png`;
            attackimg.setAttribute("draggable", "false");
            div.appendChild(attackimg);
            div.append(getFieldDesc(move.start) + " => " + (f ? "" : getFieldDesc(move.end)));
            if (f) {
                let devimg = document.createElement("img");
                devimg.classList.add("turnInfo-img");
                devimg.setAttribute("draggable", "false");
                devimg.src = `assets/chessFigures/${f.color}_${f.type}.png`;
                div.appendChild(devimg);
                div.append(getFieldDesc(move.end));
            }
            moves.appendChild(div);
            return div;
        }
    }
    loadFen(fen) {
        this.reset();
        let fenAreas = fen.split(" ");
        console.log(fenAreas);
        let figRows = fenAreas[0].split("/");
        const fenChars = { p: FIGURE.pawn, n: FIGURE.knight, b: FIGURE.bishop, r: FIGURE.rook, q: FIGURE.queen, k: FIGURE.king };
        function isNumber(value) {
            return "0123456789".indexOf(value) != -1;
        }
        function figFromFenChar(value, field) {
            let isWhite = value == value.toUpperCase();
            //@ts-ignore
            let type = fenChars[value.toLocaleLowerCase()];
            return { alreadyMoved: false, color: isWhite ? COLOR.white : COLOR.black, field: field, type: type };
        }
        for (let y = 0; y < figRows.length; y++) {
            let rowStr = figRows[y];
            let x = 0;
            for (let i = 0; i < rowStr.length; i++) {
                let c = rowStr[i];
                if (isNumber(c)) {
                    x += Number(c) - 1;
                }
                else {
                    let f = this.rows[y].getField(x);
                    f.setFigure(figFromFenChar(c, f));
                }
                x++;
            }
        }
        let onturn = fenAreas[1];
        if (onturn == "b") {
            if (this.getCurrentSide() == COLOR.white)
                this.toggleTurn();
        }
        else if (onturn == "w") {
            if (this.getCurrentSide() == COLOR.black)
                this.toggleTurn();
        }
        else {
            showAlert("Error while loading board data (1)", 5, true);
        }
        let canCastle = fenAreas[2];
        let turns = fenAreas[5];
    }
    toggleTurn() {
        let img = document.getElementById("turnColor");
        img.style.borderColor = this.onTurn;
        console.log(this.onTurn);
        if (this.getCurrentSide() == COLOR.black) {
            this.onTurn = COLOR.white;
            this.turns++;
        }
        else if (this.getCurrentSide() == COLOR.white) {
            this.onTurn = COLOR.black;
        }
        document.getElementById("turnText").innerText = this.onTurn + "'s turn!";
        img.style.backgroundColor = this.onTurn;
    }
    getThisInstanceColor() {
        return this.thisInstanceColor;
    }
    getOtherTeam(color) {
        return color == COLOR.black ? this.teams.white : this.teams.black;
    }
    getCurrentSide() {
        return this.onTurn;
    }
    getRow(index) {
        return this.rows[index];
    }
    addRow(row) {
        this.rows.push(row);
        row.setBoard(this);
    }
    doKIMove() {
        let m = new VirtualBoard(this).getBestMove(2, this.getCurrentSide());
        if (m) {
            doMove(m);
        }
        else {
            alert((this.getCurrentSide() != COLOR.white ? "white" : "black") + " wins");
        }
    }
    endGame(winner) {
        if (winner) {
            showAlert(winner.toLocaleLowerCase() + " won the match!", 20, false);
        }
        else {
            showAlert("Draw!", 20, false);
        }
        //@ts-ignore
        this.onTurn = "undefined";
        //this.reset();
    }
    reset() {
        this.teams = { white: new Team(COLOR.white), black: new Team(COLOR.black) };
        this.onTurn = COLOR.black;
        this.toggleTurn();
        this.turns = 1;
        this.fields.forEach(f => { f.setFigure(null); });
        // if(!OFFLINE_DEBUG && true){
        //     this.rows[0].setFigures(COLOR.black, FIGURE.rook, FIGURE.knight, FIGURE.bishop, FIGURE.queen, FIGURE.king, FIGURE.bishop, FIGURE.knight, FIGURE.rook);
        //     this.rows[1].setFigures(COLOR.black, FIGURE.pawn);
        //     this.rows[6].setFigures(COLOR.white, FIGURE.pawn);
        //     this.rows[7].setFigures(COLOR.white, FIGURE.rook, FIGURE.knight, FIGURE.bishop, FIGURE.queen, FIGURE.king, FIGURE.bishop, FIGURE.knight, FIGURE.rook);    
        // }else{
        //     this.rows[0].setFigures(COLOR.black, null,null,FIGURE.king);
        //     this.rows[7].setFigures(COLOR.white, FIGURE.rook, null, null, null, FIGURE.king, null, null, FIGURE.rook);
        // }
        let moves = document.getElementById("moves");
        moves.innerHTML = "<span></span><span>White</span><span>Black</span>";
        //this.toggleTurn(true);
        // setInterval(() => {
        //     this.doKIMove();
        // }, 100);
    }
}
