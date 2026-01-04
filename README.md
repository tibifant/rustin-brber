# Player Rustin Brber

This Microservice Dungeon Player was my first time programming in Rust. There was a lot to learn and discover.

Whilst implementing Rustin Brber several issues with the original Code were found and together with the quick fixes reported (see: https://gitlab.com/the-microservice-dungeon/player-teams/skeletons/player-rust/-/issues). These impended the development process and hindered Rustin Brber to get to his best form in time. But still after a lot of overtime this is where we're at:

Rustin Brber evaluates the best Option for his Robots - he can:
- purchase Robots
- move Robots to their neighbouring planet with the best resource - and to discover new planets
- upgrade the Robot's MiningLevel
- mine Resources
- regenerate Energy
- attack other Robots (untested because of limited time)
- and purchase Health Restore! ---> nvm the command caused a wild error and I have no more time and patience and sleepless nights left to fix more failures of the skeleton. So this part got commented out.

Rustin Brber handles most of the GameEvents, but there are a few less important events still left to be implemented. Those can be found in `GameEventBodyType.rs`.

Rustin Brber could also still need some help with the purchasing of Upgrade Items besides Mining - as the priority was laid onto mining for now.

Known issues: When trying to join a game we will ever so often not be able to join with Error `Game with Id xxxx not found` or `Player or game not found`. This seems to originate from a race condition in the originally provided code.
