#include <HID-Settings.h>
#include <HID-Project.h>

// STEERING WHEEL
const int STEERING_WHEEL_PIN = A0;

const int STEERING_WHEEL_DEAD_ZONE[2] = {20, 100};

const int STEERING_WHEEL_PIN_MAX_VALUE = 1024;
const int STEERING_WHEEL_PIN_CENTER_VALUE = STEERING_WHEEL_PIN_MAX_VALUE / 2;
const int STEERING_WHEEL_MAX_VALUE = STEERING_WHEEL_PIN_CENTER_VALUE - STEERING_WHEEL_DEAD_ZONE[1];
const float STEERING_WHEEL_STICK_COOF = 32767 / (float)STEERING_WHEEL_MAX_VALUE;

volatile float steering_wheel_last_value = 0.0;
volatile float steering_wheel_value = 0.0;

// TRANSMISSION & HANDBRAKE
const int TRANSMISSION_UP_PIN = 9;
const int TRANSMISSION_DOWN_PIN = 8;
const int HANDBRAKE_PIN = 10;

volatile bool transmission_up_pressed = false;
volatile bool transmission_down_pressed = false;
volatile bool handbrake_pressed = false;

// PEDALS
const int PEDAL_GAS_PIN = A2;
const int PEDAL_BRAKE_PIN = A1;
const int PEDAL_CLUTCH_PIN = A3;

const int PEDAL_GAS_DEAD_ZONE = 5;
const int PEDAL_BRAKE_DEAD_ZONE = 5;
const int PEDAL_CLUTCH_DEAD_ZONE = 5;

const int PEDAL_GAS_PIN_MAX_VALUE = 170;
const int PEDAL_GAS_MAX_VALUE = PEDAL_GAS_PIN_MAX_VALUE;
const int PEDAL_BRAKE_PIN_MAX_VALUE = 150;
const int PEDAL_BRAKE_MAX_VALUE = PEDAL_BRAKE_PIN_MAX_VALUE;
const int PEDAL_CLUTCH_PIN_MAX_VALUE = 110;
const int PEDAL_CLUTCH_MAX_VALUE = PEDAL_CLUTCH_PIN_MAX_VALUE;

const float PEDAL_GAS_TRIGGER_COOF = 32767 / (float)PEDAL_GAS_MAX_VALUE;
const float PEDAL_BRAKE_TRIGGER_COOF = 32767 / (float)PEDAL_BRAKE_MAX_VALUE;
const float PEDAL_CLUTCH_TRIGGER_COOF = 32767 / (float)PEDAL_CLUTCH_MAX_VALUE;

volatile float pedal_gas_value = 0.0;
volatile float pedal_gas_last_value = 0.0;
volatile float pedal_brake_value = 0.0;
volatile float pedal_brake_last_value = 0.0;
volatile float pedal_clutch_value = 0.0;
volatile float pedal_clutch_last_value = 0.0;

//
volatile bool gamepad_have_changes = false;

void setup() {
    pinMode(STEERING_WHEEL_PIN, INPUT);

    pinMode(TRANSMISSION_UP_PIN, INPUT_PULLUP);
    pinMode(TRANSMISSION_DOWN_PIN, INPUT_PULLUP);
    pinMode(HANDBRAKE_PIN, INPUT_PULLUP);

    pinMode(PEDAL_GAS_PIN, INPUT);
    pinMode(PEDAL_BRAKE_PIN, INPUT);
    pinMode(PEDAL_CLUTCH_PIN, INPUT);

    Serial.begin(9600);
    Gamepad.begin();
}

void loop() {
    steering_wheel_handler();
    transmission_and_handbrake_handler();
    pedals_handler();

    if (gamepad_have_changes) {
        Gamepad.write();
        gamepad_have_changes = false;
    }

    delay(10);
}

// The steering wheel handler
void steering_wheel_handler() {
    int value = analogRead(STEERING_WHEEL_PIN) - STEERING_WHEEL_PIN_CENTER_VALUE;

    // check dead zone (center):
    if (abs(value) < STEERING_WHEEL_DEAD_ZONE[0]) {
        steering_wheel_value = 0;
    }
    // check dead zone (right):
    else if (value >= STEERING_WHEEL_MAX_VALUE) {
        steering_wheel_value = STEERING_WHEEL_MAX_VALUE;
    }
    // check dead zone (left):
    else if (value <= -STEERING_WHEEL_MAX_VALUE) {
        steering_wheel_value = -STEERING_WHEEL_MAX_VALUE;
    }
    // aplying a value:
    else {
        steering_wheel_value = value - (value > 0 ? STEERING_WHEEL_DEAD_ZONE[0] : -STEERING_WHEEL_DEAD_ZONE[0]);
    }

    if (steering_wheel_value != steering_wheel_last_value) {
        Gamepad.xAxis(steering_wheel_value * STEERING_WHEEL_STICK_COOF);
        gamepad_have_changes = true;

        Serial.print("Steering wheel value: ");
        Serial.println(steering_wheel_value);

        steering_wheel_last_value = steering_wheel_value;
    }
}

// The transmission & handbrake handler
void transmission_and_handbrake_handler() {
    // transmission up:
    if (digitalRead(TRANSMISSION_UP_PIN) == LOW) {
        if (!transmission_up_pressed) {
            transmission_up_pressed = true;
            Gamepad.press(1);
            gamepad_have_changes = true;

            Serial.println("Transmission up pressed");
        }
    } else if (transmission_up_pressed) {
        transmission_up_pressed = false;
        Gamepad.release(1);
        gamepad_have_changes = true;

        Serial.println("Transmission up released");
    }

    // transmission down:
    if (digitalRead(TRANSMISSION_DOWN_PIN) == LOW) {
        if (!transmission_down_pressed) {
            transmission_down_pressed = true;
            Gamepad.press(2);
            gamepad_have_changes = true;

            Serial.println("Transmission down pressed");
        }
    } else if (transmission_down_pressed) {
        transmission_down_pressed = false;
        Gamepad.release(2);
        gamepad_have_changes = true;

        Serial.println("Transmission down released");
    }

    // handbrake:
    if (digitalRead(HANDBRAKE_PIN) == LOW) {
        if (!handbrake_pressed) {
            handbrake_pressed = true;
            Gamepad.press(3);
            gamepad_have_changes = true;
            
            Serial.println("Handbrake pressed");
        }
    } else if (handbrake_pressed) {
        handbrake_pressed = false;
        Gamepad.release(3);
        gamepad_have_changes = true;

        Serial.println("Handbrake released");
    }
}

// The pedals handler
void pedals_handler() {
    // PEDAL GAS:
    int gas_value = analogRead(PEDAL_GAS_PIN);

    // check dead zone
    if (gas_value < PEDAL_GAS_DEAD_ZONE) {
        pedal_gas_value = 0;
    }
    // check max value
    else if (gas_value >= PEDAL_GAS_MAX_VALUE) {
        pedal_gas_value = PEDAL_GAS_MAX_VALUE;
    }
    // applying value
    else {
        pedal_gas_value = gas_value - PEDAL_GAS_DEAD_ZONE;
    }
    
    if (pedal_gas_value != pedal_gas_last_value) {
        Gamepad.yAxis(pedal_gas_value * PEDAL_GAS_TRIGGER_COOF);
        gamepad_have_changes = true;

        Serial.print("Pedal 'gas' value: ");
        Serial.println(pedal_gas_value);

        pedal_gas_last_value = pedal_gas_value;
    }

    // PEDAL BRAKE:
    int brake_value = analogRead(PEDAL_BRAKE_PIN);

    // check dead zone
    if (brake_value < PEDAL_BRAKE_DEAD_ZONE) {
        pedal_brake_value = 0;
    }
    // check max value
    else if (brake_value >= PEDAL_BRAKE_MAX_VALUE) {
        pedal_brake_value = PEDAL_BRAKE_MAX_VALUE;
    }
    // applying value
    else {
        pedal_brake_value = brake_value - PEDAL_BRAKE_DEAD_ZONE;
    }
    
    if (pedal_brake_value != pedal_brake_last_value) {
        Gamepad.ryAxis(pedal_brake_value * PEDAL_BRAKE_TRIGGER_COOF);
        gamepad_have_changes = true;

        Serial.print("Pedal 'brake' value: ");
        Serial.println(pedal_brake_value);

        pedal_brake_last_value = pedal_brake_value;
    }

    // PEDAL CLUTCH:
    int clutch_value = analogRead(PEDAL_CLUTCH_PIN);

    // check dead zone
    if (clutch_value < PEDAL_CLUTCH_DEAD_ZONE) {
        pedal_clutch_value = 0;
    }
    // check max value
    else if (clutch_value >= PEDAL_CLUTCH_MAX_VALUE) {
        pedal_clutch_value = PEDAL_CLUTCH_MAX_VALUE;
    }
    // applying value
    else {
        pedal_clutch_value = clutch_value - PEDAL_CLUTCH_DEAD_ZONE;
    }
    
    if (pedal_clutch_value != pedal_clutch_last_value) {
        Gamepad.rxAxis(pedal_clutch_value * PEDAL_CLUTCH_TRIGGER_COOF);
        gamepad_have_changes = true;

        Serial.print("Pedal 'clutch' value: ");
        Serial.println(pedal_clutch_value);

        pedal_clutch_last_value = pedal_clutch_value;
    }
}
