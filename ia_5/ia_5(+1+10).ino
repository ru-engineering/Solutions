
#include<Wire.h> 

unsigned int x;

void setup()
{
  Wire.begin(104);                // join i2c bus with address #4
  Wire.onReceive(receiveEvent); // register event
  Wire.onRequest(requestEvent);
  Serial.begin(115200);           // start serial for output
}

void loop()
{ 
  delay(100);
}

// function that executes whenever data is received from master
// this function is registered as an event, see setup()
void receiveEvent(int howMany)
{
  x = Wire.read();    // receive byte as an integer
  x = x+1;
  Serial.print("Arduino gets: ");
  Serial.print(x-1);
  Serial.print('\n');
}

void requestEvent()
{
  Wire.write(x);
  Serial.print("Arduino sends: "); 
  Serial.print(x); 
}
