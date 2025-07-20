#define WHEEL_PIN               A0

#define FEEDBACK_L_PIN          8
#define FEEDBACK_R_PIN          7
#define FEEDBACK_PWM_PIN        9      // WARNING!: The pin timer must be set to 16-20 kHz, depending on the motor
#define FEEDBACK_PWM_MAX_VALUE  799    // WARNING!: The value 799 is needed to operate at a frequency of 16kHz. (don't touch it if you don't know how)

#define PEDAL_GAS_PIN           A3
#define PEDAL_BRAKE_PIN         A1
#define PEDAL_CLUTCH_PIN        A2

#define TRANSMISSION_UP_PIN     4
#define TRANSMISSION_DOWN_PIN   5
#define HANDBRAKE_PIN           6

// Send steering wheel data to COM port
void sendState() {
    int wheel = analogRead(WHEEL_PIN);
    int gas = analogRead(PEDAL_GAS_PIN);
    int brake = analogRead(PEDAL_BRAKE_PIN);
    int clutch = analogRead(PEDAL_CLUTCH_PIN);
    bool up = digitalRead(TRANSMISSION_UP_PIN) == LOW;
    bool down = digitalRead(TRANSMISSION_DOWN_PIN) == LOW;
    bool handbrake = digitalRead(HANDBRAKE_PIN) == LOW;

    Serial.print("{\"wheel\":");
    Serial.print(wheel);
    Serial.print(",\"gas\":");
    Serial.print(gas);
    Serial.print(",\"brake\":");
    Serial.print(brake);
    Serial.print(",\"clutch\":");
    Serial.print(clutch);
    Serial.print(",\"up\":");
    Serial.print(up ? "true" : "false");
    Serial.print(",\"down\":");
    Serial.print(down ? "true" : "false");
    Serial.print(",\"handbrake\":");
    Serial.print(handbrake ? "true" : "false");
    Serial.println("}");
}

// Turn off's motor
void turnOffMotor() {
    analogWrite9(0);
    delay(10);  // for short circuit protection

    pinMode(FEEDBACK_L_PIN, INPUT);
    digitalWrite(FEEDBACK_L_PIN, LOW);

    pinMode(FEEDBACK_R_PIN, INPUT);
    digitalWrite(FEEDBACK_R_PIN, LOW);
}

unsigned long lastValidFeedback = 0;
bool lastDirection = false;  // false=left, true=right

// Handle feedback motor commands from COM port
void handleFeedback(const String& cmd) {
    String trimmed = cmd;
    trimmed.trim();

    // validation JSON data:
    if (trimmed.startsWith("{") && trimmed.endsWith("}")) {
        lastValidFeedback = millis();
    } else {
        return;
    }
    
    // parsing power:
    int power = 0;
    if(int idx = trimmed.indexOf("\"power\":"); idx != -1) {
        power = trimmed.substring(idx + 8).toInt();
    } else {
        power = 0;
    }
    
    // parsing direction and set power:
    if (power > 0 && trimmed.indexOf("\"motor\"") != -1) {
        // to left:
        if (trimmed.indexOf("\"left\"") != -1) {
            // from right to left:
            if (lastDirection == true) {
                analogWrite9(0);
                delay(10);  // for short circuit protection
                lastDirection = false;
            }
            
            pinMode(FEEDBACK_L_PIN, OUTPUT);
            digitalWrite(FEEDBACK_L_PIN, HIGH);
            
            pinMode(FEEDBACK_R_PIN, INPUT);
            digitalWrite(FEEDBACK_R_PIN, LOW);

            analogWrite9(power);
        }
        // to right:
        else if (trimmed.indexOf("\"right\"") != -1) {
            // from left to right:
            if (lastDirection == false) {
                analogWrite9(0);
                delay(10);  // for short circuit protection
                lastDirection = true;
            }
            
            pinMode(FEEDBACK_L_PIN, INPUT);
            digitalWrite(FEEDBACK_L_PIN, LOW);

            pinMode(FEEDBACK_R_PIN, OUTPUT);
            digitalWrite(FEEDBACK_R_PIN, HIGH);

            analogWrite9(power);
        }
        // to center, turn off motor:
        else {
            turnOffMotor();
        }
    }
    // no power, turn off motor:
    else {
        turnOffMotor();
    }
}

// Analog write for pin 9
void analogWrite9(uint16_t duty) {
    OCR1A = constrain(duty, 0, FEEDBACK_PWM_MAX_VALUE);
}

/*
// Analog write for pin 10
void analogWrite10(uint16_t duty) {
    OCR1B = constrain(duty, 0, FEEDBACK_PWM_MAX_VALUE);
}
*/

void setup() {
    // init wheel pin:
    pinMode(WHEEL_PIN, INPUT);

    // setup PWM frequency (pins 9,10):
    TCCR1A = 0;
    TCCR1B = 0;
    TCCR1A = (1 << COM1A1) | (1 << COM1B1) | (1 << WGM11);
    TCCR1B = (1 << WGM13) | (1 << WGM12) | (1 << CS10);

    ICR1 = FEEDBACK_PWM_MAX_VALUE; // 799 for 20 kHz (16MHz / (1 * (799 + 1)) )

    OCR1A = 0;
    OCR1B = 0;

    // init feedback pins:
    pinMode(FEEDBACK_L_PIN, INPUT);
    pinMode(FEEDBACK_R_PIN, INPUT);
    pinMode(FEEDBACK_PWM_PIN, OUTPUT);

    digitalWrite(FEEDBACK_L_PIN, LOW);
    digitalWrite(FEEDBACK_R_PIN, LOW);

    // init transmission pins:
    pinMode(TRANSMISSION_UP_PIN, INPUT_PULLUP);
    pinMode(TRANSMISSION_DOWN_PIN, INPUT_PULLUP);
    pinMode(HANDBRAKE_PIN, INPUT_PULLUP);

    // init pedal pins:
    pinMode(PEDAL_GAS_PIN, INPUT);
    pinMode(PEDAL_BRAKE_PIN, INPUT);
    pinMode(PEDAL_CLUTCH_PIN, INPUT);

    Serial.begin(115200);
}

const unsigned long responseTimeout = 500;
static unsigned long lastSend = 0;

void loop() {
    // send state:
    if (millis() - lastSend > 10) {  // sending frequency ~100 Hz
        sendState();
        lastSend = millis();
    }

    // handle response:
    static String input;
    while (Serial.available()) {
        char c = Serial.read();

        if (c == '\n' || c == '\r') {
            if (input.length() > 0) {
                handleFeedback(input);
                input = "";
            }
        } else {
            input += c;
            if (input.length() > 100) input = "";  // reset if response is too long
        }
    }

    // connection failed, turn off motor:
    if (millis() - lastValidFeedback > responseTimeout) {
        turnOffMotor();
    }
}
