This is a simple take on a chat application built in pure rust.This was an assignment we had to do for school.

This is a basic and simple app,not anything special but it does have a cool name.

The code has two parts:
  1. Server code - This will open a socket and listen to the client messages,This should also store the messages in a Vec.
  2. Client code - This should run and allow the client to send messages limited to 1000 bytes to the server.

Future of the project:
  1. Allow another client to join the server and send messages back and forth between the server and other clients.
  2. A UI built in maybe Slint or any framework that I may deem fit.
  3. Host the code on AWS or something,preferrably on an EC2 instance.
