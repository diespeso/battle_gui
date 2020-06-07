# battle_gui
Battle GUI experiment on rust using ggez. Plase don't use this, is a mess

# Branch for development of a basic animation system for transformation in sprites and clusters of sprites / drawables.
Heres a test:

![](https://github.com/diespeso/battle_gui/blob/old-state/animation_test.gif)

What can be seen here is: a gui component called StatusCard being animated after implementing an empty
Animatable trait on it, and also a Movable trait, which is the part that is animated.

### Only linear animations are planned to be implemented given that any other type would mean rewriting all the system
and honestly i dont think i'll need non linear animation types or ill try a workaroud, idk
