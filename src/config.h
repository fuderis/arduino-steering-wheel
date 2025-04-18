// STEERING WHEEL
const int STEERING_WHEEL_PIN = A0;

const int STEERING_WHEEL_DEAD_ZONE[2] = {30, 100};

// STEERING WHEEL FEEDBACK MOTOR
const int STEERING_WHEEL_FEEDBACK_ENA_PIN = 3;
const int STEERING_WHEEL_FEEDBACK_IN1_PIN = 4;
const int STEERING_WHEEL_FEEDBACK_IN2_PIN = 5;

const int STEERING_WHEEL_FEEDBACK_DEAD_ZONE = 10;

const float STEERING_WHEEL_FEEDBACK_MIN_POWER = 245;  // [0, 255]
const float STEERING_WHEEL_FEEDBACK_MAX_POWER = 255;  // [0, 255]

// TRANSMISSION & HANDBRAKE
const int TRANSMISSION_UP_PIN = 9;
const int TRANSMISSION_DOWN_PIN = 8;
const int HANDBRAKE_PIN = 10;

// PEDALS
const int PEDAL_GAS_PIN = A2;
const int PEDAL_BRAKE_PIN = A1;
const int PEDAL_CLUTCH_PIN = A3;

const int PEDAL_GAS_DEAD_ZONE = 10;
const int PEDAL_BRAKE_DEAD_ZONE = 10;
const int PEDAL_CLUTCH_DEAD_ZONE = 10;

const int PEDAL_GAS_MAX_VALUE = 170;
const int PEDAL_BRAKE_MAX_VALUE = 150;
const int PEDAL_CLUTCH_MAX_VALUE = 110;
