This is a simple take on a chat application built in pure rust.This was an assignment we had to do for school.

This is a basic and simple app,not anything special but it does have a cool name.

The code has two parts:

	1. The new code:
		Using rocket framework to abstract all the code.
	2. The old code:
		a. Server code - This will open a socket and listen to the client messages,This should also store the messages in a Vec.
		b. Client code - This should run and allow the client to send messages limited to 1000 bytes to the server.
Future of the project:
1. Allow another client to join the server and send messages back and forth between the server and other clients.
2. A UI built in maybe Slint or any framework that I may deem fit.
3. Host the code on AWS or something,preferrably on an EC2 instance.

This is how the code looks:
![screenshot-127 ![Screenshot from 2024-04-08 23-13-55](https://github.com/eiidoubleyuwes/Oxiron/assets/148796574/2102306a-8e26-4b57-921a-ba990dae9742)

0 0 1_8000-2024 04 08-23_11_21](https://github.com/eiidoubleyuwes/Oxiron/assets/148796574/8847e0d2-16c5-481e-9a7c-ce7d5706361f)





