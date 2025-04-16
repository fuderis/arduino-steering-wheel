# Steering Wheel Controller Firmware

## Overview

This firmware is designed to control a steering wheel setup with pedals and transmission controls based on the electronic board 'Arduino Leonardo'. It uses the [HID-Project](https://github.com/NicoHood/HID) (Human Interface Device) library to communicate with the computer as a gamepad.

The full gamepad API from the [HID-Project](https://github.com/NicoHood/HID) library, you can find [here](https://github.com/NicoHood/HID/wiki/Gamepad-API).

## Preferences (file config.h)

### Steering Wheel:
* STEERING_WHEEL_PIN: The analog pin for the steering wheel potentiometer
* STEERING_WHEEL_DEAD_ZONE: The array defining the dead zone values [START, END]

### Steering Wheel Feedback:
* STEERING_WHEEL_FEEDBACK_ENA_PIN: The digital pin with interruptions (with '~' prefix) for the feedback motor ENA pin
* STEERING_WHEEL_FEEDBACK_IN1_PIN: The digital pin for the feedback motor input1 pin
* STEERING_WHEEL_FEEDBACK_IN2_PIN: The digital pin for the feedback motor input2 pin
* STEERING_WHEEL_FEEDBACK_DEAD_ZONE: Constant defining dead zone for starting the motor moving
* STEERING_WHEEL_FEEDBACK_MIN_POWER: The minimal power of the feedback motor [0, 255]
* STEERING_WHEEL_FEEDBACK_MAX_POWER: The maximum power of the feedback motor [0, 255]

### Transmission & Handbrake:
* TRANSMISSION_UP_PIN: The digital pin for the transmission up button
* TRANSMISSION_DOWN_PIN: The digital pin for the transmission down button
* HANDBRAKE_PIN: The digital pin for the handbrake button

### Pedals:
* PEDAL_GAS_PIN: The analog pin for the gas pedal potentiometer
* PEDAL_BRAKE_PIN: The analog pin for the brake pedal potentiometer
* PEDAL_CLUTCH_PIN: The analog pin for the clutch pedal potentiometer
* PEDAL.._DEAD_ZONE: Constants defining dead zones for each pedals
* PEDAL.._MAX_VALUE: The maximum possible value for the displacement of your pedal's potentiometers

## Usage
This firmware reads inputs from the steering wheel, pedals, and buttons, applies necessary scaling and dead zone adjustments, and can be configured to send these values as gamepad inputs.

# Licensing
Distributed under the MIT license.

# Feedback
You can contact me via GitHub or send a message to my Telegram [@fuderis](https://t.me/fuderis).

This project is constantly evolving, and I welcome your suggestions and feedback.
