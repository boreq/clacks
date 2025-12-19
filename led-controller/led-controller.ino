#include <FastLED.h>

#define NUM_LEDS 96
#define DATA_PIN 8
#define LED_TYPE WS2812B
#define BRIGHTNESS 255
#define SATURATION 255

CRGB leds[NUM_LEDS];

void setup() {
  FastLED.addLeds<LED_TYPE, DATA_PIN>(leds, NUM_LEDS);
}

void loop() {
  for (int j = 0; j < 255; j++) {
    for (int i = 0; i < NUM_LEDS; i++) {
      leds[i] = CHSV(i - (j * 2), SATURATION, BRIGHTNESS); /* The higher the mult value the less fade there is and vice versa */ 
    }
    FastLED.show();
    delay(25);
  }
}
