"use strict";
// 8 x 8
function getRow(num) { return Math.floor(num / 8); }
function getX(num) { return num % 8; }
function getFieldDesc(num, onlyside) {
    let x = getX(num);
    let y = 8 - getRow(num);
    let arr = ["A", "B", "C", "D", "E", "F", "G", "H"];
    if (onlyside) {
        let s = "";
        if (arr[x] == "H")
            s += y;
        if (y == 1)
            s += arr[x];
        return s;
    }
    else {
        return arr[x] + "" + y;
    }
}
var chessBoard;
function createField(chessRow, white, j) {
    let k = chessRow.index * 8 + j;
    let field = document.createElement("div");
    field.classList.add("vstack", "boardfield", white ? "boardfield-white" : "boardfield-black");
    let figureContainer = document.createElement("div");
    figureContainer.classList.add("field-figurecontainer");
    let fieldCover = document.createElement("div");
    fieldCover.classList.add("field-cover");
    let num = document.createElement("span");
    let chessField = new ChessField(figureContainer, num, chessRow, j, fieldCover);
    chessField.setColor(white ? COLOR.white : COLOR.black);
    num.classList.add("field-number");
    num.style.color = white ? "var(--blackFieldColor)" : "var(--whiteFieldColor)";
    num.innerText = true ? getFieldDesc(k, true) : k + "";
    field.appendChild(figureContainer);
    field.appendChild(fieldCover);
    field.appendChild(num);
    return { field: field, chessField: chessField };
}
{
    let chessBoardElement = document.getElementById("board");
    chessBoard = new ChessBoard(chessBoardElement);
    let white = true;
    for (let i = 0; i < 8; i++) {
        let row = document.createElement("div");
        let chessRow = new ChessRow(row, i);
        chessBoard.addRow(chessRow);
        row.classList.add("hstack", "boardrow");
        //Field
        for (let j = 0; j < 8; j++) {
            let { field, chessField } = createField(chessRow, white, j);
            white = !white;
            chessRow.addField(chessField);
            row.appendChild(field);
        }
        chessBoardElement.appendChild(row);
        white = !white;
    }
}
console.log(chessBoard);
let stylesheet = document.styleSheets[0];
function getCSSRule(name) {
    for (let i = 0; i < stylesheet.cssRules.length; i++) {
        let r = stylesheet.cssRules[i];
        if (r.selectorText == name)
            return r;
    }
}
function setBoardColor(white, black, lastMove) {
    getCSSRule(".boardfield-white").style.setProperty("--fbg", white);
    getCSSRule(".boardfield-black").style.setProperty("--fbg", black);
    getCSSRule(":root").style.setProperty("--lastMoveColor", lastMove);
}
function getMethods(obj) {
    var result = [];
    for (var id in obj) {
        try {
            if (typeof (obj[id]) == "function") {
                result.push(id + ": " + obj[id].toString());
            }
        }
        catch (err) {
            result.push(id + ": inaccessible");
        }
    }
    return result;
}
function getBoardColor() {
    let root = getCSSRule(":root").style;
    let avaible = getCSSRule(".field-cover").style;
    let lastMove = getCSSRule(".lastMove").style;
    return {
        white: root.getPropertyValue("--whiteFieldColor"),
        black: root.getPropertyValue("--blackFieldColor"),
        avaible: avaible.getPropertyValue("--c"),
        lastMove: lastMove.getPropertyValue("--c"),
        setWhite: (value) => root.setProperty("--whiteFieldColor", value),
        setBlack: (value) => root.setProperty("--blackFieldColor", value),
        setLastMove: (value) => lastMove.setProperty("--c", value),
        setAvaible: (value) => avaible.setProperty("--c", value)
    };
}
function setCSSRule(selector, property, value) {
    getCSSRule(selector).style.setProperty(property, value);
}
function loadPreset1() {
    setBoardColor("gray", "darkgray", "rgba(36, 117, 192, 0.479)");
}
chessBoard.reset();
