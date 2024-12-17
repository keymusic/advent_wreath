Advent Wreath
=============

An electronic Advent wreath with four "candles" based on the _Arduino Uno_.

The _Arduino Uno_ Revision 3 is equipped with a prototyping shield with the following components:

| Arduino name | ATmega328P name | component          |
|--------------|-----------------|--------------------|
| A0, D14      | PC0             | red push button    |
| A1, D15      | PC1             | yellow push button |
| A2, D16      | PC2             | green push button  |
| A3, D17      | PC3             | blue push button   |
| D9           | PB1 (PWM OC1A)  | red LED            |
| D10          | PB2 (PWM OC1B)  | yellow LED         |
| D11          | PB3 (PWM OC2A)  | green LED          |
| D3           | PD3 (PWM OC2B)  | blue LED           |

The push buttons are normally open and connect to ground when pressed (inverted logic).
**Attention:** The push buttons are not debounced.

The LEDs are sourced by the related output pins (normal logic). The selected pins can be used as an output of a Pulse Width Modulation (PWM) unit:
- The outputs OC1A and OC1B connect to a 16-bit PWM unit.
- The outputs OC2A and OC2B connect to a 8-bit PWM unit.

## Dependencies
This application uses the Hardware Abstraction Layer by Rahix for the Atmel / Microchip AVR microcontrollers. For further information see [`avr-hal` README].

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme