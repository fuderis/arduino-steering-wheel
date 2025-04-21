const int TRANSMISSION_UP_BUTTON = 10;
const int TRANSMISSION_DOWN_BUTTON = 11;
const int HANDBRAKE_BUTTON = 12;

volatile bool transmission_up_pressed = false;
volatile bool transmission_down_pressed = false;
volatile bool handbrake_pressed = false;

// The transmission & handbrake handler
void transmission_handbrake_handler() {
    // transmission up:
    if (digitalRead(TRANSMISSION_UP_PIN) == LOW) {
        if (!transmission_up_pressed) {
            transmission_up_pressed = true;

            Gamepad.press(TRANSMISSION_UP_BUTTON);
            Gamepad.write();

            Serial.println("Transmission up pressed");
        }
    } else if (transmission_up_pressed) {
        transmission_up_pressed = false;

        Gamepad.release(TRANSMISSION_UP_BUTTON);
        Gamepad.write();

        Serial.println("Transmission up released");
    }

    // transmission down:
    if (digitalRead(TRANSMISSION_DOWN_PIN) == LOW) {
        if (!transmission_down_pressed) {
            transmission_down_pressed = true;

            Gamepad.press(TRANSMISSION_DOWN_BUTTON);
            Gamepad.write();

            Serial.println("Transmission down pressed");
        }
    } else if (transmission_down_pressed) {
        transmission_down_pressed = false;

        Gamepad.release(TRANSMISSION_DOWN_BUTTON);
        Gamepad.write();

        Serial.println("Transmission down released");
    }

    // handbrake:
    if (digitalRead(HANDBRAKE_PIN) == LOW) {
        if (!handbrake_pressed) {
            handbrake_pressed = true;

            Gamepad.press(HANDBRAKE_BUTTON);
            Gamepad.write();
            
            Serial.println("Handbrake pressed");
        }
    } else if (handbrake_pressed) {
        handbrake_pressed = false;

        Gamepad.release(HANDBRAKE_BUTTON);
        Gamepad.write();

        Serial.println("Handbrake released");
    }
}
