# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman 

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. In the observer pattern diagram explained by the Head First Design Pattern book and also my understanding of Observer design patterns, on BambangShop case a single model struct is enough. This struct model is for Subscriber. This is so that the main app will retain lists of Subscribers that follow a product type. This is because we are doing the push model which is specifically to execute notify() method in the main app to make the HTTP requests to be send to the /receive URL of the receiver app to every Subscriber that subscribed to the relevant product type when notification data must be send based on the three different scenarios of there is a new Product with the same product type, or a product with the same product type is delete dfrom the store or BambangShop promotes a product with the same product type. 
2. Yes. For examplpe using DashMap for both subscriber and product because DashMap itself can store key and value pairs then it make sense to store each product and subscriber object value that has keys of their id and url. This way when using or trying to check if a product or subscriber exist in the repository it is easier to just call the key and check if the value exist when getting it.
3. Yes you can implement Singleton pattern in this case but by using DashMap instead, because DashMap abstracts the complexity when managing concurrency, provide better performance, and reduces maintenance overhead making it more robust for managing mutable state in multithreaded Rust applications. By using Singleton pattern, it is not inherently suited to manage thread safety for multithreaded unless using Mutex or RwLock but these could be implemented correctly but stilll can be error-prone and could introduce performance overhead due to locking.  
#### Reflection Publisher-2
1. The reason that service and repository are seperated from a model is because a model should just be responsible on encapsulating data or if needed implement some basic rules. For business logic and data access features it would violate a principle from SOLID which is Single Responsibility Principle. Violating this principle would create a lesser modularity. So the service layer can contain the business logic part to manipulate the data and repository layer focuses on data access.
2. The big circumstance when only usingi model for program, subscriber, notification is that everything that was seperated for the repository for data access and service for business logic implementation would be merged and this would create one three large files in the model directory for each program, subscriber, notification. This means that all the functions and fields would be specified in the model directory.
3. Yes Postman is very helpful on testing the endpoints of my project. I am very interested on the Postman feature that would allow to test POST methods such as POST and GET. All I need to do is specify the endpoints and the necessary body for posting for example if I want to use POST and it would then return to me the response so that I can check if it is working as intended from the implementation or not. 
#### Reflection Publisher-3
1. The observer pattern variation that is being used here is the Push model because the publisher is pushing the notification to subscribers when the publisher want to notify the subscriber everytime a product is created, promotion, and deletion of the products. 
2. The advantages of using pull model observer pattern variation for this tutorial case is that the subscribers could pull data from the publisher but the disadvantage is that the publisher would not be able to push data to subscribers or notify them as what is originally intended in this tutorial case.
3. By not using multithreading for notification, then the publisher must wait for each subscriber to access the notification before it can send the notification to other subscribers. Especially if the subscriber that already received the notification is trying to modify the shared resources but other publisher is also accessing this resources then there could be problems such as race conditions, data corruption, and other inconsistent problems. This would slow down the performance of the project significantly and also make scalability hard to do in the future. 