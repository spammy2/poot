# Poot
Photop api wrapper written in rust (not finished).

`photop-client` will soon be deprecated. You want to make bots? you use poot.

## Differences between photop-client and poot.
- poot is written in rust instead of javascript (:rocket: blazingly fast).
- The dependency "simplesocket" made independently, so the numerous bugs made in simplesocket will not be carried over.
- Less shared mutability bullshit.
- Use REST api instead of websockets.

This, and simplesocket.rs, are both heavily based on serenity.