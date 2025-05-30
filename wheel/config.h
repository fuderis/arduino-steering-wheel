// ----------- STEERING WHEEL: --------------

const int WHEEL_PIN = A0;

const int WHEEL_DEGS_MAX = 1800;       // the max possible degs to the one side
const int WHEEL_DEGS_LIMIT = 420;      // the limit of degs to the one side [90, 180, 360, 420, 540, 720, 900, or another values..]

const float WHEEL_DEAD_ZONE = 0.90;     // [0.0, 100.0]


// ----------- FEEDBACK MOTOR: --------------

const int FEEDBACK_PWM_L_PIN = 7;
const int FEEDBACK_PWM_R_PIN = 8;
const int FEEDBACK_EN_L_PIN = 9;       // WARNING!: The pin timer must be set to 16-20 kHz, depending on the motor (see function setupPWMFrequency() in 'wheel/wheel.ino').
const int FEEDBACK_EN_R_PIN = 10;      // WARNING!: The pin timer must be set to 16-20 kHz, depending on the motor (see function setupPWMFrequency() in 'wheel/wheel.ino').

const float FEEDBACK_DEAD_ZONE = 5.0;  // [0.0, 100.0]

const float FEEDBACK_MIN_SPEED = 52.0;   // [0.0, 100.0]
const float FEEDBACK_MAX_SPEED = 65.0;   // [0.0, 100.0]


// -------------- PEDALS: -------------------

const int PEDAL_CLUTCH_PIN = A1;
const int PEDAL_GAS_PIN = A2;
const int PEDAL_BRAKE_PIN = A3;

const float PEDAL_CLUTCH_DEAD_ZONE = 12;  // [0.0, 100.0]
const float PEDAL_GAS_DEAD_ZONE = 10;     // [0.0, 100.0]
const float PEDAL_BRAKE_DEAD_ZONE = 10;   // [0.0, 100.0]

const int PEDAL_CLUTCH_MAX_VALUE = 60;
const int PEDAL_GAS_MAX_VALUE = 170;
const int PEDAL_BRAKE_MAX_VALUE = 150;


// ------ TRANSMISSION & HANDBRAKE: ----------

const int TRANSMISSION_UP_PIN = 11;
const int TRANSMISSION_DOWN_PIN = 12;
const int HANDBRAKE_PIN = 13;
