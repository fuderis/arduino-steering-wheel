# Steering Wheel Controller Firmware

## Overview

This firmware is designed to control a steering wheel setup with pedals and transmission controls based on the electronic board 'Arduino Leonardo'. It uses the [HID-Project](https://github.com/NicoHood/HID) (Human Interface Device) library to communicate with the computer as a gamepad.

The full gamepad API from the [HID-Project](https://github.com/NicoHood/HID) library, you can find [here](https://github.com/NicoHood/HID/wiki/Gamepad-API).

## Preferences (file [config.h](https://github.com/fuderis/arduino-steering-wheel/blob/main/wheel/config.h))

### Steering Wheel:
Realized by multi-turn resistor.

* WHEEL_PIN: Analog pin for the steering wheel potentiometer
* WHEEL_DEGS_MAX: Maximum angle of your steering wheel's potentiometer (in each direction)
* WHEEL_DEGS_LIMIT: Maximum steering angle (in each direction)
* WHEEL_DEAD_ZONE: Is an array defining the values of the dead zones

### Steering Wheel Feedback:
Realized by engine driver BTS7960.

* FEEDBACK_PWM_L_PIN: Digital pin for the feedback engine direction left
* FEEDBACK_PWM_R_PIN: Digital pin for the feedback engine direction right
* FEEDBACK_EN_L_PIN: Digital pin with interruptions (with '~' prefix) for the feedback engine input1 pin (recommended ~9)
* FEEDBACK_EN_R_PIN: Digital pin with interruptions (with '~' prefix) for the feedback engine input2 pin (recommended ~10)
* FEEDBACK_DEAD_ZONE: Constant defining dead zone for starting the engine moving
* FEEDBACK_MIN_SPEED: The minimum speed of the feedback engine
* FEEDBACK_MAX_SPEED: The maximum speed of the feedback engine

### Transmission & Handbrake:
Realized by buttons.

* TRANSMISSION_UP_PIN: Digital pin for the transmission up button
* TRANSMISSION_DOWN_PIN: Digital pin for the transmission down button
* HANDBRAKE_PIN: Digital pin for the handbrake button

### Pedals:
Realized by potentiometers.

* PEDAL_GAS_PIN: Analog pin for the gas pedal potentiometer
* PEDAL_BRAKE_PIN: Analog pin for the brake pedal potentiometer
* PEDAL_CLUTCH_PIN: Analog pin for the clutch pedal potentiometer
* PEDAL.._DEAD_ZONE: Constants defining dead zones for each pedals
* PEDAL.._MAX_VALUE: Maximum possible value for the displacement of your pedal potentiometers

## Usage
This firmware reads inputs from the steering wheel, pedals, and buttons, applies necessary scaling and dead zone adjustments, and can be configured to send these values as gamepad inputs.

# Licensing
Distributed under the MIT license.

# Feedback
You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This project is constantly evolving, and I welcome your suggestions and feedback.
