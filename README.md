# gravi-bodies - 2d Gravitational Bodies Simulation
Requires SFML to be installed along with the rust sfml bindings

`sudo apt-get install libsfml-dev`

`cargo add sfml`

## How to Build
gravi-bodies is built using cargo with the following command

`cargo build`
## How to Run
gravi-bodies needs to be run within the `gravi-bodies/` folder in order for filepaths to work properly. gravi-bodies can be run through cargo, or through the binary that was built using cargo. A screenshot of the window as it currently appears with be saved to the current working directory upon closing.

`cargo run -- <path-to-scenario-file> <[flags]>`

`./target/debug/gravi-bodies <path-to-scenario-file> <[flags]>`

gravi-bodies is run by building scenarios described in formatted text files explained below. These text files tell the program where to place each body, how big it is, what color it is, and it's starting velocity. Once the program starts, these bodies begin to interact with each other according to the gravitational formula.

Note: G has been changed to increase the effect of gravity since distances will not be able to be as large as they are in space

$$ G = 6.674 * 10^-3 $$

$$ F = G {{ m_1 * m_2 } \over r^2} $$

### Body Types
Each body that you create in these text files can be 1 of 4 different body types. The below table describes the interactions between each body type. a `+` means that they attract each other, and a `-` means that they repel each other.
| | NEUTRAL | POSITIVE | NEGATIVE | WEIRD |
|-|---------|----------|----------|-------|
| NEUTRAL | + | + | + | - |
| POSITIVE | + | - | + | - |
| NEGATIVE | + | + | - | - |
| WEIRD | - | - | - | - |

## Scenario Files
Scenario files describe the bodies that interact with each other within the scenario. Each value is seperated by spaces while different bodies are seperated by newlines. RGB here describes the color of the circle as it appears in the window. The expected layout is as follows:
| Body Type | Mass | Starting X Position | Starting Y Position | Starting X Speed | Starting Y Speed | Radius of Circle | R | G | B |
|-|-|-|-|-|-|-|-|-|-|

Note: Lines beginning with `#` will be ignored

A few examples are provided.

## Command Line Options
### -d --draw-trails
This option enables drawing the trail of the body as it moves through space

Passing in this flag will by default only draw the trail from the most recent 50 frames of travel per body. You may pass in a value using the syntax:
`-d=#` or `--draw-trails=#` to set your own number of frames to draw the trail from.

ex. `-d=200` to draw the trail for the most recent 200 frames per body

## Example Run
`cargo run -- scenarios/stable_2 -d=200`

Not very stable, but very cool
![stable_2](https://github.com/lglista/gravi-bodies/assets/32312607/dd9ccaa1-6580-49bd-aea3-63b7a8697109)