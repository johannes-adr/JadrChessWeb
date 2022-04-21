const defaultColors = "eyJ3IjoiI2Y4ZGNiNCIsImIiOiIjYjg4YzY0IiwiYSI6IiMyZWNjNzEiLCJsIjoiIzI0NzVjMCJ9";

interface BoardColorData {
    //whiteField
    w: string;
    //blackField
    b: string;
    //avaible;
    a: string;
    //LastMove;
    l: string;
}

function edit(e: { field: HTMLElement, chessField: ChessField }, prev: Element) {
    e.field.classList.add("colorPickerField");
    if (prev.classList.contains("avaibleMovePreview")) {
        e.chessField.highlight();
    } else if (prev.classList.contains("lastMovePreview")) {
        e.field.appendChild(createLastMoveCover());
    }
    return e.field;
}

function loadFieldPreviews() {
    {
        let wprevs = document.getElementsByClassName("whiteFieldPreview")
        let chessRow1 = new ChessRow(undefined!, 0);

        for (let i = 0; i < wprevs.length; i++) {
            let prev = wprevs[i];
            if(prev.childElementCount > 0)continue;
            prev.appendChild(edit(createField(chessRow1, true, 0), prev))
        }
    }
    {
        let wprevs = document.getElementsByClassName("blackFieldPreview")
        let chessRow1 = new ChessRow(undefined!, 0);

        for (let i = 0; i < wprevs.length; i++) {
            let prev = wprevs[i];
            if(prev.childElementCount > 0)continue;
            prev.appendChild(edit(createField(chessRow1, false, 0), prev))
        }
    }

}

loadFieldPreviews();


let colors = getBoardColor()

let whiteField: string = colors.white;
let blackField: string = colors.black;
let avaibleField: string = colors.avaible;
let lastMoveField: string = colors.lastMove;

function setWhite(v: string) {
    colors.setWhite(v);
    onUpdate();
    whiteField = v;
    whiteFieldPicker.value = v;
}

function setBlack(v: string) {
    colors.setBlack(v);
    onUpdate();
    blackField = v;
    blackFieldPicker.value = v;
}

function setAvaible(v: string) {
    colors.setAvaible(v);
    onUpdate();
    avaibleField = v;
    avaibleMoveFieldPicker.value = v;

}

function setLastMove(v: string) {
    colors.setLastMove(v);
    onUpdate();
    lastMoveField = v;
    lastMoveFieldPicker.value = v;
}

//@ts-ignore
let whiteFieldPicker: HTMLInputElement = document.getElementById("whiteFieldPicker")!;
whiteFieldPicker.value = colors.white;
whiteFieldPicker.oninput = e => {
    setWhite(whiteFieldPicker.value);
}
whiteFieldPicker.addEventListener("focusout", e => save())


//@ts-ignore
let blackFieldPicker: HTMLInputElement = document.getElementById("blackFieldPicker")!;
blackFieldPicker.value = colors.black;
blackFieldPicker.oninput = e => {
    setBlack(blackFieldPicker.value)
}
blackFieldPicker.addEventListener("focusout", e => save())



//@ts-ignore
let avaibleMoveFieldPicker: HTMLInputElement = document.getElementById("avaiblemoveFieldPicker")!;
avaibleMoveFieldPicker.value = colors.avaible;
avaibleMoveFieldPicker.oninput = e => {
    setAvaible(avaibleMoveFieldPicker.value);
}
avaibleMoveFieldPicker.addEventListener("focusout", e => save())

//@ts-ignore
let lastMoveFieldPicker: HTMLInputElement = document.getElementById("lastmoveFieldPicker")!;
lastMoveFieldPicker.value = colors.lastMove;
lastMoveFieldPicker.oninput = e => {
    setLastMove(lastMoveFieldPicker.value);
}
lastMoveFieldPicker.addEventListener("focusout", e => save())
//@ts-ignore
let boardcolorCode: HTMLTextAreaElement = document.getElementById("boardColorcode")!;
function onUpdate() {
    let colorData: BoardColorData = { w: whiteField, b: blackField, a: avaibleField, l: lastMoveField }
    boardcolorCode.value = btoa(JSON.stringify(colorData));

}

console.log(document.cookie);

let importBoardColor = document.getElementById("importBoardColor")!;
let reset = document.getElementById("boardColorReset")!;
reset.onclick = e => loadColor(defaultColors);
function save() {
    setCookie("colors", boardcolorCode.value, 356)
}
function loadColor(b64: string) {
    try {
        let jsonRaw = atob(b64);
        let json: BoardColorData = JSON.parse(jsonRaw);
        setWhite(json.w);
        setBlack(json.b);
        setAvaible(json.a);
        setLastMove(json.l);
        onUpdate();
        save();
    } catch (error) {
        console.error(error);
        showAlert("Error loading color code - string damaged");
    }
}

importBoardColor.onclick = e => {
    loadColor(boardcolorCode.value);
};

let colorCookie = getCookie("colors");
if (!colorCookie) {
    colorCookie = defaultColors;
}
loadColor(colorCookie);

fetch("/data/BoardColors.json").then(res => res.json()).then(dataSet => {
    let colorBoardList = document.getElementById("colorBoardList")!;
    console.log(dataSet);
    dataSet.forEach(data => {

        let dJson: BoardColorData = JSON.parse(atob(data.data));
        let colorElement = document.createElement("div");

        let text = document.createElement("span");
        text.innerText = data.name;
        text.style.marginRight = "10px";
        colorElement.appendChild(text);

        colorElement.classList.add("hstack", "colorElement");

        let colors = document.createElement("div");
        colors.classList.add("hstack");
        colorElement.appendChild(colors);
        function addElement(c: string) {
            let e = document.createElement("div");
            e.classList.add("small-colorpreview");
            e.style.backgroundColor = c;
            colors.appendChild(e);
            return e.style;
        }
        
        
        addElement(dJson.w).borderRadius = "10px 0 0 10px";
        addElement(dJson.b);
        addElement(dJson.a);
        addElement(dJson.l).borderRadius = "0 10px 10px 0";
        colorElement.onclick = e=>{
            loadColor(data.data);
        }
        colorBoardList.appendChild(colorElement);
    });
    loadFieldPreviews();
})
