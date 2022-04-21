"use strict";
const OFFLINE_DEBUG = false;
const SOUNDS = {
    move: new Audio("/assets/sound/move.mp3"),
    enemykill: new Audio("/assets/sound/enemykillmove.mp3"),
    notification: new Audio("/assets/sound/message_notification.mp3"),
    click: new Audio("/assets/sound/click.mp3"),
    check: new Audio("/assets/sound/check.mp3")
};
//SOUND SETTINGS
{
    SOUNDS.move.volume = 0.1;
    SOUNDS.enemykill.volume = 0.1;
    SOUNDS.notification.volume = 0.05;
    SOUNDS.click.volume = 0.1;
    SOUNDS.check.volume = 0.1;
    for (let key in SOUNDS) {
        //@ts-ignore
        let sound = SOUNDS[key];
        let play = sound.play;
        //@ts-ignore
        sound.play = () => {
            sound.pause();
            sound.currentTime = 0;
            play.call(sound);
        };
    }
}
//moinmaistä123
var FIGURE;
(function (FIGURE) {
    FIGURE["pawn"] = "pawn";
    FIGURE["knight"] = "knight";
    FIGURE["bishop"] = "bishop";
    FIGURE["rook"] = "rook";
    FIGURE["queen"] = "queen";
    FIGURE["king"] = "king";
})(FIGURE || (FIGURE = {}));
var COLOR;
(function (COLOR) {
    COLOR["white"] = "white";
    COLOR["black"] = "black";
})(COLOR || (COLOR = {}));
const CHESS_FIGURES_VALUES = { pawn: 1, knight: 3, bishop: 3, rook: 5, queen: 9, king: Number.POSITIVE_INFINITY };
function createLastMoveCover() {
    let lilcover = document.createElement("div");
    lilcover.classList.add("field-cover", "lastMove");
    return lilcover;
}
let lastMovedElements = [];
function highlightMoveElement(...ids) {
    lastMovedElements.forEach(e => {
        e.remove();
    });
    lastMovedElements = [];
    let f1 = null;
    let f2 = null;
    ids.forEach(id => {
        let fieldElement = chessBoard.fields[id].getElement();
        if (!f1)
            f1 = fieldElement;
        else if (!f2)
            f2 = fieldElement;
        let parent = fieldElement.parentElement;
        let lilcover = createLastMoveCover();
        parent.appendChild(lilcover);
        lastMovedElements.push(lilcover);
    });
}
function doMove(move) {
    let start = chessBoard.fields[move.start];
    if (!start.getFigure())
        return;
    let startFig = start.getFigure();
    //if (!OFFLINE_DEBUG) if (startFig.color != start.getRow().getBoard()?.getCurrentSide()) return;
    let end = chessBoard.fields[move.end];
    let f = end.getFigure();
    startFig.alreadyMoved = true;
    if (move.flag) {
        let flag = move.flag;
        if (flag.flag == FLAG.CASTLE) {
            end.setFigure(startFig);
            start.setFigure(null);
            let data = flag.data;
            let rook = chessBoard.fields[data[0]];
            chessBoard.fields[data[1]].setFigure(rook.getFigure());
            rook.setFigure(null);
        }
        else if (flag.flag == FLAG.PROMOTE) {
            startFig.type = FIGURE.queen;
            end.setFigure(startFig);
            start.setFigure(null);
        }
    }
    else {
        end.setFigure(startFig);
        start.setFigure(null);
    }
    //Check if enemy can move, if not the enemy king may be in check
    let vb = new VirtualBoard(chessBoard);
    let enemyColor = getOtherColor(startFig.color);
    if (vb.isFigureTypeUnderAttack(FIGURE.king, enemyColor)) {
        SOUNDS.check.play();
        showAlert(enemyColor.toLocaleLowerCase() + "'s king in check!", 3, true);
    }
    else if (vb.isFigureTypeUnderAttack(FIGURE.king, startFig.color)) {
        SOUNDS.check.play();
        showAlert(startFig.color.toLocaleLowerCase() + "'s king in check!", 3, true);
    }
    else {
        if (!f) {
            SOUNDS.move.play();
        }
        else {
            SOUNDS.enemykill.play();
        }
    }
    {
        highlightMoveElement(move.start, move.end);
        let div = chessBoard.addToMoveContainer(move, startFig, f);
        onhover(div, {
            start: () => {
                let w = 10;
                let s = "solid";
                start.getElement().style.border = `${s} ${w}px var(--hoverHighlightStart)`;
                end.getElement().style.border = `${s} ${w}px var(--hoverHighlightEnd)`;
                SOUNDS.click.play();
            },
            end: () => {
                function unhighlighHover(fig) { fig.getElement().style.border = ""; }
                unhighlighHover(start);
                unhighlighHover(end);
            },
            click: () => { }
        });
        setFocusOnDivWithId(document.getElementById("moves"));
    }
    let enemyFigures = vb.directPieces[enemyColor];
    let enemyCanMove = false;
    for (let i = 0; i < enemyFigures.length; i++) {
        let enemyFig = enemyFigures[i];
        if (!enemyFig)
            continue;
        let avaibleMovesEnemy = enemyFig.getAvaibleMoves(true);
        if (avaibleMovesEnemy.length > 0) {
            enemyCanMove = true;
            break;
        }
    }
    if (!enemyCanMove) {
        //If king is in check, we win, else its an draw
        if (vb.isFigureTypeUnderAttack(FIGURE.king, enemyColor)) {
            //We win
            chessBoard.endGame(startFig.color);
        }
        else {
            //Draw
            chessBoard.endGame(null);
        }
    }
    start.getRow().getBoard().toggleTurn();
}
class Team {
    constructor(color) {
        this.figures = [];
        this.color = color;
    }
}
function getOtherColor(color) {
    return color == COLOR.black ? COLOR.white : COLOR.black;
}
function ran(to) {
    return Math.floor(Math.random() * to);
}
function setFocusOnDivWithId(element) {
    element.scrollTo({ top: element.scrollHeight, behavior: 'smooth' });
}
const popup = document.getElementById("popup");
const cover = document.getElementById("cover");
cover.addEventListener("click", () => closePopup());
function closePopup() {
    popup.style.top = "-100%";
    cover.style.display = "none";
}
function openPopup() {
    cover.style.display = "block";
    let s = popup.style;
    s.transition = "0.5s";
    s.top = "50%";
    s.left = "calc(50%)";
    s.transform = "translate(-50%, -50%)";
}
function onhover(trigger, data) {
    let view = data.view;
    if (view) {
        trigger.addEventListener("mouseenter", e => trigger.appendChild(view));
        trigger.addEventListener("mouseleave", e => view.remove());
    }
    trigger.addEventListener("mouseenter", e => data.start());
    trigger.addEventListener("mouseleave", e => data.end());
    //@ts-ignore
    if (data.click)
        trigger.addEventListener("click", e => data.click());
}
function setCookie(cname, cvalue, exdays) {
    var d = new Date();
    d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
    var expires = "expires=" + d.toUTCString();
    document.cookie = cname + "=" + cvalue + ";" + expires + ";path=/";
}
function getCookie(cname) {
    var name = cname + "=";
    var decodedCookie = decodeURIComponent(document.cookie);
    var ca = decodedCookie.split(';');
    for (var i = 0; i < ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(name) == 0) {
            return c.substring(name.length, c.length);
        }
    }
    return undefined;
}
