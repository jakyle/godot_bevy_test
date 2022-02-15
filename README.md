![Project Logo](./godot/assets/godot-ferris-128x128.png)

# godot-bevy-test
> This is a godot-rust hybrid game. Run around as the bevy bird and avoid being collided with the godot-rust robot.
> 90% of the game
> logic lives in bevy-rust.  I use godot for the UI, and 
> all it's convenient nodes, but I manipulate it all within
> Bevy.

## Table of contents
- [godot-bevy-test](#godot-bevy-test)
  - [Table of contents](#table-of-contents)
  - [General Information](#general-information)
  - [Setup](#setup)
  - [Usage](#usage)
  - [Contact](#contact)

## General Information
So I really really really like working in ECS, it's a great developer experience and Bevy implements this design pattern really nicely. I created a Singleton "AutoLoad" script that creates the bevy App.  I then use Bevy's scheduler to loop when godot loops; I use the world to sync bevy with events.  

## Setup
Setup, installation, configuration, etc. Please read this [wiki](https://github.com/macalimlim/godot-rust-template/wiki) for setting up this template

## Usage
```shell
$ cargo make run
```
## Contact
Jakyle <jjackson360@gmail.com>
