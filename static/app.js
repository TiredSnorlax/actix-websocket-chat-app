let socket;

const connect = () => {
  const { location } = window;

  const proto = location.protocol.startsWith("https") ? "wss" : "ws";
  const wsUri = `${proto}://${location.host}/ws/67e55044-10b1-426f-9247-bb680e5fe0c8`;

  socket = new WebSocket(wsUri);

  socket.onopen = () => {
    console.log("Connected");
  };

  socket.onclose = () => {
    console.log("Disconnected");
  };

  socket.onmessage = (ev) => {
    console.log(`Received: ${ev.data}`);
  };
};

connect();
