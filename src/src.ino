#include <HID-Settings.h>
#include <HID-Project.h>

#include "config.h"

#include "wheel_handler.h"
#include "wheel_feedback_handler.h"
#include "transmission_handbrake_handler.h"
#include "pedals_handler.h"

#include "demonstration.h"

void setup() {
    pinMode(WHEEL_PIN, INPUT);

    pinMode(WHEEL_FEEDBACK_IN1_PIN, OUTPUT);
    pinMode(WHEEL_FEEDBACK_IN2_PIN, OUTPUT);

    pinMode(TRANSMISSION_UP_PIN, INPUT_PULLUP);
    pinMode(TRANSMISSION_DOWN_PIN, INPUT_PULLUP);
    pinMode(HANDBRAKE_PIN, INPUT_PULLUP);

    pinMode(PEDAL_GAS_PIN, INPUT);
    pinMode(PEDAL_BRAKE_PIN, INPUT);
    pinMode(PEDAL_CLUTCH_PIN, INPUT);

    Serial.begin(9600);
    Gamepad.begin();

    demonstration();
}

void loop() {
    wheel_handler();
    wheel_feedback_handler();
    transmission_handbrake_handler();
    pedals_handler();

    delay(10);
}