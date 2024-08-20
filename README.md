
=== Example
CONNECT
client >> "Name[myname]\r\n" >> server
server >> "Hello[servername]\r\n" >> client
client >> "Message[message]" >> server
server >> "Message[$message]" >> client
client >> "Bye" CLOSE


