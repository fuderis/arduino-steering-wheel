#include <HID-Settings.h>
#include <HID-Project.h>

#include "config.h"

// STEERING WHEEL
const int STEERING_WHEEL_CENTER_VALUE = 1024 / 2;
const int STEERING_WHEEL_MAX_VALUE = STEERING_WHEEL_CENTER_VALUE - STEERING_WHEEL_DEAD_ZONE[1];
const float STEERING_WHEEL_STICK_COOF = 32767 / (float)STEERING_WHEEL_MAX_VALUE;

volatile float steering_wheel_last_value = 0.0;
volatile float steering_wheel_value = 0.0;

// STEERING WHEEL FEEDBACK MOTOR
const float STEERING_WHEEL_FEEDBACK_COOF = (STEERING_WHEEL_FEEDBACK_MAX_POWER - STEERING_WHEEL_FEEDBACK_MIN_POWER) / (float)STEERING_WHEEL_MAX_VALUE;

// TRANSMISSION & HANDBRAKE
volatile bool transmission_up_pressed = false;
volatile bool transmission_down_pressed = false;
volatile bool handbrake_pressed = false;

// PEDALS
const float PEDAL_GAS_STICK_COOF = 32767 / (float)PEDAL_GAS_MAX_VALUE;
const float PEDAL_BRAKE_STICK_COOF = 32767 / (float)PEDAL_BRAKE_MAX_VALUE;
const float PEDAL_CLUTCH_STICK_COOF = 32767 / (float)PEDAL_CLUTCH_MAX_VALUE;

volatile float pedal_gas_value = 0.0;
volatile float pedal_gas_last_value = 0.0;
volatile float pedal_brake_value = 0.0;
volatile float pedal_brake_last_value = 0.0;
volatile float pedal_clutch_value = 0.0;
volatile float pedal_clutch_last_value = 0.0;

#include "steering_wheel_handler.h"
#include "steering_wheel_feedback_handler.h"
#include "transmission_handbrake_handler.h"
#include "pedals_handler.h"

#include "demonstrative_start.h"

void setup() {
    pinMode(STEERING_WHEEL_PIN, INPUT);

    pinMode(STEERING_WHEEL_FEEDBACK_ENA_PIN, OUTPUT);
    pinMode(STEERING_WHEEL_FEEDBACK_IN1_PIN, OUTPUT);
    pinMode(STEERING_WHEEL_FEEDBACK_IN2_PIN, OUTPUT);

    pinMode(TRANSMISSION_UP_PIN, INPUT_PULLUP);
    pinMode(TRANSMISSION_DOWN_PIN, INPUT_PULLUP);
    pinMode(HANDBRAKE_PIN, INPUT_PULLUP);

    pinMode(PEDAL_GAS_PIN, INPUT);
    pinMode(PEDAL_BRAKE_PIN, INPUT);
    pinMode(PEDAL_CLUTCH_PIN, INPUT);

    Serial.begin(9600);
    Gamepad.begin();

    demonstrative_start();
}

void loop() {
    steering_wheel_handler();
    steering_wheel_feedback_handler();
    transmission_handbrake_handler();
    pedals_handler();

    delay(10);
}