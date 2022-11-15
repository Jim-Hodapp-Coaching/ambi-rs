let temperatureTd = document.getElementById('temperature');
let humidityTd = document.getElementById('humidity');
let pressureTd = document.getElementById('pressure');
let dustConcentrationTd = document.getElementById('dust_concentration');
let airPurityTd = document.getElementById('air_purity');
let statusTd = document.getElementById('status');

var STATE = {
    connected: false,
  }

// Subscribe to the event source at `uri` with exponential backoff reconnect.
function subscribe(uri) {
    var retryTime = 1;
  
    function connect(uri) {
      const events = new EventSource(uri);
  
      events.addEventListener("message", (ev) => {
        console.log("raw data", JSON.stringify(ev.data));
        console.log("decoded data", JSON.stringify(JSON.parse(ev.data)));
        const msg = JSON.parse(ev.data);
        if (!"temperature" in msg && !"humidity" in msg && !"pressure" in msg
            && !"dust_concentration" in msg && !"air_purity" in msg) return;
        temperatureTd.innerText = msg.temperature + ' °C';
        humidityTd.innerText = msg.humidity + ' %';
        pressureTd.innerText = msg.pressure + ' mbars';
        dustConcentrationTd.innerText = msg.dust_concentration + ' pcs/ltr';
        airPurityTd.innerText = msg.air_purity;
      });
  
      events.addEventListener("open", () => {
        setConnectedStatus(true);
        console.log(`connected to event stream at ${uri}`);
        retryTime = 1;
      });
  
      events.addEventListener("error", () => {
        setConnectedStatus(false);
        events.close();
  
        let timeout = retryTime;
        retryTime = Math.min(64, retryTime * 2);
        console.log(`connection lost. attempting to reconnect in ${timeout}s`);
        setTimeout(() => connect(uri), (() => timeout * 1000)());
      });
    }
  
    connect(uri);
}

// Set the connection status: `true` for connected, `false` for disconnected.
function setConnectedStatus(status) {
    STATE.connected = status;
    statusTd.innerText = (status) ? "connected" : "reconnecting";
  }

function init() {
    console.log(`Setting up in init()`);

    // Subscribe to server-sent events.
    subscribe("/events");
}

init()