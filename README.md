# Steering Wheel (Arduino -> Xbox360)

![Preview](readme/preview.png)

**This is an interface for software** that allows you to control homemade gaming steering wheels with pedals and a gearshift unit based on any Arduino microcontroller, even if it does not support HID interfaces.</br>

**Everything is thought out for maximum comfort**: the program filters signals, eliminates unnecessary noise and provides flexible dead zone settings for optimal control response.


## How does it work?

* **The firmware on the board**: simply **transmits the potentiometers and button data to the computer via the COM port**, and also **receives commands to control the force feedback motor**.
* **The software**: reads data from the COM port and **emulates a virtual Xbox 360 gamepad**, which is **fully supported by most games**. All of this runs at incredible speed, as it’s **written in the high-performance Rust programming language**.
* **Flexible settings**: steering wheel and pedal controls with smooth signal filtering, adjustable rotation angle, and dead zone configuration.
* **Force feedback**: the ability to change the strength, smoothness, and sensitivity of the response.
* **Easy configuration** via a convenient software interface.

![Preview](readme/presentation.png)


## Feedback:

You can find me [here](https://t.me/fuderis), also see my [telegram channel](https://t.me/fuderis_club).
I welcome your offers and feedback!

> Copyright (c) 2025 *Bulat Sh.* ([fuderis](https://t.me/fuderis))
