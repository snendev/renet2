import * as wasm from 'echo_client_wt';

const app = new wasm.GameApp();
app.run();

// async function runLoop() {
//   while(true) {
//     app.update();
//     await new Promise((resolve) => {
//       setTimeout(resolve, 50);
//     })
//   }
// }

// init();

// function init() {
//   // get the chat elements
//   const chat = document.getElementById('chat-input');
//   const send = document.getElementById('chat-send');
//   const output = document.getElementById('chat-output');

//   // send message when button is clicked
//   send.addEventListener('click', () => {
//     if (app) {
//       messages_to_send.push(chat.value);
//       chat.value = '';
//     }
//   });

//   // send message when enter is pressed
//   chat.addEventListener('keyup', (e) => {
//     if (e.key === 'Enter') {
//       send.click();
//     }
//   });

  

// runLoop()
// }
