import socket
import time
import sys
import argparse
import json
       
if __name__ == "__main__":
    try:
        parser = argparse.ArgumentParser()
        parser.add_argument('-i', '--ip', default='192.168.0.250', help='server IP')
        parser.add_argument('-t', '--type', default='start', help='type of command')
        parser.add_argument('-f', '--frequency', type=int, default=2000, help='step frequency')
        parser.add_argument('-d', '--direction', type=int, default=0, help='step direction')
        args = parser.parse_args()

        dict_data = dict(vars(args))
        dict_data.pop('ip')

        msg = json.dumps(dict_data)
        print(msg)

        addr = args.ip
        port = 2020

        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        dest = (addr, port)

        s.connect(dest)
        s.settimeout(0.5)
        
        s.sendall(msg.encode('utf-8'))

        s.close()
    except Exception as ex:
        print(ex)