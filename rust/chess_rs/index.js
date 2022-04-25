import init, { start } from "./pkg/chess_rs.js";
const wasm = await init();

function createCanvas(w, h) {
    const c = document.createElement("canvas");
    const ctx = c.getContext("2d");

    c.width = w;
    c.height = h;
    c.style = "border: solid 1px";
    document.body.appendChild(c);
    return ctx;
}



const GAME = start();
const CANV_2D = createCanvas(GAME.width, GAME.height);
const FPS_GOAL = 60,
    FPS_LABEL_UPDATER = 1;
const WASM_IMG_BUFFER = new ImageData(
    new Uint8ClampedArray(
        wasm.memory.buffer,
        GAME.img_buf_ptr,
        4 * GAME.width * GAME.height
    ),
    GAME.width,
    GAME.height
);

let frames = 0,
    msPerFrame = 0;
const fpscounter = (() => {
    let fpscounter = document.createElement("span");
    fpscounter.style = `position: fixed; top: 10px; left: 10px`;
    document.body.appendChild(fpscounter);
    return fpscounter;
})();

setInterval(() => {
    fpscounter.innerText = `${frames * FPS_LABEL_UPDATER} FPS / ${Math.round(msPerFrame/FPS_GOAL)}ms`;
    frames = 0;
    msPerFrame = 0;
}, 1000 / FPS_LABEL_UPDATER);

setInterval(() => {
    let start = Date.now();
    GAME.js_tick(start);
    msPerFrame += Date.now() - start;
    frames++;
    CANV_2D.putImageData(WASM_IMG_BUFFER, 0, 0);

}, 1000 / FPS_GOAL);

window.addEventListener("keydown", (ev) => GAME.keyevent(ev.key, true));
window.addEventListener("keyup", (ev) => GAME.keyevent(ev.key, false));