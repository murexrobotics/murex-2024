const dgram = require("dgram");
const server = dgram.createSocket("udp4");
let num = 0
server
  .on("connection", function (steam) {
    console.log("connected")
  })
  .on("message", function (msg) {
    // Displaying the client message
    let json = JSON.parse(msg.toString())
    console.log(`Client Message ${++num}: ` + json);

  })
  .bind(5555, "127.0.0.1", () => {
    console.log("server started")
  })