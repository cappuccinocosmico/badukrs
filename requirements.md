This is designed to be a frontend for running baduk/go games in the browser.

Main technology we are wanting to use for this is bevy, there is some example code in:

badukrs/2d

and here is some code for how to run this in the browser:

https://bevy-cheatbook.github.io/platforms/wasm.html

However there a couple things I want this to work with:

1. Have a board on the webpage that can actually play the game, let one side make movements, then switch it over to the other side and let you make moves with it. Like if you had two people at the same computer.

2. Once the page is loaded it should be possible to continuously interact with the webpage and play the game, even if the server is offline.

3. It should be able to run some bots, so you can play and practice offline. This is going to involve running some kind of neural network using webgpu.
