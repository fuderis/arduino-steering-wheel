const float PEDAL_GAS_STICK_COOF = 32767 / (float)PEDAL_GAS_MAX_VALUE;
const float PEDAL_BRAKE_STICK_COOF = 32767 / (float)PEDAL_BRAKE_MAX_VALUE;
const float PEDAL_CLUTCH_STICK_COOF = 32767 / (float)PEDAL_CLUTCH_MAX_VALUE;

volatile float pedal_gas_value = 0.0;
volatile float pedal_gas_last_value = 0.0;
volatile float pedal_brake_value = 0.0;
volatile float pedal_brake_last_value = 0.0;
volatile float pedal_clutch_value = 0.0;
volatile float pedal_clutch_last_value = 0.0;

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
        Gamepad.yAxis(pedal_gas_value * PEDAL_GAS_STICK_COOF);
        Gamepad.write();

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
        Gamepad.ryAxis(pedal_brake_value * PEDAL_BRAKE_STICK_COOF);
        Gamepad.write();

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
        Gamepad.rxAxis(pedal_clutch_value * PEDAL_CLUTCH_STICK_COOF);
        Gamepad.write();

        Serial.print("Pedal 'clutch' value: ");
        Serial.println(pedal_clutch_value);

        pedal_clutch_last_value = pedal_clutch_value;
    }
}
