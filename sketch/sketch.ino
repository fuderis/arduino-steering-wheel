#define WHEEL_PIN               A0

#define FEEDBACK_L_PIN          8
#define FEEDBACK_R_PIN          7
#define FEEDBACK_PWM_PIN        9      // WARNING!: The pin timer must be set to 16-20 kHz, depending on the motor (see function setupPWMFrequency() in 'wheel/wheel.ino').
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

// Handle feedback motor commands from COM port
void handleFeedback(const String& cmd) {
    if(cmd.indexOf("\"motor\"") != -1) {
        // to left:
        if (cmd.indexOf("\"left\"") != -1) {
            pinMode(FEEDBACK_L_PIN, OUTPUT);
            digitalWrite(FEEDBACK_L_PIN, HIGH);
            
            pinMode(FEEDBACK_R_PIN, INPUT);
            digitalWrite(FEEDBACK_R_PIN, LOW);
        }
        // to right:
        else if (cmd.indexOf("\"right\"") != -1) {
            pinMode(FEEDBACK_L_PIN, INPUT);
            digitalWrite(FEEDBACK_L_PIN, LOW);

            pinMode(FEEDBACK_R_PIN, OUTPUT);
            digitalWrite(FEEDBACK_R_PIN, HIGH);
        }
        // center:
        else {
            pinMode(FEEDBACK_L_PIN, INPUT);
            digitalWrite(FEEDBACK_L_PIN, LOW);

            pinMode(FEEDBACK_R_PIN, INPUT);
            digitalWrite(FEEDBACK_R_PIN, LOW);
        }
    }

    if(int idx = cmd.indexOf("\"power\":"); idx != -1) {
        int pval = cmd.substring(idx + 8).toInt();
        analogWrite9(pval);
    } else {
        analogWrite9(0);
    }
}

// Analog write for pin 9
void analogWrite9(uint16_t duty) {
    OCR1A = constrain(duty, 0, FEEDBACK_PWM_MAX_VALUE);
}

void setup() {
    pinMode(WHEEL_PIN, INPUT);

    // Set PWM pins frequency:
    TCCR1A = 0;
    TCCR1B = 0;
    TCCR1A = (1 << COM1A1) | (1 << COM1B1) | (1 << WGM11);
    TCCR1B = (1 << WGM13) | (1 << WGM12) | (1 << CS10);

    ICR1 = FEEDBACK_PWM_MAX_VALUE; // 799 for 20 kHz (16MHz / (1 * (799 + 1)) )

    OCR1A = 0;
    OCR1B = 0;

    pinMode(FEEDBACK_L_PIN, INPUT);
    pinMode(FEEDBACK_R_PIN, INPUT);
    pinMode(FEEDBACK_PWM_PIN, OUTPUT);

    digitalWrite(FEEDBACK_L_PIN, LOW);
    digitalWrite(FEEDBACK_R_PIN, LOW);

    pinMode(TRANSMISSION_UP_PIN, INPUT_PULLUP);
    pinMode(TRANSMISSION_DOWN_PIN, INPUT_PULLUP);
    pinMode(HANDBRAKE_PIN, INPUT_PULLUP);

    pinMode(PEDAL_GAS_PIN, INPUT);
    pinMode(PEDAL_BRAKE_PIN, INPUT);
    pinMode(PEDAL_CLUTCH_PIN, INPUT);

    Serial.begin(115200);
}

void loop() {
    static unsigned long lastSend = 0;
    
    // Send state:
    if (millis() - lastSend > 10) { // sending frequency ~100 Hz
        sendState();
        lastSend = millis();
    }

    // Handle response:
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
            if (input.length() > 100) input = ""; // reset if response too long
        }
    }
}
