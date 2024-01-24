# player
player is physically modeled wherever possible despite being represented abstractly. the player is composed of their body,a larger collider, linked with joint(s) to item(s) they are holding. player's physics has damping applied to simulate friction with the ground. the held items have lesser mass and no or minimal damping, as they are held in the air. in the case of weapons, this should increase responsiveness but also the inertial jiggle of the weapon as it is aimed and fired. dragged items should maintain generally the same damping as the body.

todo: add melee weapons via a Sharp(ness) component and damage calculated based on mass and velocity of swings

generalized action system processed for all Bodys
player movement integrated into this action system