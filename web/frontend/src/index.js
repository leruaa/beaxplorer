import Epochs from "./Epochs.svelte";

let app = new Epochs({
  target: document.getElementById("main"),
});

export default app;