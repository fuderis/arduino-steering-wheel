#include <HID-Settings.h>
#include <HID-Project.h>

#include "config.h"

#include "wheel_handler.h"
#include "feedback_handler.h"
#include "pedals_handler.h"
#include "transmission_handler.h"

#include "demonstration.h"

void setup() {
    pinMode(WHEEL_PIN, INPUT);

    // configuring the Timer1 for Fast PWM with frequency ~7.8 kHz
    TCCR1A = _BV(COM1A1) | _BV(COM1B1) | _BV(WGM11);
    TCCR1B = _BV(WGM13) | _BV(WGM12) | _BV(CS12);
    ICR1 = 80;  // 16MHz / (256 * 80) = 7.8125 kHz

    pinMode(FEEDBACK_IN1_PIN, INPUT);
    pinMode(FEEDBACK_IN2_PIN, INPUT);

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
    feedback_handler();
    pedals_handler();
    transmission_handler();

    delay(10);
}