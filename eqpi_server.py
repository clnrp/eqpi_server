import socket
import time
import sys
import time
import json
import RPi.GPIO as GPIO
from threading import Thread

RA_DIR = 13
RA_STEP = 19
RA_EN = 12
RA_M1 = 16
RA_M2 = 17
RA_M3 = 20

DEC_DIR = 24
DEC_STEP = 18
DEC_EN = 4
DEC_M1 = 21
DEC_M2 = 22
DEC_M3 = 27

class StepperMotor:
    def __init__(self, pinout, dutycycle=50, frequency=2000):
        self.pinout = pinout
        self.dutycycle = dutycycle
        self.frequency = frequency
        self.direction = 0
        self.working = 0
        self.pwm = GPIO.PWM(self.pinout['pwm'], frequency)

    def start(self, frequency = 2000, direction = 1):
        self.frequency = frequency
        self.direction = direction
        self.pwm.start(self.dutycycle)
        self.setFrequency(self.frequency)
        self.setDirection(self.direction)
        self.working = 1
        GPIO.output(self.pinout['enable'], GPIO.HIGH)
    
    def stop(self):
        GPIO.output(self.pinout['enable'], GPIO.LOW)
        self.pwm.stop()
        self.working = 0

    def setDirection(self, value):
        self.direction = value
        if self.direction == 1:
            GPIO.output(self.pinout['direction'], GPIO.HIGH)
        else:
            GPIO.output(self.pinout['direction'], GPIO.LOW)

    def setFrequency(self, frequency):
        self.frequency = frequency
        self.pwm.ChangeFrequency(self.frequency)

    def getFrequency(self):
        return self.frequency

    def getState(self):
        return {"working": self.working, "frequency": self.frequency, "direction": self.direction}

class EqPIServer(Thread):

    def __init__(self, conn, RA, DEC):
        Thread.__init__(self)
        self.name = "EqPIServer"
        self.conn = conn
        self.conn.settimeout(0.1)
        self.RA = RA
        self.DEC = DEC

    def run(self):
        time_ack = time.time()
        thread = True
        bdata = ''
        data = ''

        while thread:
            try:
                time.sleep(0.1)
 
                try:
                    bdata = self.conn.recv(1024)
                except socket.timeout:
                    pass

                if len(bdata) > 0:
                    print(bdata)
                    data = json.loads(bdata)
                    bdata = ''
                    if data['type'] == 'start':
                        self.RA.start(data['frequency'], data['direction'])
                    elif data['type'] == 'stop':
                        self.RA.stop()
                    elif data['type'] == 'ra_direction':
                        self.RA.setDirection(data['direction'])
                    elif data['type'] == 'steps_per_second':
                        self.RA.setFrequency(data['frequency'])

                if abs(time.time()-time_ack) >= 2:
                    time_ack = time.time()
                    msg = json.dumps(self.RA.getState())+"\r\n"
                    self.conn.sendall(msg.encode('utf-8'))

            except BrokenPipeError:
                thread = False
            except Exception as ex:
                print(str(ex))
        self.conn.close()


if __name__ == "__main__":
    args = sys.argv[1:]
    port = 2020

    GPIO.setmode(GPIO.BCM)

    GPIO.setup(RA_DIR, GPIO.OUT)
    GPIO.setup(RA_STEP, GPIO.OUT)
    GPIO.setup(RA_EN, GPIO.OUT)
    GPIO.setup(RA_M1, GPIO.OUT)
    GPIO.setup(RA_M2, GPIO.OUT)
    GPIO.setup(RA_M3, GPIO.OUT)
    GPIO.output(RA_DIR, GPIO.LOW)
    GPIO.output(RA_STEP, GPIO.LOW)
    GPIO.output(RA_EN, GPIO.LOW)
    GPIO.output(RA_M1, GPIO.LOW)
    GPIO.output(RA_M2, GPIO.LOW)
    GPIO.output(RA_M3, GPIO.LOW)

    GPIO.setup(DEC_DIR, GPIO.OUT)
    GPIO.setup(DEC_STEP, GPIO.OUT)
    GPIO.setup(DEC_EN, GPIO.OUT)
    GPIO.setup(DEC_M1, GPIO.OUT)
    GPIO.setup(DEC_M2, GPIO.OUT)
    GPIO.setup(DEC_M3, GPIO.OUT)
    GPIO.output(DEC_DIR, GPIO.LOW)
    GPIO.output(DEC_STEP, GPIO.LOW)
    GPIO.output(DEC_EN, GPIO.LOW)
    GPIO.output(DEC_M1, GPIO.LOW)
    GPIO.output(DEC_M2, GPIO.LOW)
    GPIO.output(DEC_M3, GPIO.LOW)

    RA = StepperMotor({'pwm':RA_STEP,'direction':RA_DIR,'enable':RA_EN})
    DEC = StepperMotor({'pwm':DEC_STEP,'direction':DEC_DIR,'enable':DEC_EN})

    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            s.bind(('', port))
        except socket.error as ex:
            print(str(ex))
            exit(0)
        s.listen(10)

        while True:
            try:
                time.sleep(1)
                conn, addr = s.accept()
                eqPIServer = EqPIServer(conn, RA, DEC)
                eqPIServer.start()
                print(str(addr))
            except Exception as ex:
                print(str(ex))

    except Exception as ex:
        print(str(ex))
    finally:
        s.close()