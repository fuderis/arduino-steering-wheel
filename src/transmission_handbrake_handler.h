// The transmission & handbrake handler
void transmission_handbrake_handler() {
    // transmission up:
    if (digitalRead(TRANSMISSION_UP_PIN) == LOW) {
        if (!transmission_up_pressed) {
            transmission_up_pressed = true;

            Gamepad.press(1);
            Gamepad.write();

            Serial.println("Transmission up pressed");
        }
    } else if (transmission_up_pressed) {
        transmission_up_pressed = false;

        Gamepad.release(1);
        Gamepad.write();

        Serial.println("Transmission up released");
    }

    // transmission down:
    if (digitalRead(TRANSMISSION_DOWN_PIN) == LOW) {
        if (!transmission_down_pressed) {
            transmission_down_pressed = true;

            Gamepad.press(2);
            Gamepad.write();

            Serial.println("Transmission down pressed");
        }
    } else if (transmission_down_pressed) {
        transmission_down_pressed = false;

        Gamepad.release(2);
        Gamepad.write();

        Serial.println("Transmission down released");
    }

    // handbrake:
    if (digitalRead(HANDBRAKE_PIN) == LOW) {
        if (!handbrake_pressed) {
            handbrake_pressed = true;

            Gamepad.press(3);
            Gamepad.write();
            
            Serial.println("Handbrake pressed");
        }
    } else if (handbrake_pressed) {
        handbrake_pressed = false;

        Gamepad.release(3);
        Gamepad.write();

        Serial.println("Handbrake released");
    }
}
