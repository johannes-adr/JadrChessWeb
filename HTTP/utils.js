const alertbox = document.getElementById("alertBox");
let alertqueue = [];
let isCurrentlyAlertOpen = false;
function showAlert(msg, seconds, iserror) {
    if(alertqueue > 0 || isCurrentlyAlertOpen){
        alertqueue.push({msg: msg, seconds: seconds, iserror: iserror});
        return;
    }
    if(seconds == undefined)seconds = 4;
    if(iserror != undefined){
        if(iserror){
            document.getElementById("alertBoxTXT").innerHTML = "<span style='color: var(--flatred)'>"+msg+"</span>";
        }else{ 
            document.getElementById("alertBoxTXT").innerHTML = `<span style='color: var(--green)'>${msg}</span>`;
        }
        
    }else{
        document.getElementById("alertBoxTXT").innerHTML = msg;
    }
    
    alertbox.style.transition = "0.5s";
    alertbox.style.bottom = "20px";
    setTimeout(() => {
        if(alertqueue.length > 0){
            let alert = alertqueue[0];
            alertqueue.shift();
            hidealert();
            showAlert(alert.msg,alert.seconds,alert.iserror);
        }else{
            hidealert();
        }
    }, seconds * 1000);
    isCurrentlyAlertOpen = true;
}

function hidealert() {
    alertbox.style.transition = "0.5s";
    alertbox.style.bottom = "-50%";
    isCurrentlyAlertOpen = false;
}

function fetchJson(url, callback) {
    fetch(url)
        .then(response => response.json())
        .then(json => callback(json, null))
        .catch(error => callback(null, error))
}

function copyToClipboard(message) {
    var textArea = document.createElement("textarea");
    textArea.value = message;
    textArea.style.opacity = "0";
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();

    try {
        var successful = document.execCommand('copy');
        successful ? showAlert("copied!",false) : showAlert('an unexpected error occured', 3, true);
    } catch (err) {
        showAlert('Unable to copy value , error : ' + err.message, 4, true);
    }

    document.body.removeChild(textArea);
}