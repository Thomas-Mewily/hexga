The hexga engine have the following goal:

- Multi media context 
  - Input: getting the external input (keyboard, mouse, controller...)
  - Pen: draw stuff on the screen (2D and 3D), 
  - Camera where the camera is
  - Sound: play music, sound effect
  - Rng random number generator
  - Telemetry : Bench the performance of some part of the code
  - Io ? open/save file (event based?)

- Define the program main loop design
  - Event driven
  - Easy async code without blocking (sending back an event later) (to target web, doing web call...) 
  - Ideally, the loop is `handle_event` -> `input` -> `update` -> `draw`
  - Easy online multiplayer (rollback or cloning game app state)

Problem:
How to decouple easily the game logic ?
- The game should be easy to be run manually without any graphics/sound...
- How to create window ? fetch the event (event iterator: easy to use, but not modular eought, it is easier to dispatch the event indivialy) 
- How to pass the multi media context ?
  - By mutable reference in every function ? (to heavy)
  - Singleton ? Hard to test, but a thread local singleton can do the tricks (max 1 simulation/game per thread?)
