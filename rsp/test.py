#!/usr/bin/env python3

import socket


xml = """

<target>
  <architecture>riscv</architecture>


<feature name="org.gnu.gdb.riscv.cpu">
  <reg name="zero" bitsize="32" type="int" regnum="0"/>
  <reg name="ra" bitsize="32" type="code_ptr"/>
  <reg name="sp" bitsize="32" type="data_ptr"/>
  <reg name="gp" bitsize="32" type="data_ptr"/>
  <reg name="tp" bitsize="32" type="data_ptr"/>
  <reg name="t0" bitsize="32" type="int"/>
  <reg name="t1" bitsize="32" type="int"/>
  <reg name="t2" bitsize="32" type="int"/>
  <reg name="fp" bitsize="32" type="data_ptr"/>
  <reg name="s1" bitsize="32" type="int"/>
  <reg name="a0" bitsize="32" type="int"/>
  <reg name="a1" bitsize="32" type="int"/>
  <reg name="a2" bitsize="32" type="int"/>
  <reg name="a3" bitsize="32" type="int"/>
  <reg name="a4" bitsize="32" type="int"/>
  <reg name="a5" bitsize="32" type="int"/>
  <reg name="a6" bitsize="32" type="int"/>
  <reg name="a7" bitsize="32" type="int"/>
  <reg name="s2" bitsize="32" type="int"/>
  <reg name="s3" bitsize="32" type="int"/>
  <reg name="s4" bitsize="32" type="int"/>
  <reg name="s5" bitsize="32" type="int"/>
  <reg name="s6" bitsize="32" type="int"/>
  <reg name="s7" bitsize="32" type="int"/>
  <reg name="s8" bitsize="32" type="int"/>
  <reg name="s9" bitsize="32" type="int"/>
  <reg name="s10" bitsize="32" type="int"/>
  <reg name="s11" bitsize="32" type="int"/>
  <reg name="t3" bitsize="32" type="int"/>
  <reg name="t4" bitsize="32" type="int"/>
  <reg name="t5" bitsize="32" type="int"/>
  <reg name="t6" bitsize="32" type="int"/>
  <reg name="pc" bitsize="32" type="code_ptr"/>
</feature>
</target>
"""


def create_response(text):
    sum = 0
    for ch in text:
        sum += ord(ch)

    result = "$%s#%s" % (text, hex(sum % 256)[2:])
    return result.encode("utf8")

def handle_connection(conn):

    addr = 0x24

    while True:
        data = conn.recv(1024)
        if not data:
            break
        print(b"Received: " + data)
        # conn.send(b"+")
        response = b"$#00"
        if b"+" == data:
            continue

        conn.send(b"+")
        if b"$?" in data:
            response = create_response("S05")

        if b"qSupported" in data:
            response = create_response("PacketSize=120")

        if b"$g#" in data:
            response = create_response("11111111" * 32 + "{:2x}000000".format(addr))

        if b"read:target.xml" in data:
            response = create_response(xml)

        if b"vCont?" in data:
            response = create_response("")

        if b"$s#" in data:
            addr += 4
            response = create_response("S05")

        print(b"Sent: " + response)
        conn.send(response)



HOST = '127.0.0.1'  # Standard loopback interface address (localhost)
PORT = 3000        # Port to listen on (non-privileged ports are > 1023)

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen()
    conn, addr = s.accept()
    with conn:
        print('Connected by', addr)
        handle_connection(conn)