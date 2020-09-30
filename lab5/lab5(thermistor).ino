
#define thermistor_current_pin A3
#define thermistor_analog_read_pin A2

void setup() {

    // Initialize serial port as we will be printing temperature data to it.
    Serial.begin(115200);
    
    // Setup how our pins are used
    pinMode(thermistor_current_pin, OUTPUT);
    pinMode(thermistor_analog_read_pin, INPUT);
} // End setup

void loop() { 
  
    float celcius_thermistor_temperature_0 = 0;
    for (int i = 0; i <= 9; i++){
    charge_input_capacitor();
    
    int raw_thermistor_reading = read_thermistor_data();
    // Serial.println(raw_thermistor_reading);
    
    double unfiltered_resistance = 
        calculate_resistance_from_analog_read(raw_thermistor_reading);
    // Serial.println(unfiltered_resistance);
    
    double filtered_resistance = filter_resistance_reading(unfiltered_resistance);
    // Serial.println(filtered_resistance);
    
    float kelvin_thermistor_temperature = calculate_temperature_from_resistance(filtered_resistance);
    float celcius_thermistor_temperature = kelvin_thermistor_temperature - 273.15;
    float fahrenheit_thermistor_temperature = ((celcius_thermistor_temperature*9)/5.0) + 32.0;
    
    
    
      celcius_thermistor_temperature_0 = celcius_thermistor_temperature_0 + celcius_thermistor_temperature;
      delay(1);
      }
      Serial.println(celcius_thermistor_temperature_0/10);
    // This delay here is a temporary placeholder
} // End main loop


// This function charges up the ADC input capacitor
// and does not continue until it is charged
void charge_input_capacitor(){
  
      // Start by charging up the capacitor on the input pin
    digitalWrite(thermistor_current_pin, HIGH);
    
    // Wait 100 milliseconds for the input capacitor to fully charge.
    // Currently delay() is used as a placeholder.
    // For most applications we will want to use a non-blocking timer function.
    delay(1);
} // End charge_input_capacitor function


// This function records and returns an analog reading.
// It also turns off the current pin once complete
int read_thermistor_data(){
    
    // Read analog data from charged capacitor.
    int raw_thermistor_reading = analogRead(thermistor_analog_read_pin);
  
    // Turn off the thermistor current pin to minimize self-heating of temperature sensor
    digitalWrite(thermistor_current_pin, LOW);
    
    return raw_thermistor_reading;
} // End read_thermistor_data function


/** 
 This function calculates the rough resistance of the thermistor but does not filter the results for more accuracy. 
 That is handled in another function.
 For the math here, the full scale range of the analog read is 0 to 1023, (1024 steps) because the arduino nano has a 10bit ADC.
 2^10 = 1024
 raw_thermistor_reading / (1023.0 - raw_thermistor_reading) calculates the proportion of the voltage across the thermistor in the voltage divider comparated to the voltage acrross the constant resistor in the voltage divider.
 Once the proportion of that voltage is known, we can calulate the resistance of the thermistor by multiplying that proportion by the resitance of the constant resistor in the voltage divider.
**/ 
double calculate_resistance_from_analog_read(int raw_thermistor_reading){
  
    // The resistance of the 10 kΩ resistor in the voltage divider is included here as a local variable.
    // If you have a more precise reading of the resistor you can change it here for more accuracy 
    double voltage_divider_resistor = 10000.0;
    
    // If there is a full scale reading (1023) there is an open circuit, and we end the function early and simply return 999999 to avoid dividing by 0
    if(raw_thermistor_reading >= 1023){
      return 999999.9;
    }
  
    // See function comment for more explanation of the math
    double unfiltered_resistance = voltage_divider_resistor * (
        raw_thermistor_reading / (1023.0 - raw_thermistor_reading)
                                                                 );                                                           
    return unfiltered_resistance;
} // End calculate_resistance_from_analog_read function


/**
 This function filters the resistance reading. Filtering gives better results because no measurement system is perfect.
 In this case, measuring the voltage of the thermistor absorbs some of it's current during the read process, and slightly alters the true voltage of the thermistor. 
 This function compensates that and returns resistance readings much closer to their true value.
 **/
double filter_resistance_reading(double unfiltered_resistance){

    // These compensation values are specific to the ADC of the Arduino Nano or Uno, to the resistance of the voltage divider, capacitance at the input, and wait time between measurements. 
    // If any of those parameters change, the values will likely have to be adjusted.
    double calibration_factor = -3.27487396E-07 * unfiltered_resistance +
        8.25744292E-03;
  
    double filtered_resistance = unfiltered_resistance * (1+ calibration_factor);
  
    return filtered_resistance;
} // end filter_resistance_reading function


/**
  This function uses the 4 term Steinhart-Hart equation to determine the temperature of a thermistor from its resistance.
  Go to https://www.northstarsensors.com/calculating-temperature-from-resistance
  for more information about the Steinhart-Hart equation
**/
float calculate_temperature_from_resistance(double thermistor_resistance){
    // These constant values are for a North Star Sensors, curve 44, 10 kΩ at 25 °C thermistor.
    // They are generated from 4 data points at 0, 25, 50, and 70 °C.
    // If you are measuring outside that range, use constants specialized with data points in the range you need.
    double SH_A_constant = 1.21500454194620E-03;
    double SH_B_constant = 2.05334949463842E-04;
    double SH_C_constant = 3.19176316497180E-06;
    double SH_D_constant = -2.93752010251114E-08;
  
    // In arduino log() calculates the natural logarithm sometimes written as ln, not log base 10
    double natural_log_of_resistance = log(thermistor_resistance);
  
    // pow() is a function which rases a number to its power.
    // For example x to the power of 2, x^2, is pow(x, 2)
    float thermistor_temperature_kelvin = 1 / ( SH_A_constant +
                                                SH_B_constant * natural_log_of_resistance +
                                                SH_C_constant * pow(natural_log_of_resistance, 2) +
                                                SH_D_constant * pow(natural_log_of_resistance, 3)
                                              );
    return thermistor_temperature_kelvin;
} // end calculate_temperature_from_resistance function
