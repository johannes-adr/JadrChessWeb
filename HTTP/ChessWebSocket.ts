enum SystemMessages  {
    ERROR="ERROR",WARNING="WARNING",SUCCESS="SUCCESS"
}

let websocket: WebSocket = null!;
createWebSocket();

function createWebSocket(){
    if(websocket)websocket.close();
    websocket = new WebSocket(`ws://${location.hostname}:7721`)
}

websocket.onmessage = e => {
    let res = JSON.parse(e.data);
    if ("error" in res) {
        console.error(res.error);
        return;
    }
    console.log(res);
    switch (res.type) {
        case "lobbycreated":
            onLobbyCreate(res);
            break;
        case "move":
            console.log("GOT MOVE: ", res);
            doMove(res)
            break;
        case "lobbyJoined":
            onLobbyJoin(res);
            break;
        case "chatmessage":
            receiveChatMessage(res.args[0])
            break;
        case "fenMap":
            chessBoard.loadFen(res.fen)
            break;
        case "kiinfo":
            onKiInfo(res);
            break;
        default:
            break;
    }
}

websocket.onopen = e => {
    //@ts-ignore
    let queue: PACKET[] = websocket.queue;
    if (queue) {
        queue.forEach(p => sendPacket(p));
        //@ts-ignore
        websocket.queue = [];
    }
}

function onKiInfo(data: {turndata: string, sitestrenght: Number, FEN: string}){
    document.getElementById("kimsg")!.innerText = data.turndata;
    console.log(data.FEN);
    document.getElementById("boardInfo")!.innerHTML = `<br><div style="display: flex; align-items: center">FEN: <textarea>${data.FEN}</textarea></div><br><br>Lead - ${data.sitestrenght}`
}


function onLobbyCreate(data: LOBBY_PACKET) {
    
    chessBoard.thisInstanceColor = COLOR.white;
    editServerInfoData(data);
    showAlert("Lobby created!", 5, false);
}

function onLobbyJoin(data: LOBBY_PACKET){
    console.log(data);
    //@ts-ignore
    chessBoard.thisInstanceColor = data.status;
    editServerInfoData(data);
    chessBoard.rotate(180);
    showAlert("Joined lobby " + data.lobbyid,5,false);
}

function receiveChatMessage(obj: any) {
    let msg = obj.msg;
    console.log(obj)
    let msgSpan = document.createElement("div");
    msgSpan.classList.add("chatMessage");
    if("systemmessage" in obj){
        let systemMessageDiv = document.createElement("div");
        let systemMessage: SystemMessages = obj.systemmessage;
        console.log(systemMessage);
        switch (systemMessage) {
            case SystemMessages.SUCCESS:
                systemMessageDiv.style.color = "var(--green)";
                break;
            case SystemMessages.ERROR:
                systemMessageDiv.style.color = "var(--flatred)";
                break;
            case SystemMessages.WARNING:
                systemMessageDiv.style.color = "var(--yellow)";
                break;
        }
        systemMessageDiv.innerText = msg;
        msgSpan.appendChild(systemMessageDiv);
    }else{
        msgSpan.innerText = msg;
    }
    
    chatEntries.appendChild(msgSpan);
    setFocusOnDivWithId(chatEntries);
    SOUNDS.notification.play();
}


function editServerInfoData(data: LOBBY_PACKET){
    let serverTable = document.getElementById("servertable")!;
    let createLobbyBtn = document.getElementById("createLobby")!;
    createLobbyBtn.remove();
    serverTable.appendChild(addInfoField("LobbyID", data.lobbyid.substr(0,10)+"...", () => {
        copyToClipboard(`http://${location.host}?lobbyid=` + data.lobbyid);
        showAlert("Copied joinlink to clipboard", 5, true);
    }));
    serverTable.appendChild(addInfoField("Your color", chessBoard.thisInstanceColor.toLocaleLowerCase()));

    //CHAT

    serverTable.appendChild(createChatElement());
    
    let kiMsg = document.createElement("span");
    kiMsg.id = "kimsg";
    kiMsg.classList.add("infodata");
    serverTable.appendChild(kiMsg);
    LOBBYID = data.lobbyid;
}

let chatEntries: HTMLDivElement = null!;
function createChatElement(){
    let chatRoot = document.createElement("div");
    chatRoot.id = "chatRoot"
    chatRoot.classList.add("vstack");
    chatEntries = document.createElement("div");
    chatEntries.id = "chatEntries"

    let chatControle = document.createElement("div");
    chatControle.classList.add("hstack");

    let chatInput = document.createElement("input");
    chatInput.setAttribute("placeholder", "write something nice...");
    chatInput.id = "chatInput"

    let chatSubmit = document.createElement("button");
    chatSubmit.id = "chatSubmit";
    chatSubmit.innerHTML = "<img src='/assets/send.svg' class='svgIcon'>"

    function send(){
        sendPacket({type: "chatmessage", args: [{msg: chatInput.value}]});
        chatInput.value = "";
    }

    chatSubmit.onclick = e=>{
        send();
    }

    chatInput.addEventListener("keydown", e=>{
        if(e.code == "Enter")send();
    });

    chatControle.appendChild(chatInput);
    chatControle.appendChild(chatSubmit);

    

    chatRoot.appendChild(chatEntries);
    chatRoot.appendChild(chatControle);
    
    return chatRoot;
}
//document.getElementById("servertable")!.append(createChatElement());
/*
document.getElementById("servertable")!.append(addInfoField("hi", "how are you?"));
document.getElementById("servertable")!.append(addInfoField("Your color", "black     dasd"));
*/
function addInfoField(title: string, text: string, onclick?: Function) {
    let root = document.createElement("div");
    root.classList.add("infofield", "hstack");

    let textHolder = document.createElement("span");
    textHolder.classList.add("infofield-text");
    textHolder.innerText = text;
    textHolder.addEventListener("click", e => {
        if (onclick) onclick();
        else {
            copyToClipboard(text);
            showAlert(`Copied '${title}'<br> to clipboard!`, 5, false);
        }
    });

    let titleHolder = document.createElement("span");
    titleHolder.classList.add("infofield-title");
    titleHolder.innerText = title;
    root.appendChild(titleHolder);
    root.appendChild(textHolder);
    return root;
}


function createLobby() {
    
    sendPacket({ type: "createLobby" })
}

function sendPacket(packet: PACKET) {
    //@ts-ignore
    packet.lobbyid = LOBBYID;
    if (websocket.OPEN === websocket.readyState) {
        
        websocket.send(JSON.stringify(packet));
    } else if (websocket.CONNECTING === websocket.readyState) {
        if ("queue" in websocket) {
            //@ts-ignore
            websocket.queue.push(packet);
        } else {
            //@ts-ignore
            websocket.queue = [packet];
        }
    }

}



interface PACKET {
    type: string
    args?: any[]
}

interface LOBBY_PACKET {
    lobbyid: string;
}



//CONNECT TO LOBBY IF GET PARAM CONTAINS LOBBYID
var url = new URL(window.location.href);
var LOBBYID = url.searchParams.get("lobbyid");
if (LOBBYID) {
    sendPacket({ type: "joinLobby", args: [LOBBYID] })
}




