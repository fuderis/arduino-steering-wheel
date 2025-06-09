#include <HID-Settings.h>
#include <HID-Project.h>

#include "config.h"

const int PWM_MAX_VALUE = 799;

// Set PWM pins frequency
void setupPWMFrequency() {
    TCCR1A = 0;
    TCCR1B = 0;

    TCCR1A = (1 << COM1A1) | (1 << COM1B1) | (1 << WGM11);
    TCCR1B = (1 << WGM13) | (1 << WGM12) | (1 << CS10);

    ICR1 = PWM_MAX_VALUE; // 799 for 20 kHz (16MHz / (1 * (799 + 1)) )

    OCR1A = 0;
    OCR1B = 0;
}

// Analog write for pin 9
void analogWrite9(uint16_t duty) {
    OCR1A = constrain(duty, 0, PWM_MAX_VALUE);
}

// // Analog write for pin 10
// void analogWrite10(uint16_t duty) {
//     OCR1B = constrain(duty, 0, PWM_MAX_VALUE);
// }

#include "wheel_handler.h"
#include "feedback_handler.h"
#include "pedals_handler.h"
#include "transmission_handler.h"

#include "demonstration.h"

void setup() {
    pinMode(WHEEL_PIN, INPUT);

    setupPWMFrequency();

    pinMode(FEEDBACK_PWM_L_PIN, INPUT);
    pinMode(FEEDBACK_PWM_R_PIN, INPUT);
    pinMode(FEEDBACK_EN_L_PIN, OUTPUT);
    // pinMode(FEEDBACK_EN_R_PIN, OUTPUT);

    digitalWrite(FEEDBACK_PWM_L_PIN, LOW);
    digitalWrite(FEEDBACK_PWM_R_PIN, LOW);

    pinMode(TRANSMISSION_UP_PIN, INPUT_PULLUP);
    pinMode(TRANSMISSION_DOWN_PIN, INPUT_PULLUP);
    pinMode(HANDBRAKE_PIN, INPUT_PULLUP);

    pinMode(PEDAL_GAS_PIN, INPUT);
    pinMode(PEDAL_BRAKE_PIN, INPUT);
    pinMode(PEDAL_CLUTCH_PIN, INPUT);

    Serial.begin(9600);
    Gamepad.begin();

    // demonstration();
}

void loop() {
    wheel_handler();
    feedback_handler();
    pedals_handler();
    transmission_handler();

    delay(10);
}
